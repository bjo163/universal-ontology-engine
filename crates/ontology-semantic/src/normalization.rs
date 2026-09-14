use ontology_core::OntologyType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NormalizationClass {
    DeclaredType,
    CallableDeclaration,
    CallableBinding,
    MethodDeclaration,
    NamedValueBinding,
    PropertyBinding,
    ModuleBoundaryStatement,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NativeEvidenceDescriptor {
    pub language: String,
    pub native_kind: String,
    pub source_type: OntologyType,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NormalizationRule {
    pub rule_id: String,
    pub rule_version: String,
    pub language: String,
    pub native_kind: String,
    pub source_type: OntologyType,
    pub class: NormalizationClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NormalizedEvidence {
    pub rule_id: String,
    pub rule_version: String,
    pub original: NativeEvidenceDescriptor,
    pub class: NormalizationClass,
}

impl NormalizationRule {
    pub fn matches(&self, evidence: &NativeEvidenceDescriptor) -> bool {
        self.language == evidence.language
            && self.native_kind == evidence.native_kind
            && self.source_type == evidence.source_type
    }

    pub fn normalize(&self, evidence: &NativeEvidenceDescriptor) -> Option<NormalizedEvidence> {
        self.matches(evidence).then(|| NormalizedEvidence {
            rule_id: self.rule_id.clone(),
            rule_version: self.rule_version.clone(),
            original: evidence.clone(),
            class: self.class,
        })
    }
}

pub fn normalize_candidates(
    evidence: &NativeEvidenceDescriptor,
    rules: impl IntoIterator<Item = NormalizationRule>,
) -> Vec<NormalizedEvidence> {
    let mut out: Vec<_> = rules.into_iter().filter_map(|rule| rule.normalize(evidence)).collect();
    out.sort();
    out.dedup();
    out
}

pub fn foundation_rules() -> Vec<NormalizationRule> {
    use NormalizationClass::*;
    use OntologyType::*;
    let mut rules = vec![
        rule("ts.class", "typescript", "class", Entity, DeclaredType),
        rule("ts.interface", "typescript", "interface", Entity, DeclaredType),
        rule("ts.type", "typescript", "type_alias", Entity, DeclaredType),
        rule("js.class", "javascript", "class", Entity, DeclaredType),
        rule("py.class", "python", "class", Entity, DeclaredType),
        rule("go.type", "go", "type", Entity, DeclaredType),
        rule("java.class", "java", "class", Entity, DeclaredType),
        rule("java.interface", "java", "interface", Entity, DeclaredType),
        rule("kt.class", "kotlin", "class", Entity, DeclaredType),
        rule("kt.interface", "kotlin", "interface", Entity, DeclaredType),
        rule("ts.function", "typescript", "function", Function, CallableDeclaration),
        rule("js.function", "javascript", "function", Function, CallableDeclaration),
        rule("py.function", "python", "function", Function, CallableDeclaration),
        rule("py.async", "python", "async_function", Function, CallableDeclaration),
        rule("go.function", "go", "function", Function, CallableDeclaration),
        rule("java.function", "java", "function", Function, CallableDeclaration),
        rule("kt.function", "kotlin", "function", Function, CallableDeclaration),
        rule("ts.arrow", "typescript", "arrow_function", Function, CallableBinding),
        rule("js.arrow", "javascript", "arrow_function", Function, CallableBinding),
        rule("java.method", "java", "method", Function, MethodDeclaration),
        rule("ts.variable", "typescript", "variable", Value, NamedValueBinding),
        rule("js.variable", "javascript", "variable", Value, NamedValueBinding),
        rule("py.assignment", "python", "assignment", Value, NamedValueBinding),
        rule("java.property", "java", "property", Value, PropertyBinding),
        rule("kt.property", "kotlin", "property", Value, PropertyBinding),
        rule("ts.module", "typescript", "module_statement", Instruction, ModuleBoundaryStatement),
        rule("js.module", "javascript", "module_statement", Instruction, ModuleBoundaryStatement),
        rule("py.import", "python", "import", Instruction, ModuleBoundaryStatement),
        rule("java.module", "java", "package_or_import", Instruction, ModuleBoundaryStatement),
        rule("kt.module", "kotlin", "package_or_import", Instruction, ModuleBoundaryStatement),
    ];
    rules.sort();
    rules
}

fn rule(id: &str, language: &str, native_kind: &str, source_type: OntologyType, class: NormalizationClass) -> NormalizationRule {
    NormalizationRule {
        rule_id: format!("norm.{id}"),
        rule_version: "1".into(),
        language: language.into(),
        native_kind: native_kind.into(),
        source_type,
        class,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_language_class_normalization_preserves_native_evidence() {
        let ts = NativeEvidenceDescriptor { language: "typescript".into(), native_kind: "class".into(), source_type: OntologyType::Entity };
        let py = NativeEvidenceDescriptor { language: "python".into(), native_kind: "class".into(), source_type: OntologyType::Entity };
        let ts_out = normalize_candidates(&ts, foundation_rules());
        let py_out = normalize_candidates(&py, foundation_rules());
        assert_eq!(ts_out[0].class, py_out[0].class);
        assert_ne!(ts_out[0].original.language, py_out[0].original.language);
    }

    #[test]
    fn ambiguous_go_declaration_is_not_normalized_as_module_boundary() {
        let evidence = NativeEvidenceDescriptor { language: "go".into(), native_kind: "declaration".into(), source_type: OntologyType::Instruction };
        assert!(normalize_candidates(&evidence, foundation_rules()).is_empty());
    }

    #[test]
    fn rule_order_does_not_change_normalization_output() {
        let evidence = NativeEvidenceDescriptor { language: "typescript".into(), native_kind: "class".into(), source_type: OntologyType::Entity };
        let mut reversed = foundation_rules();
        reversed.reverse();
        assert_eq!(normalize_candidates(&evidence, foundation_rules()), normalize_candidates(&evidence, reversed));
    }
}
