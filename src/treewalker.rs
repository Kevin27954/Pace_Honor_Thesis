use std::{fs, process};

use ast_printer::ASTPrinter;
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;

mod ast_printer;
mod errors;
mod expr_types;
mod functions;
mod interpreter;
mod parser;
mod resolver;
mod runtime_env;
mod runtime_types;
mod scanner;
mod statements;
mod structs;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        errors::error(0, format!("Unable to read file {}", path));
        String::new()
    });

    let (has_error, exit_num) = run(&buffer);
    //let (has_error, exit_num) = run(&String::from("do\nlet a = 123\na= 33\nend"));
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

    let mut ast_printer = ASTPrinter::new();
    println!("\nParser:");
    let mut parser = Parser::new(&tokens);
    let (stmt, has_error) = parser.parse();
    for stmt in &stmt {
        println!("{}", ast_printer.print_ast(stmt));
    }
    if has_error {
        return (has_error, 65);
    }
    println!();

    // No need to pass in because we will evaluate line by line and error out
    // at the point there is an error, rather than push all error to the top.
    let mut interpreter = Interpreter::new();
    let mut resolver = Resolver::new(&mut interpreter);
    let has_error = resolver.resolve(&stmt);

    if has_error {
        return (has_error, 65);
    }

    println!("\nInterpreter:");
    let has_error: bool = false;
    for stmt in &stmt {
        interpreter.interpret(stmt);
    }

    println!("\nAll Variables Delcared (For development purposes):");
    let envs = interpreter.get_runtime_env().return_runtime_env();
    let mut envs = envs.iter();
    while let Some(env) = envs.next() {
        for (var, val) in env {
            println!("{} = {}", var, val.err_format());
        }
    }

    if has_error {
        return (has_error, 70);
    }

    return (false, 0);
}
