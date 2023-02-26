//! executable code for a clap / cli learning tutorial

use clap::Parser;
use clap_acq::CliArgs;

fn main() {
    let args = CliArgs::parse();

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());
}
