//! executable code for a clap / cli learning tutorial
//! # Example Run Code:
//! ```bash
//! RUST_LOG=info cargo run --quiet -- real ./files/input.txt
//! ```

use std::{fs::File,
          io::{BufRead, BufReader}};

use anyhow::{Context, Result};
use clap::Parser;
use clap_acq::CliArgs; //`clap_acq` is the name of this crate('s library)
use log::info;

fn main() -> Result<()> {
        env_logger::init();
        info!("Starting up");

        let args = CliArgs::parse();
        let file = File::open(&args.path).with_context(|| format!("could not open file: {}", args.path.display()))?;
        let reader = BufReader::new(file);

        info!("Pattern: {}", args.pattern);
        info!("Path: {}", args.path.display());
        println!("Pattern: {}", args.pattern);
        println!("Path: {}", args.path.display());

        println!("\nSearching for matches...:");
        let mut matchfound = false;
        for line in reader.lines() {
                if line.as_ref().unwrap().contains(&args.pattern) {
                        matchfound = true;
                        println!("{:?}", line.expect("could not unwrap 'line', despite containg a value"));
                }
        }
        if !matchfound {
                println!("No match found");
        }

        // // looking at ProgressBar
        // let pb = indicatif::ProgressBar::new(100);
        // for i in 0..100 {
        //     std::thread::sleep(std::time::Duration::from_millis(100));
        //     pb.set_position(i);
        //     pb.println(format!("[+] finished #{}", i));
        //     pb.inc(1);
        // }

        // // timing results are not what I expected
        // // (Granted, this is not best practice benchmarking I imagine)
        // let timings = clap_acq::printtiming::compare().unwrap();
        // println!("timings: {:?}", timings);

        Ok(())
}
