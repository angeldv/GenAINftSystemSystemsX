// src/main.rs
/*
 * Main executable for GenAINftSystemSystemsX
 */

use clap::Parser;
use genainftsystemsystemsx::{Result, run};

#[derive(Parser)]
#[command(version, about = "GenAINftSystemSystemsX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
