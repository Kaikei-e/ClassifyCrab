use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[clap("Kaikei-e", "0.0.1", "The classifier of zip files.", long_about = None)]
    struct Args {
        #[clap(short, long)]
        verbose: bool,

        #[clap(flatten)]
        will_proceed:
    }

    fn main() {
        let args = Args::parse();

        for _ in 0..args.count {
            println!("Hello {}!", args.name)
        }
    }
}
