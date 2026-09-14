use ontology_core::{Node, NodeId, OntologyType};
use ontology_graph::OntologyGraph;
use ontology_registry::OntologyRegistry;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const IGNORED: &[&str] = &[
    ".git", ".next", ".turbo", "node_modules", "target", "dist", "build",
    "coverage", "__pycache__", ".venv", "venv", ".idea", ".vscode",
];
const SOURCE_DIRS: &[&str] = &["src", "app", "apps", "packages", "crates", "cmd", "internal", "lib", "libs", "pkg"];
const SOURCE_EXTENSIONS: &[&str] = &["rs", "ts", "tsx", "js", "jsx", "go", "py", "java", "kt", "rb", "php", "cs", "cpp", "h", "hpp"];

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
    if !workspace.exists() {
        return Err(DiscoveryError::MissingWorkspace(workspace.to_path_buf()));
    }
    if !workspace.is_dir() {
        return Err(DiscoveryError::NotDirectory(workspace.to_path_buf()));
    }

    let mut graph = OntologyGraph::new(registry);
    let mut observations = Vec::new();
    let universe = insert_node(&mut graph, None, OntologyType::Universe, "universe", true)?;

    for ecosystem_dir in read_dir_sorted(workspace)?.into_iter().filter(|p| is_ecosystem_dir(p)) {
        let ecosystem_name = ecosystem_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
        let ecosystem_id = scoped(&universe, &format!("ecosystem:{ecosystem_name}"));
        insert_node(&mut graph, Some(universe.clone()), OntologyType::Ecosystem, ecosystem_id.as_str(), true)?;
        observations.push(DiscoveryObservation { path: rel(workspace, &ecosystem_dir), kind: "ecosystem".into(), language: None, evidence: "ecosystem-* directory".into() });

        for project_dir in read_dir_sorted(&ecosystem_dir)?.into_iter().filter(|p| is_real_dir(p)) {
            let project_name = project_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
            let project_id = scoped(&ecosystem_id, &format!("project:{project_name}"));
            insert_node(&mut graph, Some(ecosystem_id.clone()), OntologyType::Project, project_id.as_str(), true)?;

            for repo_dir in repository_children(&project_dir)? {
                let repo_name = repo_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                let repo_id = scoped(&project_id, &format!("repository:{repo_name}"));
                insert_node(&mut graph, Some(project_id.clone()), OntologyType::Repository, repo_id.as_str(), true)?;
                let language = detect_language(&repo_dir);
                observations.push(DiscoveryObservation { path: rel(workspace, &repo_dir), kind: "repository".into(), language: language.clone(), evidence: "git or native manifest".into() });
                discover_repository(&mut graph, &repo_id, &repo_dir, &language, &options, workspace, &mut observations)?;
            }
        }
    }

    Ok(DiscoveryResult { graph, observations })
}

fn discover_repository(
    graph: &mut OntologyGraph,
    repository: &NodeId,
    repo_dir: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let sources: Vec<PathBuf> = read_dir_sorted(repo_dir)?.into_iter()
        .filter(|p| p.is_dir() && SOURCE_DIRS.contains(&p.file_name().and_then(|v| v.to_str()).unwrap_or_default()))
        .collect();
    let source_paths = if sources.is_empty() {
        if contains_source_files(repo_dir)? { vec![repo_dir.to_path_buf()] } else { Vec::new() }
    } else { sources };

    for source_dir in source_paths {
        let source_name = if source_dir == repo_dir { "." } else { source_dir.file_name().and_then(|v| v.to_str()).unwrap_or("source") };
        let source_id = scoped(repository, &format!("source:{source_name}"));
        insert_node(graph, Some(repository.clone()), OntologyType::Source, source_id.as_str(), true)?;
        observations.push(DiscoveryObservation { path: rel(workspace, &source_dir), kind: "source".into(), language: language.clone(), evidence: "native source directory".into() });
        discover_units(graph, &source_id, &source_dir, language, options, workspace, observations)?;
    }
    Ok(())
}

fn discover_units(
    graph: &mut OntologyGraph,
    source: &NodeId,
    source_dir: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let mut unit_candidates = Vec::new();
    for path in read_dir_sorted(source_dir)? {
        if !is_real_dir(&path) || is_ignored(&path) { continue; }
        if is_native_unit(&path, language) { unit_candidates.push(path); }
    }
    if unit_candidates.is_empty() && contains_source_files(source_dir)? {
        unit_candidates.push(source_dir.to_path_buf());
    }

    for unit_dir in unit_candidates {
        let unit_name = unit_dir.file_name().and_then(|v| v.to_str()).unwrap_or("source");
        let unit_key = if unit_dir == source_dir { "." } else { unit_name };
        let unit_id = scoped(source, &format!("unit:{unit_key}"));
        insert_node(graph, Some(source.clone()), OntologyType::Unit, unit_id.as_str(), true)?;
        observations.push(DiscoveryObservation { path: rel(workspace, &unit_dir), kind: "unit".into(), language: language.clone(), evidence: unit_evidence(&unit_dir, language) });
        discover_modules(graph, &unit_id, &unit_dir, language, options, workspace, observations, 0)?;
    }
    Ok(())
}

fn discover_modules(
    graph: &mut OntologyGraph,
    unit: &NodeId,
    unit_dir: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
    depth: usize,
) -> Result<(), DiscoveryError> {
    if options.max_depth.map(|max| depth >= max).unwrap_or(false) { return Ok(()); }
    let dirs: Vec<PathBuf> = read_dir_sorted(unit_dir)?.into_iter().filter(|p| p.is_dir() && !is_ignored(p)).collect();
    if dirs.is_empty() {
        let module_id = scoped(unit, "module:.");
        insert_node(graph, Some(unit.clone()), OntologyType::Module, module_id.as_str(), true)?;
        discover_components(graph, &module_id, unit_dir, language, options, workspace, observations)?;
        return Ok(());
    }
    for module_dir in dirs {
        let name = module_dir.file_name().and_then(|v| v.to_str()).unwrap_or("module");
        let module_id = scoped(unit, &format!("module:{name}"));
        insert_node(graph, Some(unit.clone()), OntologyType::Module, module_id.as_str(), true)?;
        observations.push(DiscoveryObservation { path: rel(workspace, &module_dir), kind: "module".into(), language: language.clone(), evidence: "native directory grouping".into() });
        discover_components(graph, &module_id, &module_dir, language, options, workspace, observations)?;
    }
    Ok(())
}

fn discover_components(
    graph: &mut OntologyGraph,
    module: &NodeId,
    module_dir: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    for path in read_dir_sorted(module_dir)? {
        if is_ignored(&path) { continue; }
        if path.is_file() && is_source_file(&path) {
            let stem = path.file_stem().and_then(|v| v.to_str()).unwrap_or("component");
            let component_id = scoped(module, &format!("component:{stem}"));
            insert_node(graph, Some(module.clone()), OntologyType::Component, component_id.as_str(), true)?;
            observations.push(DiscoveryObservation { path: rel(workspace, &path), kind: "component".into(), language: language.clone(), evidence: "source file".into() });
            discover_element(graph, &component_id, &path, language, options, workspace, observations)?;
        } else if path.is_dir() && options.max_depth.map(|max| 1 < max).unwrap_or(true) {
            let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("component");
            let component_id = scoped(module, &format!("component:{name}"));
            insert_node(graph, Some(module.clone()), OntologyType::Component, component_id.as_str(), true)?;
            discover_element(graph, &component_id, &path, language, options, workspace, observations)?;
        }
    }
    Ok(())
}

fn discover_element(
    graph: &mut OntologyGraph,
    component: &NodeId,
    path: &Path,
    language: &Option<String>,
    options: &DiscoveryOptions,
    workspace: &Path,
    observations: &mut Vec<DiscoveryObservation>,
) -> Result<(), DiscoveryError> {
    let label = path.file_stem().or_else(|| path.file_name()).and_then(|v| v.to_str()).unwrap_or("element");
    let element_id = scoped(component, &format!("element:{label}"));
    insert_node(graph, Some(component.clone()), OntologyType::Element, element_id.as_str(), true)?;
    observations.push(DiscoveryObservation { path: rel(workspace, path), kind: "element".into(), language: language.clone(), evidence: if path.is_file() { "file boundary" } else { "directory boundary" }.into() });

    if options.parse_rust_ast && path.is_file() && path.extension().and_then(|v| v.to_str()) == Some("rs") {
        discover_rust_semantics(graph, &element_id, path, workspace, observations)?;
    }

    if options.include_files && path.is_file() {
        let execution_id = scoped(&element_id, "execution:observed");
        insert_node(graph, Some(element_id), OntologyType::Execution, execution_id.as_str(), true)?;
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
    let source = std::fs::read_to_string(path).map_err(|source| DiscoveryError::ReadSource { path: path.to_path_buf(), source })?;
    match ontology_rust::parse(&source) {
        Ok(items) => {
            let mut counts = std::collections::BTreeMap::<OntologyType, usize>::new();
            for (ordinal, item) in items.iter().enumerate() {
                let label = item.name.clone().unwrap_or_else(|| format!("anonymous-{ordinal}"));
                let semantic_id = scoped(element, &format!("ast:{}:{}:{}:{}", item.ontology_type.slug().to_lowercase(), label, item.location.line_start, item.location.column_start));
                graph.insert_node(Node {
                    id: semantic_id.clone(),
                    parent: None,
                    ontology_type: item.ontology_type,
                    kind: Some(item.native_kind.clone()),
                    name: item.name.clone(),
                    source_span: Some(ontology_core::SourceSpan {
                        line_start: item.location.line_start as u32,
                        column_start: item.location.column_start as u32,
                        line_end: item.location.line_end as u32,
                        column_end: item.location.column_end as u32,
                    }),
                    materialized: true,
                })?;
                graph.add_edge(element.clone(), semantic_id, ontology_core::EdgeKind::ProjectsTo)?;
                *counts.entry(item.ontology_type).or_default() += 1;
            }
            observations.push(DiscoveryObservation {
                path: rel(workspace, path),
                kind: "rust-ast".into(),
                language: Some("rust".into()),
                evidence: format!("syn AST; {} semantic observations", items.len()),
            });
            if !counts.is_empty() {
                observations.push(DiscoveryObservation {
                    path: rel(workspace, path),
                    kind: "rust-ast-summary".into(),
                    language: Some("rust".into()),
                    evidence: counts.into_iter().map(|(ty, count)| format!("{}={count}", ty.slug())).collect::<Vec<_>>().join(", "),
                });
            }
        }
        Err(error) => {
            observations.push(DiscoveryObservation {
                path: rel(workspace, path),
                kind: "rust-ast-error".into(),
                language: Some("rust".into()),
                evidence: format!("parse error preserved as observation: {error}"),
            });
        }
    }
    Ok(())
}

fn insert_node(graph: &mut OntologyGraph, parent: Option<NodeId>, ty: OntologyType, id: impl Into<String>, materialized: bool) -> Result<NodeId, DiscoveryError> {
    let id = NodeId::new(id);
    graph.insert_node(Node { id: id.clone(), parent, ontology_type: ty, kind: None, name: None, source_span: None, materialized })?;
    Ok(id)
}
fn scoped(parent: &NodeId, local: &str) -> NodeId { NodeId::scoped(parent, local) }
fn read_dir_sorted(path: &Path) -> Result<Vec<PathBuf>, DiscoveryError> {
    let mut out = std::fs::read_dir(path).map_err(|source| DiscoveryError::ReadDir { path: path.to_path_buf(), source })?.filter_map(Result::ok).map(|e| e.path()).collect::<Vec<_>>();
    out.sort_by_key(|p| p.file_name().map(|v| v.to_string_lossy().to_lowercase()));
    Ok(out)
}
fn is_ecosystem_dir(path: &Path) -> bool { path.is_dir() && path.file_name().map(|v| v.to_string_lossy().starts_with("ecosystem-")).unwrap_or(false) }
fn is_real_dir(path: &Path) -> bool { path.is_dir() && !is_ignored(path) && !path.file_name().map(|v| v.to_string_lossy().starts_with('.')).unwrap_or(false) }
fn is_ignored(path: &Path) -> bool { path.file_name().and_then(|v| v.to_str()).map(|name| IGNORED.contains(&name)).unwrap_or(false) }
fn is_source_file(path: &Path) -> bool { path.is_file() && path.extension().and_then(|v| v.to_str()).map(|e| SOURCE_EXTENSIONS.contains(&e)).unwrap_or(false) }
fn contains_source_files(path: &Path) -> Result<bool, DiscoveryError> { Ok(read_dir_sorted(path)?.into_iter().any(|p| is_source_file(&p))) }
fn is_native_unit(path: &Path, language: &Option<String>) -> bool {
    match language.as_deref() {
        Some("rust") => path.join("Cargo.toml").is_file(),
        Some("node") => path.join("package.json").is_file(),
        Some("python") => path.join("pyproject.toml").is_file() || path.join("__init__.py").is_file(),
        Some("go") | Some("java") | Some("kotlin") => contains_source_files(path).unwrap_or(false),
        _ => contains_source_files(path).unwrap_or(false),
    }
}
fn unit_evidence(path: &Path, language: &Option<String>) -> String {
    match language.as_deref() {
        Some("rust") if path.join("Cargo.toml").is_file() => "Cargo.toml".into(),
        Some("node") if path.join("package.json").is_file() => "package.json".into(),
        Some("python") if path.join("pyproject.toml").is_file() => "pyproject.toml".into(),
        _ => "native source grouping".into(),
    }
}
fn repository_children(project: &Path) -> Result<Vec<PathBuf>, DiscoveryError> {
    let dirs = read_dir_sorted(project)?;
    let direct = dirs.iter().filter(|p| is_real_dir(p) && is_repository(p)).cloned().collect::<Vec<_>>();
    if !direct.is_empty() { return Ok(direct); }
    Ok(dirs.into_iter().filter(|p| is_real_dir(p) && is_repository(p)).collect())
}
fn is_repository(path: &Path) -> bool { path.join(".git").exists() || ["Cargo.toml", "package.json", "go.mod", "pyproject.toml", "pom.xml", "build.gradle", "build.gradle.kts"].iter().any(|name| path.join(name).is_file()) }
fn detect_language(repo: &Path) -> Option<String> {
    let has = |name: &str| repo.join(name).is_file();
    if has("Cargo.toml") { Some("rust".into()) }
    else if has("package.json") || has("pnpm-workspace.yaml") { Some("node".into()) }
    else if has("go.mod") { Some("go".into()) }
    else if has("pyproject.toml") || has("setup.py") { Some("python".into()) }
    else if has("pom.xml") || has("build.gradle") || has("build.gradle.kts") { Some("java".into()) }
    else if read_dir_sorted(repo).ok().is_some_and(|v| v.iter().any(|p| p.extension().map(|e| e == "kt").unwrap_or(false))) { Some("kotlin".into()) }
    else { None }
}
fn rel(root: &Path, path: &Path) -> String { path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/") }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn registry() -> Arc<OntologyRegistry> { Arc::new(OntologyRegistry::from_json(include_str!("../../../specifications/universal-ontology-v1.0.json")).unwrap()) }

    fn temp_workspace() -> PathBuf {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("ontology-discovery-{stamp}"));
        fs::create_dir_all(path.join("ecosystem-demo/project-a/repo-a/src/core")).unwrap();
        fs::write(path.join("ecosystem-demo/project-a/repo-a/Cargo.toml"), "[package]\nname='repo-a'\nversion='0.1.0'\n").unwrap();
        fs::write(path.join("ecosystem-demo/project-a/repo-a/src/core/lib.rs"), "pub struct User { id: u64 }\npub fn demo() {}\n").unwrap();
        path
    }

    #[test]
    fn discovers_native_workspace_without_inventing_missing_levels() {
        let path = temp_workspace();
        let result = discover_workspace(&path, registry(), DiscoveryOptions { include_files: true, max_depth: Some(3), parse_rust_ast: true }).unwrap();
        assert_eq!(result.graph.nodes_by_type(OntologyType::Ecosystem).count(), 1);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Repository).count(), 1);
        assert_eq!(result.graph.nodes_by_type(OntologyType::Source).count(), 1);
        assert!(result.graph.nodes_by_type(OntologyType::Execution).count() >= 1);
        assert!(result.graph.nodes_by_type(OntologyType::Entity).count() >= 1);
        assert!(result.graph.edge_len() >= 1);
        assert!(result.observations.iter().any(|x| x.kind == "rust-ast"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rust_parse_errors_are_observations_not_graph_corruption() {
        let path = temp_workspace();
        fs::write(path.join("ecosystem-demo/project-a/repo-a/src/core/broken.rs"), "fn broken( {\n").unwrap();
        let result = discover_workspace(&path, registry(), DiscoveryOptions { parse_rust_ast: true, ..Default::default() }).unwrap();
        assert!(result.observations.iter().any(|x| x.kind == "rust-ast-error"));
        fs::remove_dir_all(path).unwrap();
    }
}
