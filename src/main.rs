use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use classifyCrab::run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Inspect { file_number }) => {

        }
        None => {}
    }

    run()
}
