use clap::Parser;

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
pub fn writebuf_example() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "using writebuffer: {}", 42)?;
    Ok(())
}
pub fn getlock_example() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "got lock manually: {}", 42)?;
    Ok(())
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
