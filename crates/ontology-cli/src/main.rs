use clap::{Parser, Subcommand};
use ontology_core::ONTOLOGY_VERSION;
use ontology_registry::OntologyRegistry;
use std::path::PathBuf;

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
}

fn main() {
    let cli = Cli::parse();
    let registry = match OntologyRegistry::load(&cli.registry) {
        Ok(registry) => registry,
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
    }
}
