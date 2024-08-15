use std::{fs, process};

use ast_printer::print_ast;
use parser::Parser;
use scanner::Scanner;

mod ast_printer;
mod errors;
mod expr_types;
mod parser;
mod scanner;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        errors::error(0, format!("Unable to read file {}", path));
        String::new()
    });

    //let has_error = run(&buffer);
    let has_error = run(&String::from("hello"));
    if has_error {
        process::exit(1);
    }
}

pub fn run(source: &String) -> bool {
    let mut scanner = Scanner::new(source);
    let (tokens, has_error) = scanner.scan();

    for token in &tokens {
        println!("{}", token);
    }

    let mut parser = Parser::new(&tokens);
    let exprs = parser.parse();
    for expr in exprs {
        println!("{}", print_ast(&expr));
        //println!("{:?}", expr);
    }

    return has_error;
}
