mod preflight;
mod syntax_projection;

mod legacy {
    include!("main.rs");

    pub(super) fn run() {
        main();
    }
}

use clap::{Parser, Subcommand};
use ontology_core::{OntologyType, ONTOLOGY_VERSION};
use ontology_discovery::{discover_workspace, DiscoveryError, DiscoveryOptions};
use ontology_registry::OntologyRegistry;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use syntax_projection::project_workspace_syntax;

const DEFAULT_REGISTRY: &str = "specifications/universal-ontology-v1.0.json";
const DISCOVERY_SCHEMA: &str = "universal-ontology-engine/discovery-v1";
const EXIT_INPUT: i32 = 2;
const EXIT_ENGINE: i32 = 3;

#[derive(Parser, Debug)]
#[command(name = "ontology-engine", version = ONTOLOGY_VERSION)]
struct DiscoveryCli {
    #[arg(long, global = true, default_value = DEFAULT_REGISTRY)]
    registry: PathBuf,
    #[command(subcommand)]
    command: DiscoveryCommand,
}

#[derive(Subcommand, Debug)]
enum DiscoveryCommand {
    Discover {
        workspace: PathBuf,
        #[arg(long)]
        include_files: bool,
        #[arg(long = "rust-ast", conflicts_with = "syntax")]
        rust_ast: bool,
        #[arg(long, conflicts_with = "rust_ast")]
        syntax: bool,
        #[arg(long)]
        max_depth: Option<usize>,
    },
}

fn main() {
    if std::env::args().any(|argument| argument == "discover") {
        run_discover();
    } else {
        legacy::run();
    }
}

fn run_discover() {
    let cli = DiscoveryCli::parse();
    let DiscoveryCommand::Discover {
        workspace,
        include_files,
        rust_ast,
        syntax,
        max_depth,
    } = cli.command;

    if let Err(error) = preflight::validate_workspace(&workspace, max_depth) {
        eprintln!("workspace preflight failed: {error}");
        std::process::exit(EXIT_INPUT);
    }

    let registry = match OntologyRegistry::load(&cli.registry) {
        Ok(registry) => Arc::new(registry),
        Err(error) => {
            eprintln!(
                "failed to load ontology registry `{}`: {error}",
                cli.registry.display()
            );
            std::process::exit(EXIT_INPUT);
        }
    };

    let mut result = match discover_workspace(
        &workspace,
        registry,
        DiscoveryOptions {
            include_files,
            max_depth,
            parse_rust_ast: rust_ast,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("discovery failed: {error}");
            std::process::exit(discovery_exit_code(&error));
        }
    };

    if syntax {
        if let Err(error) = project_workspace_syntax(&mut result, &workspace) {
            eprintln!("syntax projection failed: {error}");
            std::process::exit(EXIT_ENGINE);
        }
    }

    let mode = if syntax {
        "syntax-projection"
    } else if rust_ast {
        "rust-ast"
    } else {
        "structural"
    };

    let mut by_level = BTreeMap::new();
    for ty in OntologyType::ALL {
        let count = result.graph.nodes_by_type(ty).count();
        if count > 0 {
            by_level.insert(format!("{:02}_{}", ty.level(), ty.slug()), count);
        }
    }

    let summary = serde_json::json!({
        "schema": DISCOVERY_SCHEMA,
        "ontology": ONTOLOGY_VERSION,
        "workspace": normalize_path(&workspace),
        "read_only": true,
        "mode": mode,
        "nodes": result.graph.len(),
        "edges": result.graph.edge_len(),
        "nodes_by_level": by_level,
        "observations": result.observations
    });
    emit_json(&summary);
}

fn discovery_exit_code(error: &DiscoveryError) -> i32 {
    match error {
        DiscoveryError::Graph(_) => EXIT_ENGINE,
        DiscoveryError::MissingWorkspace(_)
        | DiscoveryError::NotDirectory(_)
        | DiscoveryError::ReadDir { .. }
        | DiscoveryError::ReadSource { .. } => EXIT_INPUT,
    }
}

fn normalize_path(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn emit_json(value: &serde_json::Value) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize machine output: {error}");
            std::process::exit(EXIT_ENGINE);
        }
    }
}
