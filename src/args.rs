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
