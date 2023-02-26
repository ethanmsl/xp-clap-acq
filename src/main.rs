//! executable code for a clap / cli learning tutorial

use clap::Parser;
use clap_acq::CliArgs;

fn main() {
    let args = CliArgs::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read string");

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());

    let mut matchfound = false;
    println!("\nSearching for matches...:");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            matchfound = true;
            println!("{}", line);
        }
    }
    if !matchfound {
        println!("No match found");
    }
}
