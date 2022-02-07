use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap("Kaikei-e", "0.0.1", "The classifier of zip files.", long_about = None)]
struct Cli {
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Inspect {
        #[clap(short, long)]
        file_number: Vec<PathBuf>,
    },

    Target {
        #[clap(short, long)]
        file_path: Path,
    },

    Classify {
        #[clap(short, long)]
        classify: Classify(),
    },
}

fn main() {
    let cli = Cli::parse;

    match &cli.command {
        Some(Commands::Inspect { file_number }) => {}
        None => {}
    }
}
