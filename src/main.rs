//! executable code for a clap / cli learning tutorial

use clap::Parser;

#[derive(Parser)]
struct CLI {
    ///Pattern to look for
    pattern: String,
    ///path to file; for reading
    path: std::path::PathBuf,
}

fn main() {

    let args = CLI::parse();

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());
}
