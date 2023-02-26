//! executable code for a clap / cli learning tutorial

use clap::Parser;

#[derive(Parser)]
/// CLI Argument Struct\n
/// Automatically derived into parsing structs by clap
struct Cli {
    ///Pattern to look for
    pattern: String,
    ///path to file; for reading
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());
}
