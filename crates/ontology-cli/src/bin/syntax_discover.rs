#[path = "../syntax_projection.rs"]
mod syntax_projection;

use clap::Parser;
use ontology_core::{OntologyType, ONTOLOGY_VERSION};
use ontology_discovery::{discover_workspace, DiscoveryOptions};
use ontology_registry::OntologyRegistry;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use syntax_projection::project_workspace_syntax;

const DEFAULT_REGISTRY: &str = "specifications/universal-ontology-v1.0.json";

#[derive(Parser, Debug)]
#[command(name = "ontology-syntax-discover", version = ONTOLOGY_VERSION)]
struct Cli {
    workspace: PathBuf,
    #[arg(long, global = true, default_value = DEFAULT_REGISTRY)]
    registry: PathBuf,
    #[arg(long)]
    include_files: bool,
    #[arg(long)]
    max_depth: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let registry = match OntologyRegistry::load(&cli.registry) {
        Ok(registry) => Arc::new(registry),
        Err(error) => {
            eprintln!(
                "failed to load ontology registry `{}`: {error}",
                cli.registry.display()
            );
            std::process::exit(2);
        }
    };

    let mut result = match discover_workspace(
        &cli.workspace,
        registry,
        DiscoveryOptions {
            include_files: cli.include_files,
            max_depth: cli.max_depth,
            parse_rust_ast: false,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("discovery failed: {error}");
            std::process::exit(2);
        }
    };

    if let Err(error) = project_workspace_syntax(&mut result, &cli.workspace) {
        eprintln!("syntax projection failed: {error}");
        std::process::exit(2);
    }

    let mut by_level = BTreeMap::new();
    for ty in OntologyType::ALL {
        let count = result.graph.nodes_by_type(ty).count();
        if count > 0 {
            by_level.insert(format!("{:02}_{}", ty.level(), ty.slug()), count);
        }
    }

    let summary = serde_json::json!({
        "ontology": ONTOLOGY_VERSION,
        "workspace": cli.workspace,
        "read_only": true,
        "mode": "syntax-projection",
        "nodes": result.graph.len(),
        "edges": result.graph.edge_len(),
        "nodes_by_level": by_level,
        "observations": result.observations
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&summary).expect("summary is serializable")
    );
}
