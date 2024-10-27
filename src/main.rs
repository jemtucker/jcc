use std::{fs::File, process::exit};

use args::Cli;
use clap::Parser;
use jcc::{asm, lexer::Lexer, parser};
use std::io::Write;

mod args;
mod jcc;

fn main() {
    let mut args = Cli::parse();

    let file = File::open(&args.input).expect("unable to open file");
    let tokens = Lexer::new(file).into_iter();

    if args.lex {
        for token in tokens {
            match token {
                Ok(t) => {
                    println!("{:?}", t);
                }
                Err(err) => {
                    eprintln!("jcc: error: {}", err);
                    exit(1);
                }
            }
        }

        return;
    }

    let ast = parser::Parser::new(tokens).parse();

    match ast {
        Ok(ast) => {
            if args.parse {
                println!("{:?}", ast);
                return;
            }

            let asm: asm::Program = ast.into();
            let mut out = File::create(args.output()).unwrap();

            write!(out, "{}", asm).unwrap();
        }
        Err(err) => {
            eprintln!("jcc: error: {}", err);
            exit(1);
        }
    }
}
