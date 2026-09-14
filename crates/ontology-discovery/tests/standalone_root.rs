use ontology_core::OntologyType;
use ontology_discovery::{discover_workspace, DiscoveryOptions};
use ontology_registry::OntologyRegistry;
use std::fs;
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
    std::env::temp_dir().join(format!("ontology-{label}-{stamp}"))
}

#[test]
fn discovers_standalone_rust_repository_root() {
    let root = temp_root("standalone-rust");
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname='standalone-rust'\nversion='0.1.0'\n",
    )
    .unwrap();
    fs::write(
        root.join("src/lib.rs"),
        "pub struct Demo;\npub fn load() -> Demo { Demo }\n",
    )
    .unwrap();

    let result = discover_workspace(
        &root,
        registry(),
        DiscoveryOptions {
            include_files: true,
            max_depth: Some(4),
            parse_rust_ast: true,
        },
    )
    .unwrap();

    assert_eq!(
        result.graph.nodes_by_type(OntologyType::Ecosystem).count(),
        0
    );
    assert_eq!(result.graph.nodes_by_type(OntologyType::Project).count(), 0);
    assert_eq!(
        result.graph.nodes_by_type(OntologyType::Repository).count(),
        1
    );
    assert_eq!(result.graph.nodes_by_type(OntologyType::Source).count(), 1);
    assert_eq!(result.graph.nodes_by_type(OntologyType::Unit).count(), 0);
    assert_eq!(result.graph.nodes_by_type(OntologyType::Module).count(), 0);
    assert!(result.graph.nodes_by_type(OntologyType::Component).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Element).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Entity).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Function).count() >= 1);
    assert!(result
        .observations
        .iter()
        .any(|observation| observation.kind == "rust-ast"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn discovers_standalone_node_repository_without_inventing_levels() {
    let root = temp_root("standalone-node");
    fs::create_dir_all(root.join("src/features")).unwrap();
    fs::write(
        root.join("package.json"),
        r#"{"name":"standalone-node","version":"0.1.0"}"#,
    )
    .unwrap();
    fs::write(
        root.join("src/features/index.ts"),
        "export const value = 1;\n",
    )
    .unwrap();

    let result = discover_workspace(
        &root,
        registry(),
        DiscoveryOptions {
            include_files: true,
            max_depth: Some(4),
            parse_rust_ast: true,
        },
    )
    .unwrap();

    assert_eq!(
        result.graph.nodes_by_type(OntologyType::Repository).count(),
        1
    );
    assert_eq!(result.graph.nodes_by_type(OntologyType::Source).count(), 1);
    assert_eq!(result.graph.nodes_by_type(OntologyType::Unit).count(), 0);
    assert_eq!(result.graph.nodes_by_type(OntologyType::Module).count(), 0);
    assert!(result.graph.nodes_by_type(OntologyType::Component).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Element).count() >= 1);
    assert!(result
        .observations
        .iter()
        .any(|observation| observation.kind == "repository"
            && observation.language.as_deref() == Some("node")));

    fs::remove_dir_all(root).unwrap();
}
