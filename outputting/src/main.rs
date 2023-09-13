use log::{info, warn};


// for the --verbose argument
use clap::Parser;
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let x = 42;
    println!("what is x???? probably like {} or something", x);

    // logger :3
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
