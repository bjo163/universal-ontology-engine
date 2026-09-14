mod syntax_projection;

mod legacy {
    include!("main.rs");

    pub(super) fn run() {
        main();
    }
}

use clap::{Parser, Subcommand};
use ontology_core::{OntologyType, ONTOLOGY_VERSION};
use ontology_discovery::{discover_workspace, DiscoveryOptions};
use ontology_registry::OntologyRegistry;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use syntax_projection::project_workspace_syntax;

const DEFAULT_REGISTRY: &str = "specifications/universal-ontology-v1.0.json";

#[derive(Parser, Debug)]
#[command(name = "ontology-engine", version = ONTOLOGY_VERSION)]
struct SyntaxCli {
    #[arg(long, global = true, default_value = DEFAULT_REGISTRY)]
    registry: PathBuf,
    #[command(subcommand)]
    command: SyntaxCommand,
}

#[derive(Subcommand, Debug)]
enum SyntaxCommand {
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
    if std::env::args().any(|argument| argument == "--syntax") {
        run_syntax_discover();
    } else {
        legacy::run();
    }
}

fn run_syntax_discover() {
    let cli = SyntaxCli::parse();
    let SyntaxCommand::Discover {
        workspace,
        include_files,
        rust_ast,
        syntax,
        max_depth,
    } = cli.command;

    debug_assert!(syntax);
    debug_assert!(!rust_ast);

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
        &workspace,
        registry,
        DiscoveryOptions {
            include_files,
            max_depth,
            parse_rust_ast: false,
        },
    ) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("discovery failed: {error}");
            std::process::exit(2);
        }
    };

    if let Err(error) = project_workspace_syntax(&mut result, &workspace) {
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
        "workspace": workspace,
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
