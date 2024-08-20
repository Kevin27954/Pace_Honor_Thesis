use std::{fs, process};

use ast_printer::print_ast;
use interpreter::interpret;
use parser::Parser;
use scanner::Scanner;

mod ast_printer;
mod errors;
mod expr_types;
mod interpreter;
mod parser;
mod runtime_types;
mod scanner;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        errors::error(0, format!("Unable to read file {}", path));
        String::new()
    });

    let (has_error, exit_num) = run(&buffer);
    //let has_error = run(&String::from("("));
    if has_error {
        process::exit(exit_num);
    }
}

pub fn run(source: &String) -> (bool, i32) {
    let mut scanner = Scanner::new(source);
    let (tokens, has_error) = scanner.scan();

    println!("Scanner:");
    for token in &tokens {
        println!("{}", token);
    }
    if has_error {
        return (has_error, 65);
    }

    println!("\nParser:");
    let mut parser = Parser::new(&tokens);
    let (exprs, has_error) = parser.parse();
    for expr in &exprs {
        println!("{}", print_ast(expr));
    }
    if has_error {
        return (has_error, 65);
    }

    println!("\nInterpreter:");
    let mut has_error: bool = false;
    for expr in &exprs {
        has_error = interpret(expr);
    }
    if has_error {
        return (has_error, 70);
    }

    return (false, 0);
}
