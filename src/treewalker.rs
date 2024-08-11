use std::{fs, process};

use scanner::Scanner;
use token::Token;

mod errors;
mod scanner;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Unable to read file {}", path);
        String::new()
    });

    let has_error = run(buffer);
    if has_error {
        process::exit(1);
    }
}

pub fn run(source: String) -> bool {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    for token in tokens {
        println!("{}", token);
    }

    return false;
}
