//! executable code for a clap / cli learning tutorial

use clap::Parser;
use clap_acq::CliArgs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = CliArgs::parse();
    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);
    // let content = std::fs::read_to_string(&args.path).expect("could not read string");
    // let what = reader.lines();

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());

    let mut matchfound = false;

    println!("\nSearching for matches...:");
    for line in reader.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            matchfound = true;
            println!("{:?}", line);
        }
    }
    if !matchfound {
        println!("No match found");
    }
}
