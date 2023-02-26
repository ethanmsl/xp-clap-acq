//! executable code for a clap / cli learning tutorial

use anyhow::{Context, Result};
use clap::Parser;
use clap_acq::CliArgs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file: {}", args.path.display()))?;
    let reader = BufReader::new(file);

    println!("Pattern: {}", args.pattern);
    println!("Path: {}", args.path.display());

    println!("\nSearching for matches...:");
    let mut matchfound = false;
    for line in reader.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            matchfound = true;
            println!("{:?}", line);
        }
    }
    if !matchfound {
        println!("No match found");
    }

    // // timing results are not what I expected
    // // (Granted, this is not best practice benchmarking I imagine)
    // let timings = clap_acq::compare().unwrap();
    // println!("timings: {:?}", timings);

    Ok(())
}
