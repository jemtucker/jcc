use std::{fs::File, process::exit};

use args::Cli;
use clap::Parser;
use jcc::{lexer::Lexer, parser};

mod args;
mod jcc;

fn main() {
    let args = Cli::parse();

    let file = File::open(args.input).expect("unable to open file");
    let tokens = Lexer::new(file).into_iter();

    if args.lex {
        for token in tokens {
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

        return;
    }

    let ast = parser::Parser::new(tokens).parse();
    match ast {
        Ok(ast) => {
            println!("{:?}", ast);
        }
        Err(err) => {
            println!("jcc: error: {}", err);
            exit(1);
        }
    }
}
