use std::{fs::File, process::exit};

use args::Cli;
use clap::Parser;
use jcc::{lexer::Lexer, parser, CodeGenerator};
use std::io::Write;

mod args;
mod jcc;

fn main() {
    let mut args = Cli::parse();

    if args.lex {
        return lex(&args);
    }

    if args.parse {
        return parse(&args);
    }

    if args.codegen {
        return codegen(&args);
    }

    return assemble(&mut args);
}

fn lex(args: &Cli) {
    let file = File::open(&args.input).expect("unable to open file");

    for token in Lexer::new(file).into_iter() {
        match token {
            Ok(t) => {
                eprintln!("{:?}", t);
            }
            Err(err) => {
                eprintln!("jcc: error: {}", err);
                exit(1);
            }
        }
    }
}

fn parse(args: &Cli) {
    let file = File::open(&args.input).expect("unable to open file");
    let tokens = Lexer::new(file).into_iter();
    let ast = parser::Parser::new(tokens).parse();

    match ast {
        Ok(ast) => {
            eprintln!("{:?}", ast);
        }
        Err(err) => {
            eprintln!("jcc: error: {}", err);
            exit(1);
        }
    }
}

fn codegen(args: &Cli) {
    let file = File::open(&args.input).expect("unable to open file");
    let tokens = Lexer::new(file).into_iter();
    let ast = parser::Parser::new(tokens).parse();

    match ast {
        Ok(ast) => {
            eprintln!("{}", CodeGenerator::new().codegen(ast));
        }
        Err(err) => {
            eprintln!("jcc: error: {}", err);
            exit(1);
        }
    }
}

fn assemble(args: &mut Cli) {
    let file = File::open(&args.input).expect("unable to open file");
    let tokens = Lexer::new(file).into_iter();
    let ast = parser::Parser::new(tokens).parse();

    match ast {
        Ok(ast) => {
            let mut out = File::create(args.output()).unwrap();
            write!(out, "{}", CodeGenerator::new().codegen(ast)).unwrap();
        }
        Err(err) => {
            eprintln!("jcc: error: {}", err);
            exit(1);
        }
    }
}
