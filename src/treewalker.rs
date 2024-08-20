use std::{fs, process};

use ast_printer::print_ast;
use expr_types::{Expr, Primary};
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use statements::Stmt;

mod ast_printer;
mod errors;
mod expr_types;
mod interpreter;
mod parser;
mod runtime_env;
mod runtime_types;
mod scanner;
mod statements;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        errors::error(0, format!("Unable to read file {}", path));
        String::new()
    });

    let (has_error, exit_num) = run(&buffer);
    //let (has_error, exit_num) = run(&String::from("let temp = 123"));
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
    let (stmt, has_error) = parser.parse();
    for stmt in &stmt {
        println!("{}", print_ast(stmt));
    }
    if has_error {
        return (has_error, 65);
    }

    println!("\nInterpreter:");
    let mut interpreter = Interpreter::new();
    let has_error: bool = false;
    for stmt in &stmt {
        interpreter.interpret(stmt);
    }

    println!("\nAll Variables Delcared (For development purposes):");
    for (var, val) in interpreter.get_runtime_env().return_runtime_env() {
        println!("{} = {}", var, val.err_format());
    }

    if has_error {
        return (has_error, 70);
    }

    return (false, 0);
}
