use ontology_core::{EdgeKind, Node, NodeId, OntologyType, SourceSpan};
use ontology_discovery::{DiscoveryObservation, DiscoveryResult};
use ontology_language::{parse as parse_language, Language};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct ProjectedSyntax {
    ontology_type: OntologyType,
    native_kind: String,
    name: Option<String>,
    span: SourceSpan,
    evidence: String,
    language: String,
}

pub fn project_workspace_syntax(
    result: &mut DiscoveryResult,
    workspace: &Path,
) -> Result<(), String> {
    let element_paths = result
        .observations
        .iter()
        .filter(|observation| observation.kind == "element")
        .map(|observation| observation.path.clone())
        .collect::<Vec<_>>();

    for relative_path in element_paths {
        let path = workspace.join(&relative_path);
        let Some(adapter) = syntax_adapter_for_path(&path) else {
            continue;
        };
        let element = resolve_element(result, &relative_path).ok_or_else(|| {
            format!("could not resolve ELEMENT for syntax source `{relative_path}`")
        })?;
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;

        let projected = match adapter {
            SyntaxAdapter::Rust => match ontology_rust::parse(&source) {
                Ok(items) => items
                    .into_iter()
                    .map(|item| ProjectedSyntax {
                        ontology_type: item.ontology_type,
                        native_kind: item.native_kind,
                        name: item.name,
                        span: SourceSpan {
                            line_start: item.location.line_start as u32,
                            column_start: item.location.column_start as u32,
                            line_end: item.location.line_end as u32,
                            column_end: item.location.column_end as u32,
                        },
                        evidence: "syn AST".into(),
                        language: "rust".into(),
                    })
                    .collect::<Vec<_>>(),
                Err(error) => {
                    result.observations.push(DiscoveryObservation {
                        path: relative_path.clone(),
                        kind: "syntax-error".into(),
                        language: Some("rust".into()),
                        evidence: format!("parse error preserved as observation: {error}"),
                    });
                    Vec::new()
                }
            },
            SyntaxAdapter::Language(language) => parse_language(language, &source)
                .expect("selected syntax adapter is supported")
                .into_iter()
                .map(|item| ProjectedSyntax {
                    ontology_type: item.ontology_type,
                    native_kind: item.native_kind,
                    name: item.name,
                    span: item.span,
                    evidence: item.evidence,
                    language: language.slug().into(),
                })
                .collect::<Vec<_>>(),
        };

        insert_projected_items(result, &element, &relative_path, projected)?;
    }

    Ok(())
}

fn insert_projected_items(
    result: &mut DiscoveryResult,
    element: &NodeId,
    relative_path: &str,
    items: Vec<ProjectedSyntax>,
) -> Result<(), String> {
    let mut counts = BTreeMap::<OntologyType, usize>::new();
    let mut occurrences = BTreeMap::<String, usize>::new();
    let language = items
        .first()
        .map(|item| item.language.clone())
        .or_else(|| syntax_language_name(relative_path));

    for (ordinal, item) in items.iter().enumerate() {
        let base_key = format!(
            "{}:{}:{}",
            item.ontology_type.slug().to_lowercase(),
            item.native_kind,
            item.name.as_deref().unwrap_or("anonymous")
        );
        let occurrence = occurrences
            .entry(base_key.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let semantic_local = if item.name.is_some() {
            if *occurrence == 1 {
                format!("syntax:{base_key}")
            } else {
                format!("syntax:{base_key}:~{occurrence}")
            }
        } else {
            format!("syntax:{base_key}:~{ordinal}")
        };
        let semantic_id = NodeId::scoped(element, &semantic_local);
        result
            .graph
            .insert_node(Node {
                id: semantic_id.clone(),
                parent: None,
                ontology_type: item.ontology_type,
                kind: None,
                name: item.name.clone(),
                source_span: Some(item.span.clone()),
                materialized: true,
            })
            .map_err(|error| error.to_string())?;
        result
            .graph
            .add_edge(element.clone(), semantic_id, EdgeKind::ProjectsTo)
            .map_err(|error| error.to_string())?;
        result.observations.push(DiscoveryObservation {
            path: relative_path.into(),
            kind: "syntax-item".into(),
            language: Some(item.language.clone()),
            evidence: format!(
                "native_kind={}; ontology_type={}; evidence={}",
                item.native_kind,
                item.ontology_type.slug(),
                item.evidence
            ),
        });
        *counts.entry(item.ontology_type).or_default() += 1;
    }

    result.observations.push(DiscoveryObservation {
        path: relative_path.into(),
        kind: "syntax".into(),
        language: language.clone(),
        evidence: format!(
            "deterministic syntax projection; {} observations",
            items.len()
        ),
    });
    if !counts.is_empty() {
        result.observations.push(DiscoveryObservation {
            path: relative_path.into(),
            kind: "syntax-summary".into(),
            language,
            evidence: counts
                .into_iter()
                .map(|(ty, count)| format!("{}={count}", ty.slug()))
                .collect::<Vec<_>>()
                .join(", "),
        });
    }

    Ok(())
}

fn resolve_element(result: &DiscoveryResult, observation_path: &str) -> Option<NodeId> {
    result
        .graph
        .nodes_by_type(OntologyType::Element)
        .filter_map(|element| {
            let component = element.parent.as_ref()?;
            let relative = component.as_str().rsplit_once("/component:")?.1;
            if !observation_path.ends_with(relative) {
                return None;
            }
            let source_name = result
                .graph
                .path_to_root(&element.id)
                .into_iter()
                .find(|node| node.ontology_type == OntologyType::Source)
                .and_then(|node| {
                    node.id
                        .as_str()
                        .rsplit_once("/source:")
                        .map(|(_, value)| value)
                });
            let source_score = source_name
                .is_some_and(|name| observation_path.split('/').any(|segment| segment == name));
            Some((source_score, relative.len(), element.id.clone()))
        })
        .max_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| right.2.cmp(&left.2))
        })
        .map(|(_, _, id)| id)
}

#[derive(Debug, Clone, Copy)]
enum SyntaxAdapter {
    Rust,
    Language(Language),
}

fn syntax_adapter_for_path(path: &Path) -> Option<SyntaxAdapter> {
    match path.extension().and_then(|value| value.to_str())? {
        "rs" => Some(SyntaxAdapter::Rust),
        "ts" | "tsx" => Some(SyntaxAdapter::Language(Language::TypeScript)),
        "js" | "jsx" => Some(SyntaxAdapter::Language(Language::JavaScript)),
        "py" => Some(SyntaxAdapter::Language(Language::Python)),
        "go" => Some(SyntaxAdapter::Language(Language::Go)),
        "java" => Some(SyntaxAdapter::Language(Language::Java)),
        "kt" => Some(SyntaxAdapter::Language(Language::Kotlin)),
        _ => None,
    }
}

fn syntax_language_name(path: &str) -> Option<String> {
    let path = Path::new(path);
    match syntax_adapter_for_path(path)? {
        SyntaxAdapter::Rust => Some("rust".into()),
        SyntaxAdapter::Language(language) => Some(language.slug().into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_discovery::{discover_workspace, DiscoveryOptions};
    use ontology_registry::OntologyRegistry;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn registry() -> Arc<OntologyRegistry> {
        Arc::new(
            OntologyRegistry::from_json(include_str!(
                "../../../specifications/universal-ontology-v1.0.json"
            ))
            .unwrap(),
        )
    }

    fn temp_root(label: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("ontology-syntax-{label}-{stamp}"))
    }

    fn discover_node_fixture(root: &Path) -> DiscoveryResult {
        discover_workspace(
            root,
            registry(),
            DiscoveryOptions {
                include_files: true,
                max_depth: Some(4),
                parse_rust_ast: false,
            },
        )
        .unwrap()
    }

    #[test]
    fn projects_typescript_syntax_without_synthetic_structure() {
        let root = temp_root("typescript");
        fs::create_dir_all(root.join("src/features")).unwrap();
        fs::write(
            root.join("package.json"),
            r#"{"name":"typescript","version":"0.1.0"}"#,
        )
        .unwrap();
        fs::write(
            root.join("src/features/index.ts"),
            "import { value } from './value';\nexport interface Demo { id: string }\nexport const load = () => value;\nexport const answer = 42;\n",
        )
        .unwrap();

        let mut result = discover_node_fixture(&root);
        project_workspace_syntax(&mut result, &root).unwrap();

        assert_eq!(result.graph.nodes_by_type(OntologyType::Unit).count(), 0);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Module).count(), 0);
        assert!(result.graph.nodes_by_type(OntologyType::Entity).count() >= 1);
        assert!(result.graph.nodes_by_type(OntologyType::Function).count() >= 1);
        assert!(result.graph.nodes_by_type(OntologyType::Value).count() >= 1);
        assert!(
            result
                .graph
                .nodes_by_type(OntologyType::Instruction)
                .count()
                >= 1
        );
        assert!(result
            .observations
            .iter()
            .any(|observation| observation.kind == "syntax"
                && observation.language.as_deref() == Some("typescript")));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn named_typescript_ids_ignore_line_movement() {
        let root = temp_root("stable-id");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("package.json"),
            r#"{"name":"stable-id","version":"0.1.0"}"#,
        )
        .unwrap();
        let file = root.join("src/index.ts");
        fs::write(&file, "export function stable() { return 1; }\n").unwrap();

        let mut first = discover_node_fixture(&root);
        project_workspace_syntax(&mut first, &root).unwrap();
        let first_ids = first
            .graph
            .nodes_by_type(OntologyType::Function)
            .map(|node| node.id.clone())
            .collect::<Vec<_>>();

        fs::write(
            &file,
            "\n\n// moved\nexport function stable() { return 1; }\n",
        )
        .unwrap();
        let mut second = discover_node_fixture(&root);
        project_workspace_syntax(&mut second, &root).unwrap();
        let second_ids = second
            .graph
            .nodes_by_type(OntologyType::Function)
            .map(|node| node.id.clone())
            .collect::<Vec<_>>();

        assert_eq!(first_ids, second_ids);
        fs::remove_dir_all(root).unwrap();
    }
}
