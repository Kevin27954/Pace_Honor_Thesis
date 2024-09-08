use std::{env, fs, process};

use compiler::chunk::Chunk;
use vm::{InterpretError, VM};

mod compiler;
mod debug;
mod expr_prec;
mod scanner;
mod test;
mod vm;

fn main() {
    if env::args().len() > 4 {
        println!("You entered too many arguments");
        process::exit(1);
    } else {
        let args: Vec<String> = env::args().collect();
        let cmd = &args[1];

        match cmd.as_str() {
            "run" => {
                read_file(&args[2]);
            }
            "sparkling" => {
                todo!("Should start the ASCII adventure");
            }
            _ => {
                println!("Unknown command");
                process::exit(1);
            }
        }
    }
}

fn read_file(path: &String) {
    let source_str =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Error Reading File. Path: {}", path));

    let source_str = String::from("\"str1🔥\" == \"str1🔥\"");

    let mut vm = VM::new(Chunk::new());
    match vm.interpret(source_str) {
        Ok(_) => {}
        Err(err) => match err {
            InterpretError::CompileError => process::exit(65),
            InterpretError::RuntimeError => process::exit(70),
        },
    }
}
