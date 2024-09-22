use std::path::Path;

use clap::Parser;

/// A C compiler
#[derive(Parser)]
pub struct Cli {
    /// The path to the file to compile
    pub input: String,

    /// The output file location
    #[arg(short = 'o', long = "output", required = false)]
    pub output: String,

    /// Stop after lexing stage
    #[arg(long = "lex", required = false)]
    pub lex: bool,

    /// Stop after parsing stage
    #[arg(long = "parse", required = false)]
    pub parse: bool,

    /// Stop after parsing stage
    #[arg(long = "codegen", required = false)]
    pub codegen: bool,
}

impl Cli {
    pub fn output(&mut self) -> &str {
        if self.output == "" {
            let path = Path::new(&self.input).with_extension("s");
            self.output = path.to_str().unwrap().to_owned();
        }

        &self.output
    }
}
