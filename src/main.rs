// src/main.rs
/*
 * Main executable for LegaltechsolutionsPlatform
 */

use clap::Parser;
use legaltechsolutionsplatform::{Result, run};

#[derive(Parser)]
#[command(version, about = LegaltechsolutionsPlatform - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
