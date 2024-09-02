use std::{env, fs, path::Path, process};

use compiler::{
    chunk::{Chunk, OpCode},
    values::Value,
};
use debug::disassemble_chunk;
use scanner::Scanner;
use vm::{InterpretResult, VM};

mod compiler;
mod debug;
mod scanner;
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
    let buffer =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Error Reading File. Path: {}", path));

    let mut scanner = Scanner::new(String::from("end"));
    println!("{}", scanner.scan_token());
    let result: InterpretResult = InterpretResult::OK;

    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        _ => {}
    }
}
