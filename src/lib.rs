use clap::Parser;
pub mod printtiming;

////////////////////////////////////
// Arguments Object
//

#[derive(Parser)]
// CLI Argument Struct
// Automatically derived into parsing structs by clap
/// Takes a given literal pattern and searches a given file location for it.
pub struct CliArgs {
    ///Pattern to look for
    pub pattern: String,
    ///path to file; for reading
    pub path: std::path::PathBuf,
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
