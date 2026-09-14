use clap::{Parser, Subcommand};
use ontology_core::{OntologyType, ONTOLOGY_VERSION};
use ontology_discovery::{discover_workspace, DiscoveryOptions};
use ontology_language::{parse as parse_language, Language};
use ontology_registry::OntologyRegistry;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const DEFAULT_REGISTRY: &str = "specifications/universal-ontology-v1.0.json";
const SELF_IGNORED_DIRS: &[&str] = &[
    ".git",
    ".next",
    ".turbo",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
    "__pycache__",
    ".venv",
    "venv",
];

#[derive(Parser, Debug)]
#[command(name = "ontology-engine", version = ONTOLOGY_VERSION)]
struct Cli {
    #[arg(long, global = true, default_value = DEFAULT_REGISTRY)]
    registry: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Levels,
    Inspect {
        level: u8,
    },
    Validate,
    Discover {
        workspace: PathBuf,
        #[arg(long)]
        include_files: bool,
        #[arg(long)]
        rust_ast: bool,
        #[arg(long)]
        max_depth: Option<usize>,
    },
    Parse {
        language: String,
        file: PathBuf,
    },
    /// Inspect this repository before analyzing external repositories.
    #[command(name = "self")]
    SelfCheck {
        /// Repository root to inspect. Defaults to the current directory.
        #[arg(default_value = ".")]
        root: PathBuf,
        /// Emit a machine-readable JSON snapshot.
        #[arg(long)]
        json: bool,
    },
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

    match cli.command {
        Command::Levels => {
            for ty in registry.all() {
                let definition = registry
                    .definition(*ty)
                    .expect("registry is internally consistent");
                println!(
                    "{:02} Z{} {} — {}",
                    ty.level(),
                    ty.zone(),
                    ty,
                    definition.definition
                );
            }
        }
        Command::Inspect { level } => match registry.level(level) {
            Some(ty) => {
                let definition = registry
                    .definition(ty)
                    .expect("registry is internally consistent");
                println!("level={} zone={} type={}", level, ty.zone(), ty);
                println!("definition={}", definition.definition);
            }
            None => {
                eprintln!("invalid level: {level}; expected 1..=49");
                std::process::exit(2);
            }
        },
        Command::Validate => println!(
            "valid: ontology={} levels={} source={}",
            ONTOLOGY_VERSION,
            registry.len(),
            cli.registry.display()
        ),
        Command::Discover {
            workspace,
            include_files,
            rust_ast,
            max_depth,
        } => {
            let result = match discover_workspace(
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
                "observations": result.observations
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&summary).expect("summary is serializable")
            );
        }
        Command::Parse { language, file } => {
            let language = match Language::parse_name(&language) {
                Some(value) => value,
                None => {
                    eprintln!("unsupported language: {language}");
                    std::process::exit(2);
                }
            };
            let source = match fs::read_to_string(&file) {
                Ok(source) => source,
                Err(error) => {
                    eprintln!("failed to read `{}`: {error}", file.display());
                    std::process::exit(2);
                }
            };
            let observations =
                parse_language(language, &source).expect("selected language is supported");
            let output = serde_json::json!({
                "ontology": ONTOLOGY_VERSION,
                "language": language.slug(),
                "file": file,
                "adapter_mode": "deterministic_syntax_observation",
                "observations": observations
            });
            println!(
                "{}",
                serde_json::to_string_pretty(&output).expect("output is serializable")
            );
        }
        Command::SelfCheck { root, json } => {
            let snapshot = match build_self_snapshot(&root, &registry, &cli.registry) {
                Ok(snapshot) => snapshot,
                Err(error) => {
                    eprintln!("self inspection failed: {error}");
                    std::process::exit(2);
                }
            };

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&snapshot).expect("snapshot is serializable")
                );
            } else {
                println!("UNIVERSAL ONTOLOGY ENGINE — SELF INSPECTION");
                println!("repository       {}", snapshot["repository"].as_str().unwrap_or("unknown"));
                println!("ontology         {}", snapshot["ontology"].as_str().unwrap_or("unknown"));
                println!("canonical levels {}", snapshot["canonical_levels"]);
                println!("workspace crates {}", snapshot["workspace_crates"]);
                println!("rust sources     {}", snapshot["rust_sources"]);
                println!("cargo manifest   {}", snapshot["checks"]["cargo_manifest"]);
                println!("registry         {}", snapshot["checks"]["registry"]);
                println!("status           {}", snapshot["status"].as_str().unwrap_or("unknown").to_uppercase());
            }

            if snapshot["status"] != "healthy" {
                std::process::exit(1);
            }
        }
    }
}

fn build_self_snapshot(
    root: &Path,
    registry: &OntologyRegistry,
    registry_path: &Path,
) -> Result<serde_json::Value, std::io::Error> {
    let cargo_manifest = root.join("Cargo.toml").is_file();
    let workspace_crates = count_workspace_crates(root)?;
    let rust_sources = count_rust_sources(root)?;
    let repository = root
        .canonicalize()
        .ok()
        .as_deref()
        .and_then(Path::file_name)
        .and_then(|value| value.to_str())
        .unwrap_or("repository")
        .to_owned();
    let registry_valid = registry.len() == OntologyType::ALL.len();
    let healthy = cargo_manifest && registry_valid && workspace_crates > 0 && rust_sources > 0;

    Ok(serde_json::json!({
        "schema": "universal-ontology-engine/self-v1",
        "self": true,
        "repository": repository,
        "ontology": ONTOLOGY_VERSION,
        "canonical_levels": registry.len(),
        "workspace_crates": workspace_crates,
        "rust_sources": rust_sources,
        "read_only": true,
        "status": if healthy { "healthy" } else { "degraded" },
        "checks": {
            "cargo_manifest": if cargo_manifest { "pass" } else { "fail" },
            "registry": if registry_valid { "pass" } else { "fail" },
            "workspace_crates": if workspace_crates > 0 { "pass" } else { "fail" },
            "rust_sources": if rust_sources > 0 { "pass" } else { "fail" }
        },
        "sources": {
            "registry": registry_path,
            "workspace": "Cargo.toml",
            "crates": "crates/"
        }
    }))
}

fn count_workspace_crates(root: &Path) -> Result<usize, std::io::Error> {
    let crates_dir = root.join("crates");
    if !crates_dir.is_dir() {
        return Ok(0);
    }

    let mut count = 0;
    for entry in fs::read_dir(crates_dir)? {
        let path = entry?.path();
        if path.is_dir() && path.join("Cargo.toml").is_file() {
            count += 1;
        }
    }
    Ok(count)
}

fn count_rust_sources(root: &Path) -> Result<usize, std::io::Error> {
    fn walk(path: &Path) -> Result<usize, std::io::Error> {
        let mut count = 0;
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("");
                if SELF_IGNORED_DIRS.contains(&name) {
                    continue;
                }
                count += walk(&path)?;
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                count += 1;
            }
        }
        Ok(count)
    }

    walk(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn self_inventory_counts_workspace_crates_and_rust_sources() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("ontology-self-{stamp}"));
        fs::create_dir_all(root.join("crates/example/src")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        fs::write(root.join("crates/example/Cargo.toml"), "[package]\nname='example'\nversion='0.1.0'\n").unwrap();
        fs::write(root.join("crates/example/src/lib.rs"), "pub fn demo() {}\n").unwrap();
        fs::create_dir_all(root.join("target/generated")).unwrap();
        fs::write(root.join("target/generated/ignored.rs"), "fn ignored() {}\n").unwrap();

        assert_eq!(count_workspace_crates(&root).unwrap(), 1);
        assert_eq!(count_rust_sources(&root).unwrap(), 1);

        fs::remove_dir_all(root).unwrap();
    }
}
