use std::{fs, process};

mod errors;
mod token;
pub mod token_types;

pub fn run_file(path: &String) {
    let buffer = fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Unable to read file {}", path);
        String::new()
    });

    let has_error = run(buffer);
    if has_error {
        process::exit(1);
    }
}

pub fn run(source: String) -> bool {
    println!("{}", source);

    false
}
