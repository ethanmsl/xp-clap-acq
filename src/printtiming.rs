//!
//! **Command Line Applications in Rust** book
//! in "[A note on printing performance](https://rust-cli.github.io/book/tutorial/output.html#a-note-on-printing-performance)"
//! mentions that writing to terminal can be quite slow
//! here we implement some of their nominally faster examples
//! and do a crude, interpreted style, timing check on perf
//! the results show one of the nominally faster examples as the slowest
//! and all three as quite close
//!
//! I don't have a clear interpretations of this.
//! But it bares possible investigation in the future.
//! (And testing with more 'proper' benchmarking tools)
//!
//! NOTE: the writebuffer approach is more notably slower when
//! not run with `--release`

////////////////////////////////////
// Fast writing approaches
//

use std::io::{self, Write};
use std::time::Instant;

fn writebuf_example(num: i32) -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "foo: {}", num)?;
    Ok(())
}
fn getlock_example(num: i32) -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", num)?;
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
    for i in 0..1000000 {
        writebuf_example(i)?;
    }
    let new_now = Instant::now();
    let timing_1 = new_now.duration_since(now).as_millis();

    let now = Instant::now();
    for i in 0..1000000 {
        getlock_example(i)?;
    }
    let new_now = Instant::now();
    let timing_2 = new_now.duration_since(now).as_millis();

    let now = Instant::now();
    for i in 0..1000000 {
        println!("foo: {}", i);
    }
    let new_now = Instant::now();
    let timing_3 = new_now.duration_since(now).as_millis();

    Ok((timing_1 as u32, timing_2 as u32, timing_3 as u32))
}
