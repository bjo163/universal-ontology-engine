use ontology_core::{OntologyType, SourceSpan};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Language { TypeScript, JavaScript, Python, Go, Java, Kotlin }

impl Language {
    pub fn parse_name(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "typescript" | "ts" | "tsx" => Some(Self::TypeScript),
            "javascript" | "js" | "jsx" => Some(Self::JavaScript),
            "python" | "py" => Some(Self::Python),
            "go" | "golang" => Some(Self::Go),
            "java" => Some(Self::Java),
            "kotlin" | "kt" => Some(Self::Kotlin),
            _ => None,
        }
    }
    pub const fn slug(self) -> &'static str {
        match self { Self::TypeScript => "typescript", Self::JavaScript => "javascript", Self::Python => "python", Self::Go => "go", Self::Java => "java", Self::Kotlin => "kotlin" }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LanguageObservation {
    pub language: Language,
    pub ontology_type: OntologyType,
    pub native_kind: String,
    pub name: Option<String>,
    pub span: SourceSpan,
    pub evidence: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LanguageError {
    #[error("unsupported language: {0}")]
    UnsupportedLanguage(String),
}

pub fn parse(language: Language, source: &str) -> Result<Vec<LanguageObservation>, LanguageError> {
    let mut out = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let line_no = (index + 1) as u32;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") || trimmed.starts_with('*') { continue; }
        let rules = match language {
            Language::TypeScript | Language::JavaScript => js_rules(trimmed),
            Language::Python => python_rules(trimmed),
            Language::Go => go_rules(trimmed),
            Language::Java | Language::Kotlin => jvm_rules(trimmed, language),
        };
        for (ty, kind, name, evidence) in rules {
            let column = line.find(trimmed).unwrap_or(0) as u32;
            out.push(LanguageObservation {
                language,
                ontology_type: ty,
                native_kind: kind.to_owned(),
                name,
                span: SourceSpan { line_start: line_no, column_start: column + 1, line_end: line_no, column_end: line.len() as u32 },
                evidence: evidence.to_owned(),
            });
        }
    }
    Ok(out)
}

fn js_rules(line: &str) -> Vec<(OntologyType, &'static str, Option<String>, &'static str)> {
    let mut out = Vec::new();
    if let Some(n) = after_keyword(line, "class") { out.push((OntologyType::Entity, "class", Some(n), "class declaration")); }
    if let Some(n) = after_keyword(line, "interface") { out.push((OntologyType::Entity, "interface", Some(n), "interface declaration")); }
    if let Some(n) = after_keyword(line, "type") { out.push((OntologyType::Entity, "type_alias", Some(n), "type declaration")); }
    if let Some(n) = after_keyword(line, "function") { out.push((OntologyType::Function, "function", Some(n), "function declaration")); }
    if let Some(n) = assignment_function(line) { out.push((OntologyType::Function, "arrow_function", Some(n), "arrow function assignment")); }
    if line.starts_with("import ") || line.starts_with("export ") { out.push((OntologyType::Instruction, "module_statement", None, "module statement")); }
    if let Some(n) = variable_name(line) { out.push((OntologyType::Value, "variable", Some(n), "variable declaration")); }
    out
}

fn python_rules(line: &str) -> Vec<(OntologyType, &'static str, Option<String>, &'static str)> {
    let mut out = Vec::new();
    if let Some(n) = after_keyword(line, "class") { out.push((OntologyType::Entity, "class", Some(n), "class declaration")); }
    if let Some(n) = after_keyword(line, "def") { out.push((OntologyType::Function, "function", Some(n), "function declaration")); }
    if let Some(n) = after_keyword(line, "async def") { out.push((OntologyType::Function, "async_function", Some(n), "async function declaration")); }
    if line.starts_with("import ") || line.starts_with("from ") { out.push((OntologyType::Instruction, "import", None, "import statement")); }
    if let Some(n) = assignment_name(line) { out.push((OntologyType::Value, "assignment", Some(n), "assignment")); }
    out
}

fn go_rules(line: &str) -> Vec<(OntologyType, &'static str, Option<String>, &'static str)> {
    let mut out = Vec::new();
    if let Some(n) = after_keyword(line, "type") { out.push((OntologyType::Entity, "type", Some(n), "type declaration")); }
    if let Some(n) = after_keyword(line, "func") { out.push((OntologyType::Function, "function", Some(n), "function declaration")); }
    if line.starts_with("import ") || line.starts_with("const ") || line.starts_with("var ") { out.push((OntologyType::Instruction, "declaration", None, "package/import/declaration statement")); }
    out
}

fn jvm_rules(line: &str, language: Language) -> Vec<(OntologyType, &'static str, Option<String>, &'static str)> {
    let mut out = Vec::new();
    for keyword in ["class", "interface", "object", "enum class"] {
        if let Some(n) = after_keyword(line, keyword) { out.push((OntologyType::Entity, keyword, Some(n), "type declaration")); }
    }
    if let Some(n) = after_keyword(line, if language == Language::Kotlin { "fun" } else { "void" }) {
        out.push((OntologyType::Function, "function", Some(n), "function declaration"));
    }
    if line.starts_with("package ") || line.starts_with("import ") { out.push((OntologyType::Instruction, "package_or_import", None, "package/import statement")); }
    if let Some(n) = assignment_name(line) { out.push((OntologyType::Value, "property", Some(n), "property/field declaration")); }
    out
}

fn after_keyword(line: &str, keyword: &str) -> Option<String> {
    let rest = line.strip_prefix(keyword)?.trim_start();
    let name = rest.split(|c: char| c == '(' || c == '{' || c == ':' || c == '<' || c.is_whitespace()).next()?;
    (!name.is_empty()).then(|| name.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '$').to_owned())
}

fn assignment_function(line: &str) -> Option<String> {
    let (left, right) = line.split_once('=')?;
    if !right.contains("=>") { return None; }
    let name = left.split_whitespace().last()?.trim_matches(';');
    (!name.is_empty()).then(|| name.to_owned())
}

fn variable_name(line: &str) -> Option<String> {
    for keyword in ["const", "let", "var"] {
        if let Some(rest) = line.strip_prefix(keyword) {
            return rest.trim_start().split(|c: char| c == '=' || c == ':' || c.is_whitespace()).next().map(str::to_owned);
        }
    }
    None
}

fn assignment_name(line: &str) -> Option<String> {
    let (left, _) = line.split_once('=')?;
    let name = left.split_whitespace().last()?.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
    (!name.is_empty()).then(|| name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn javascript_adapter_finds_core_declarations() {
        let items = parse(Language::JavaScript, "class User {}\nfunction load() {}\nconst make = () => 1;\nimport x from 'x';").unwrap();
        assert!(items.iter().any(|x| x.native_kind == "class" && x.name.as_deref() == Some("User")));
        assert!(items.iter().any(|x| x.ontology_type == OntologyType::Function));
        assert!(items.iter().any(|x| x.ontology_type == OntologyType::Instruction));
    }

    #[test]
    fn python_adapter_finds_definitions() {
        let items = parse(Language::Python, "class User:\n    def load(self):\n        return 1\n").unwrap();
        assert_eq!(items.iter().filter(|x| x.ontology_type == OntologyType::Entity).count(), 1);
        assert!(items.iter().any(|x| x.ontology_type == OntologyType::Function && x.name.as_deref() == Some("load")));
    }

    #[test]
    fn all_phase_six_languages_have_an_adapter() {
        assert!(!parse(Language::TypeScript, "class Demo {}\n").unwrap().is_empty());
        assert!(!parse(Language::JavaScript, "class Demo {}\n").unwrap().is_empty());
        assert!(!parse(Language::Python, "class Demo:\n").unwrap().is_empty());
        assert!(!parse(Language::Go, "type Demo struct{}\n").unwrap().is_empty());
        assert!(!parse(Language::Java, "class Demo {}\n").unwrap().is_empty());
        assert!(!parse(Language::Kotlin, "class Demo\n").unwrap().is_empty());
    }
}
