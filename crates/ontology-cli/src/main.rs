use clap::{Parser, Subcommand};
use ontology_core::{OntologyType, ONTOLOGY_VERSION};
use ontology_discovery::{discover_workspace, DiscoveryOptions};
use ontology_registry::OntologyRegistry;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_REGISTRY: &str = "specifications/universal-ontology-v1.0.json";

#[derive(Parser, Debug)]
#[command(name = "ontology-engine", version = ONTOLOGY_VERSION)]
struct Cli {
    /// Path to the canonical ontology registry JSON.
    #[arg(long, global = true, default_value = DEFAULT_REGISTRY)]
    registry: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print all canonical ontology levels from the registry.
    Levels,
    /// Inspect one canonical level (1..=49).
    Inspect { level: u8 },
    /// Validate that the registry is compatible with this engine.
    Validate,
    /// Discover a workspace read-only and emit the observed graph summary as JSON.
    Discover {
        /// Workspace root containing ecosystem-* directories.
        workspace: PathBuf,
        /// Include observed file-boundary execution nodes.
        #[arg(long)]
        include_files: bool,
        /// Parse Rust source files through the AST adapter.
        #[arg(long)]
        rust_ast: bool,
        /// Maximum nested native directory depth under each unit.
        #[arg(long)]
        max_depth: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();
    let registry = match OntologyRegistry::load(&cli.registry) {
        Ok(registry) => Arc::new(registry),
        Err(error) => {
            eprintln!("failed to load ontology registry `{}`: {error}", cli.registry.display());
            std::process::exit(2);
        }
    };

    match cli.command {
        Command::Levels => {
            for ty in registry.all() {
                let definition = registry.definition(*ty).expect("registry is internally consistent");
                println!("{:02} Z{} {} — {}", ty.level(), ty.zone(), ty, definition.definition);
            }
        }
        Command::Inspect { level } => match registry.level(level) {
            Some(ty) => {
                let definition = registry.definition(ty).expect("registry is internally consistent");
                println!("level={} zone={} type={}", level, ty.zone(), ty);
                println!("definition={}", definition.definition);
            }
            None => {
                eprintln!("invalid level: {level}; expected 1..=49");
                std::process::exit(2);
            }
        },
        Command::Validate => {
            println!("valid: ontology={} levels={} source={}", ONTOLOGY_VERSION, registry.len(), cli.registry.display());
        }
        Command::Discover { workspace, include_files, rust_ast, max_depth } => {
            let result = match discover_workspace(
                &workspace,
                registry,
                DiscoveryOptions { include_files, max_depth, parse_rust_ast: rust_ast },
            ) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("discovery failed: {error}");
                    std::process::exit(2);
                }
            };

            let mut by_level = BTreeMap::new();
            for ty in OntologyType::ALL {
                let count = result.graph.nodes_by_type(ty).count();
                if count > 0 {
                    by_level.insert(format!("{:02}_{}", ty.level(), ty.slug()), count);
                }
            }
            let summary = serde_json::json!({
                "ontology": ONTOLOGY_VERSION,
                "workspace": workspace,
                "read_only": true,
                "nodes": result.graph.len(),
                "edges": result.graph.edge_len(),
                "nodes_by_level": by_level,
                "observations": result.observations,
            });
            println!("{}", serde_json::to_string_pretty(&summary).expect("summary is serializable"));
        }
    }
}
