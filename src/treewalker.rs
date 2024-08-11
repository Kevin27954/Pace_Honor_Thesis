use std::{fs, process};

use scanner::Scanner;

mod errors;
mod scanner;
mod token;
mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        errors::error(0, format!("Unable to read file {}", path));
        String::new()
    });

    let has_error = run(buffer);
    if has_error {
        process::exit(1);
    }
}

pub fn run(source: String) -> bool {
    let mut scanner = Scanner::new(source);
    let has_error = scanner.scan();
    println!("{}", has_error);

    for token in scanner.get_tokens() {
        println!("{}", token);
    }

    return has_error;
}
