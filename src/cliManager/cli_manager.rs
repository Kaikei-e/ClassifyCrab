use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[clap(flatten)]
    will_proceed: WillProceed,
}

