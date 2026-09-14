use ontology_core::{EdgeKind, OntologyType};
use ontology_discovery::{
    discover_workspace, discover_workspace_with_syntax, DiscoveryOptions,
};
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
        "import { value } from './value';\nexport interface Demo { id: string }\nexport const load = () => value;\nexport const answer = 42;\n",
    )
    .unwrap();

    let result = discover_workspace_with_syntax(
        &root,
        registry(),
        DiscoveryOptions {
            include_files: true,
            max_depth: Some(4),
            parse_rust_ast: false,
        },
        true,
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
    assert!(result.graph.nodes_by_type(OntologyType::Entity).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Function).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Value).count() >= 1);
    assert!(result.graph.nodes_by_type(OntologyType::Instruction).count() >= 1);

    let element = result
        .graph
        .nodes_by_type(OntologyType::Element)
        .next()
        .unwrap();
    assert!(result
        .graph
        .outgoing_kind(&element.id, EdgeKind::ProjectsTo)
        .next()
        .is_some());
    assert!(result
        .observations
        .iter()
        .any(|observation| observation.kind == "repository"
            && observation.language.as_deref() == Some("node")));
    assert!(result
        .observations
        .iter()
        .any(|observation| observation.kind == "syntax"
            && observation.language.as_deref() == Some("typescript")));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn named_typescript_semantic_ids_ignore_line_movement() {
    let root = temp_root("typescript-id-stability");
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("package.json"),
        r#"{"name":"typescript-id-stability","version":"0.1.0"}"#,
    )
    .unwrap();
    let file = root.join("src/index.ts");
    fs::write(&file, "export function stable() { return 1; }\n").unwrap();

    let options = DiscoveryOptions {
        include_files: false,
        max_depth: Some(4),
        parse_rust_ast: false,
    };
    let first = discover_workspace_with_syntax(&root, registry(), options.clone(), true).unwrap();
    let first_ids = first
        .graph
        .nodes_by_type(OntologyType::Function)
        .map(|node| node.id.clone())
        .collect::<Vec<_>>();

    fs::write(
        &file,
        "\n\n// declaration moved without semantic identity change\nexport function stable() { return 1; }\n",
    )
    .unwrap();
    let second = discover_workspace_with_syntax(&root, registry(), options, true).unwrap();
    let second_ids = second
        .graph
        .nodes_by_type(OntologyType::Function)
        .map(|node| node.id.clone())
        .collect::<Vec<_>>();

    assert_eq!(first_ids, second_ids);
    fs::remove_dir_all(root).unwrap();
}
