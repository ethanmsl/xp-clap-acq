use clap::Parser;

#[derive(Parser)]
/// CLI Argument Struct
/// Automatically derived into parsing structs by clap
pub struct CliArgs {
    ///Pattern to look for
    pub pattern: String,
    ///path to file; for reading
    pub path: std::path::PathBuf,
}

/////////////////////////////////////
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
