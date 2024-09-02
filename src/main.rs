use std::{fs::File, process::exit};

use args::Cli;
use clap::Parser;
use jcc::lexer::Lexer;

mod args;
mod jcc;

fn main() {
    let args = Cli::parse();

    let file = File::open(args.input).expect("unable to open file");

    for token in Lexer::new(file).into_iter() {
        match token {
            Ok(t) => {
                println!("{:?}", t);
            }
            Err(err) => {
                println!("jcc: error: {}", err);
                exit(1);
            }
        }
    }
}
