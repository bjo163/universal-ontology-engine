use clap::{Parser, Subcommand};
use ontology_core::{canonical_path, OntologyType, ONTOLOGY_VERSION};

#[derive(Parser, Debug)]
#[command(name = "ontology-engine", version = ONTOLOGY_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Levels,
    Inspect { level: u8 },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Levels => {
            for ty in canonical_path() {
                println!("{:02} Z{} {}", ty.level(), ty.zone(), ty);
            }
        }
        Command::Inspect { level } => {
            match OntologyType::ALL.get(level.saturating_sub(1) as usize) {
                Some(ty) if ty.level() == level => {
                    println!("level={} zone={} type={}", level, ty.zone(), ty)
                }
                _ => {
                    eprintln!("invalid level: {level}; expected 1..=49");
                    std::process::exit(2);
                }
            }
        }
    }
}
