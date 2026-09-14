use ontology_core::{EdgeKind, Node, NodeId, OntologyType, SourceSpan};
use ontology_graph::OntologyGraph;
use ontology_registry::OntologyRegistry;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const IGNORED: &[&str] = &[
    ".git", ".next", ".turbo", "node_modules", "target", "dist", "build", "coverage",
    "__pycache__", ".venv", "venv", ".idea", ".vscode", "vendor",
];
const SOURCE_DIRS: &[&str] = &[
    "src", "app", "apps", "packages", "crates", "cmd", "internal", "lib", "libs", "pkg",
];
const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "go", "py", "java", "kt", "rb", "php", "cs", "cpp", "h", "hpp",
];
const REPOSITORY_MANIFESTS: &[&str] = &[
    "Cargo.toml", "package.json", "go.mod", "pyproject.toml", "setup.py", "pom.xml", "build.gradle", "build.gradle.kts",
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DiscoveryObservation {
    pub path: String,
    pub kind: String,
    pub language: Option<String>,
    pub evidence: String,
}

#[derive(Debug, Clone, Default)]
pub struct DiscoveryOptions {
    pub include_files: bool,
    pub max_depth: Option<usize>,
    pub parse_rust_ast: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("workspace does not exist: {0}")]
    MissingWorkspace(PathBuf),
    #[error("workspace is not a directory: {0}")]
    NotDirectory(PathBuf),
    #[error("failed to read `{path}`: {source}")]
    ReadDir { path: PathBuf, source: std::io::Error },
    #[error("failed to read source `{path}`: {source}")]
    ReadSource { path: PathBuf, source: std::io::Error },
    #[error("graph insertion failed: {0}")]
    Graph(#[from] ontology_core::OntologyError),
}

#[derive(Debug)]
pub struct DiscoveryResult {
    pub graph: OntologyGraph,
    pub observations: Vec<DiscoveryObservation>,
}

pub fn discover_workspace(
    workspace: impl AsRef<Path>,
    registry: Arc<OntologyRegistry>,
    options: DiscoveryOptions,
) -> Result<DiscoveryResult, DiscoveryError> {
    let workspace = workspace.as_ref();
    if !workspace.exists() { return Err(DiscoveryError::MissingWorkspace(workspace.to_path_buf())); }
    if !workspace.is_dir() { return Err(DiscoveryError::NotDirectory(workspace.to_path_buf())); }

    let mut graph = OntologyGraph::new(registry);
    let mut observations = Vec::new();
    let universe = insert_node(&mut graph, None, OntologyType::Universe, NodeId::new("universe"), true)?;

    for ecosystem_dir in read_dir_sorted(workspace)?.into_iter().filter(|p| is_ecosystem_dir(p)) {
        let ecosystem_name = file_name(&ecosystem_dir, "ecosystem");
        let ecosystem_id = NodeId::scoped(&universe, &format!("ecosystem:{ecosystem_name}"));
        insert_node(&mut graph, Some(universe.clone()), OntologyType::Ecosystem, ecosystem_id.clone(), true)?;
        observations.push(DiscoveryObservation {
            path: rel(workspace, &ecosystem_dir), kind: "ecosystem".into(), language: None,
            evidence: "ecosystem-* directory".into(),
        });
        discover_ecosystem(&mut graph, &ecosystem_id, &ecosystem_dir, &options, workspace, &mut observations)?;
    }
    Ok(DiscoveryResult { graph, observations })
}

fn discover_ecosystem(
    graph: &mut OntologyGraph,
    ecosystem: &NodeId,
    ecosystem_dir: &Path,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    for child in read_dir_sorted(ecosystem_dir)?.into_iter().filter(|p| is_real_dir(p)) {
        if is_repository(&child) {
            discover_repository(graph, ecosystem, &child, options, workspace, observations)?;
            continue;
        }
        let project_name = file_name(&child, "project");
        let project_id = NodeId::scoped(ecosystem, &format!("project:{project_name}"));
        insert_node(graph, Some(ecosystem.clone()), OntologyType::Project, project_id.clone(), true)?;
        observations.push(DiscoveryObservation {
            path: rel(workspace, &child), kind: "project".into(), language: None,
            evidence: "ecosystem child directory".into(),
        });
        for repo in read_dir_sorted(&child)?.into_iter().filter(|p| is_real_dir(p) && is_repository(p)) {
            discover_repository(graph, &project_id, &repo, options, workspace, observations)?;
        }
    }
    Ok(())
}

fn discover_repository(
    graph: &mut OntologyGraph,
    parent: &NodeId,
    repo_dir: &Path,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let repo_name = file_name(repo_dir, "repository");
    let repo_id = NodeId::scoped(parent, &format!("repository:{repo_name}"));
    insert_node(graph, Some(parent.clone()), OntologyType::Repository, repo_id.clone(), true)?;
    let language = detect_language(repo_dir);
    observations.push(DiscoveryObservation {
        path: rel(workspace, repo_dir), kind: "repository".into(), language: language.clone(),
        evidence: "git or native manifest".into(),
    });

    let source_dirs = read_dir_sorted(repo_dir)?.into_iter()
        .filter(|p| is_real_dir(p) && SOURCE_DIRS.contains(&file_name(p, "").as_str()))
        .collect::<Vec<_>>();
    if source_dirs.is_empty() {
        if contains_source_files_recursive(repo_dir, options.max_depth)? {
            discover_source_tree(graph, &repo_id, repo_dir, &language, options, workspace, observations)?;
        }
    } else {
        for source_dir in source_dirs {
            discover_source_tree(graph, &repo_id, &source_dir, &language, options, workspace, observations)?;
        }
    }
    Ok(())
}

fn discover_source_tree(
    graph: &mut OntologyGraph,
    repository: &NodeId,
    source_root: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let source_name = file_name(source_root, "source");
    let source_id = NodeId::scoped(repository, &format!("source:{source_name}"));
    insert_node(graph, Some(repository.clone()), OntologyType::Source, source_id.clone(), true)?;
    observations.push(DiscoveryObservation {
        path: rel(workspace, source_root), kind: "source".into(), language: language.clone(),
        evidence: "native source boundary".into(),
    });
    observations.push(DiscoveryObservation {
        path: rel(workspace, source_root), kind: "unmaterialized-levels".into(), language: language.clone(),
        evidence: "UNIT and MODULE omitted because discovery has no native ownership evidence; canonical intermediate levels may remain unmaterialized".into(),
    });
    walk_source_files(graph, &source_id, source_root, source_root, language, options, workspace, observations, 0)
}

fn walk_source_files(
    graph: &mut OntologyGraph,
    structural_parent: &NodeId,
    root: &Path,
    current: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
    depth: usize,
) -> Result<(), DiscoveryError> {
    if options.max_depth.is_some_and(|max| depth > max) { return Ok(()); }
    for path in read_dir_sorted(current)? {
        if is_ignored(&path) { continue; }
        if path.is_dir() {
            walk_source_files(graph, structural_parent, root, &path, language, options, workspace, observations, depth + 1)?;
        } else if is_source_file(&path) {
            let relative = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            let component_id = NodeId::scoped(structural_parent, &format!("component:{relative}"));
            insert_node(graph, Some(structural_parent.clone()), OntologyType::Component, component_id.clone(), true)?;
            observations.push(DiscoveryObservation {
                path: rel(workspace, &path), kind: "component".into(), language: language.clone(),
                evidence: "source file".into(),
            });
            let label = path.file_stem().and_then(|v| v.to_str()).unwrap_or("element");
            let element_id = NodeId::scoped(&component_id, &format!("element:{label}"));
            insert_node(graph, Some(component_id), OntologyType::Element, element_id.clone(), true)?;
            observations.push(DiscoveryObservation {
                path: rel(workspace, &path), kind: "element".into(), language: language.clone(),
                evidence: "file boundary; language adapter may project semantics".into(),
            });
            if options.parse_rust_ast && language.as_deref() == Some("rust") {
                discover_rust_semantics(graph, &element_id, &path, workspace, observations)?;
            }
            if options.include_files {
                let execution_id = NodeId::scoped(&element_id, "execution:observed");
                insert_node(graph, None, OntologyType::Execution, execution_id.clone(), true)?;
                graph.add_edge(execution_id.clone(), element_id.clone(), EdgeKind::ObservedAt)?;
                observations.push(DiscoveryObservation {
                    path: rel(workspace, &path), kind: "execution".into(), language: language.clone(),
                    evidence: "observed file boundary; represented by ObservedAt relation, not containment".into(),
                });
            }
        }
    }
    Ok(())
}

fn discover_rust_semantics(
    graph: &mut OntologyGraph,
    element: &NodeId,
    path: &Path,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let source = fs::read_to_string(path).map_err(|source| DiscoveryError::ReadSource { path: path.to_path_buf(), source })?;
    match ontology_rust::parse(&source) {
        Ok(items) => {
            let mut counts = BTreeMap::<OntologyType, usize>::new();
            let mut occurrences = BTreeMap::<String, usize>::new();
            for (ordinal, item) in items.iter().enumerate() {
                let native_kind = item.native_kind.as_str();
                let base_key = format!("{}:{}:{}", item.ontology_type.slug().to_lowercase(), native_kind, item.name.as_deref().unwrap_or("anonymous"));
                let occurrence = occurrences.entry(base_key.clone()).and_modify(|count| *count += 1).or_insert(1);
                let semantic_local = if item.name.is_some() {
                    if *occurrence == 1 { format!("ast:{base_key}") } else { format!("ast:{base_key}:~{occurrence}") }
                } else {
                    format!("ast:{base_key}:~{ordinal}")
                };
                let semantic_id = NodeId::scoped(element, &semantic_local);
                graph.insert_node(Node {
                    id: semantic_id.clone(), parent: None, ontology_type: item.ontology_type,
                    kind: None, name: item.name.clone(),
                    source_span: Some(SourceSpan {
                        line_start: item.location.line_start as u32, column_start: item.location.column_start as u32,
                        line_end: item.location.line_end as u32, column_end: item.location.column_end as u32,
                    }),
                    materialized: true,
                })?;
                graph.add_edge(element.clone(), semantic_id, EdgeKind::ProjectsTo)?;
                observations.push(DiscoveryObservation {
                    path: rel(workspace, path), kind: "rust-ast-item".into(), language: Some("rust".into()),
                    evidence: format!("native_kind={native_kind}; ontology_type={}", item.ontology_type.slug()),
                });
                *counts.entry(item.ontology_type).or_default() += 1;
            }
            observations.push(DiscoveryObservation {
                path: rel(workspace, path), kind: "rust-ast".into(), language: Some("rust".into()),
                evidence: format!("syn AST; {} semantic observations", items.len()),
            });
            if !counts.is_empty() {
                observations.push(DiscoveryObservation {
                    path: rel(workspace, path), kind: "rust-ast-summary".into(), language: Some("rust".into()),
                    evidence: counts.into_iter().map(|(ty, count)| format!("{}={count}", ty.slug())).collect::<Vec<_>>().join(", "),
                });
            }
        }
        Err(error) => observations.push(DiscoveryObservation {
            path: rel(workspace, path), kind: "rust-ast-error".into(), language: Some("rust".into()),
            evidence: format!("parse error preserved as observation: {error}"),
        }),
    }
    Ok(())
}

fn insert_node(
    graph: &mut OntologyGraph,
    parent: Option<NodeId>,
    ty: OntologyType,
    id: impl Into<NodeId>,
    materialized: bool,
) -> Result<NodeId, DiscoveryError> {
    let id = id.into();
    graph.insert_node(Node { id: id.clone(), parent, ontology_type: ty, kind: None, name: None, source_span: None, materialized })?;
    Ok(id)
}

fn read_dir_sorted(path: &Path) -> Result<Vec<PathBuf>, DiscoveryError> {
    let mut out = fs::read_dir(path)
        .map_err(|source| DiscoveryError::ReadDir { path: path.to_path_buf(), source })?
        .filter_map(Result::ok).map(|entry| entry.path()).collect::<Vec<_>>();
    out.sort_by_key(|p| file_name(p, "").to_ascii_lowercase());
    Ok(out)
}

fn contains_source_files_recursive(path: &Path, max_depth: Option<usize>) -> Result<bool, DiscoveryError> {
    fn walk(path: &Path, depth: usize, max_depth: Option<usize>) -> Result<bool, DiscoveryError> {
        if max_depth.is_some_and(|max| depth > max) { return Ok(false); }
        for child in read_dir_sorted(path)? {
            if is_ignored(&child) { continue; }
            if is_source_file(&child) { return Ok(true); }
            if child.is_dir() && walk(&child, depth + 1, max_depth)? { return Ok(true); }
        }
        Ok(false)
    }
    walk(path, 0, max_depth)
}

fn is_ecosystem_dir(path: &Path) -> bool { path.is_dir() && file_name(path, "").starts_with("ecosystem-") }
fn is_real_dir(path: &Path) -> bool { path.is_dir() && !is_ignored(path) && !file_name(path, "").starts_with('.') }
fn is_ignored(path: &Path) -> bool { IGNORED.contains(&file_name(path, "").as_str()) }
fn is_source_file(path: &Path) -> bool { path.is_file() && path.extension().and_then(|v| v.to_str()).is_some_and(|e| SOURCE_EXTENSIONS.contains(&e)) }
fn file_name(path: &Path, fallback: &str) -> String { path.file_name().and_then(|v| v.to_str()).map(str::to_owned).unwrap_or_else(|| fallback.to_owned()) }
fn rel(root: &Path, path: &Path) -> String { path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/") }
fn is_repository(path: &Path) -> bool { path.join(".git").exists() || REPOSITORY_MANIFESTS.iter().any(|name| path.join(name).is_file()) }

fn detect_language(repo: &Path) -> Option<String> {
    let has = |name: &str| repo.join(name).is_file();
    if has("Cargo.toml") { Some("rust".into()) }
    else if has("package.json") || has("pnpm-workspace.yaml") { Some("node".into()) }
    else if has("go.mod") { Some("go".into()) }
    else if has("pyproject.toml") || has("setup.py") { Some("python".into()) }
    else if has("pom.xml") || has("build.gradle") || has("build.gradle.kts") { Some("java".into()) }
    else if read_dir_sorted(repo).ok().is_some_and(|items| items.iter().any(|p| p.extension().and_then(|v| v.to_str()) == Some("kt"))) { Some("kotlin".into()) }
    else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn registry() -> Arc<OntologyRegistry> {
        Arc::new(OntologyRegistry::from_json(include_str!("../../../specifications/universal-ontology-v1.0.json")).unwrap())
    }

    fn fixture() -> PathBuf {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("ontology-discovery-{stamp}"));
        fs::create_dir_all(root.join("ecosystem-demo/project-a/repo-a/src/core")).unwrap();
        fs::write(root.join("ecosystem-demo/project-a/repo-a/Cargo.toml"), "[package]\nname='repo-a'\nversion='0.1.0'\n").unwrap();
        fs::write(root.join("ecosystem-demo/project-a/repo-a/src/core/lib.rs"), "pub fn demo() {}\n").unwrap();
        root
    }

    #[test]
    fn discovery_does_not_fabricate_unit_or_module() {
        let root = fixture();
        let result = discover_workspace(&root, registry(), DiscoveryOptions::default()).unwrap();
        assert_eq!(result.graph.nodes_by_type(OntologyType::Unit).count(), 0);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Module).count(), 0);
        assert!(result.observations.iter().any(|x| x.kind == "unmaterialized-levels"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn discovers_nested_rust_source_and_execution_observation() {
        let root = fixture();
        let result = discover_workspace(&root, registry(), DiscoveryOptions { include_files: true, max_depth: Some(6), parse_rust_ast: true }).unwrap();
        assert_eq!(result.graph.nodes_by_type(OntologyType::Ecosystem).count(), 1);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Repository).count(), 1);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Source).count(), 1);
        assert!(result.graph.nodes_by_type(OntologyType::Execution).count() >= 1);
        assert!(result.graph.nodes_by_type(OntologyType::Function).count() >= 1);
        let execution = result.graph.nodes_by_type(OntologyType::Execution).next().unwrap();
        assert_eq!(execution.parent, None);
        assert!(result.graph.outgoing_kind(&execution.id, EdgeKind::ObservedAt).next().is_some());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn project_to_does_not_change_structural_parent() {
        let root = fixture();
        let result = discover_workspace(&root, registry(), DiscoveryOptions { include_files: false, max_depth: Some(6), parse_rust_ast: true }).unwrap();
        let element = result.graph.nodes_by_type(OntologyType::Element).next().unwrap();
        let projected = result.graph.outgoing_kind(&element.id, EdgeKind::ProjectsTo).next().unwrap();
        let semantic = result.graph.node(&projected.to).unwrap();
        assert_eq!(semantic.parent, None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rust_parse_errors_are_observations_not_graph_corruption() {
        let root = fixture();
        let bad = root.join("ecosystem-demo/project-a/repo-a/src/core/bad.rs");
        fs::write(&bad, "fn broken( {").unwrap();
        let result = discover_workspace(&root, registry(), DiscoveryOptions { include_files: false, max_depth: Some(6), parse_rust_ast: true }).unwrap();
        assert!(result.observations.iter().any(|x| x.kind == "rust-ast-error"));
        assert!(result.graph.nodes_by_type(OntologyType::Element).count() >= 2);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rust_semantic_ids_do_not_depend_on_line_numbers() {
        let root = fixture();
        let file = root.join("ecosystem-demo/project-a/repo-a/src/core/lib.rs");
        let first = discover_workspace(&root, registry(), DiscoveryOptions { include_files: false, max_depth: Some(6), parse_rust_ast: true }).unwrap();
        let first_ids = first.graph.nodes_by_type(OntologyType::Function).map(|node| node.id.clone()).collect::<Vec<_>>();
        fs::write(&file, "\n\n\n// moved\npub fn demo() {}\n").unwrap();
        let second = discover_workspace(&root, registry(), DiscoveryOptions { include_files: false, max_depth: Some(6), parse_rust_ast: true }).unwrap();
        let second_ids = second.graph.nodes_by_type(OntologyType::Function).map(|node| node.id.clone()).collect::<Vec<_>>();
        assert_eq!(first_ids, second_ids);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn repository_directly_under_ecosystem_is_not_forced_into_fake_project() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("ontology-discovery-direct-{stamp}"));
        fs::create_dir_all(root.join("ecosystem-demo/repo-a/src")).unwrap();
        fs::write(root.join("ecosystem-demo/repo-a/Cargo.toml"), "[package]\nname='repo-a'\nversion='0.1.0'\n").unwrap();
        fs::write(root.join("ecosystem-demo/repo-a/src/lib.rs"), "pub fn demo() {}\n").unwrap();
        let result = discover_workspace(&root, registry(), DiscoveryOptions::default()).unwrap();
        assert_eq!(result.graph.nodes_by_type(OntologyType::Repository).count(), 1);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Project).count(), 0);
        fs::remove_dir_all(root).unwrap();
    }
}