mod subcommands;
mod utils;

use clap::{Parser, Subcommand};
use subcommands::{Alias, AliasCommand};
use utils::{ load_from_file, save_to_file, RealFileReader};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Alias usage")]
    Alias {
        #[clap(subcommand)]
        subcommand: Alias,
    },
}

fn main() {
    let cli = Cli::parse();
    const FILE_PATH: &str = "config.toml";
    let toml_config = load_from_file(&RealFileReader, FILE_PATH.to_string());
    match &cli.command {
        Commands::Alias { subcommand } => {
            let mut command = AliasCommand::new(subcommand.clone(), toml_config.unwrap());
            command.run();
            save_to_file(&RealFileReader, FILE_PATH.to_string(), &command.toml_config).unwrap();
        }
    }
}
