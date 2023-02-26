use clap::Parser;
use std::time::Instant;

////////////////////////////////////
// Arguments Object
//

#[derive(Parser)]
/// CLI Argument Struct
/// Automatically derived into parsing structs by clap
pub struct CliArgs {
    ///Pattern to look for
    pub pattern: String,
    ///path to file; for reading
    pub path: std::path::PathBuf,
}

////////////////////////////////////
// Fast writing approaches
//

use std::io::{self, Write};
fn writebuf_example() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "foo: {}", 42)?;
    Ok(())
}
fn getlock_example() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42)?;
    Ok(())
}

/// Comparing writebuffer writing, manual lock acquisition, and raw println'ing
/// nominally the first two are faster,
/// but the first actually appears to be slightly slower
/// and all appear to be very similar
/// ...
/// Granted, this isn't formal benchmarking and perhaps when I take times in the code
/// doesn't apply as clearly as I'd think due to compilation & optimization
/// , still, I'm surprised.
/// Something to perhaps look at another time with better benchmarking
pub fn compare() -> anyhow::Result<(u32, u32, u32)> {
    let now = Instant::now();
    for _ in 0..1000000 {
        writebuf_example()?;
    }
    let new_now = Instant::now();
    let timing_1 = new_now.duration_since(now).as_millis();

    let now = Instant::now();
    for _ in 0..1000000 {
        getlock_example()?;
    }
    let new_now = Instant::now();
    let timing_2 = new_now.duration_since(now).as_millis();

    let now = Instant::now();
    for _ in 0..1000000 {
        println!("foo: {}", 42);
    }
    let new_now = Instant::now();
    let timing_3 = new_now.duration_since(now).as_millis();

    Ok((timing_1 as u32, timing_2 as u32, timing_3 as u32))
}

/////////////////////////////////////
// example code
//

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
