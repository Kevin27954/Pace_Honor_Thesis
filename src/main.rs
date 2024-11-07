use std::{env, fs, process};

use biteling::{current_stage, get_info, start_file_listener, start_user_input};
use vm::{InterpretError, VM};

mod biteling;
mod compiler;
mod debug;
mod expr_prec;
mod native_functions;
mod scanner;
mod test;
mod vm;

fn main() {
    if env::args().len() > 4 {
        eprintln!("You entered too many arguments");
        process::exit(1);
    } else {
        let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            eprintln!(
                "\
Unknown command. Usage:
<placeholder> run <file name>.txt
<placeholder> learn"
            );
            process::exit(1);
        }

        let cmd = &args[1];

        match cmd.as_str() {
            "run" => {
                read_file(&args[2]);
            }
            "learn" => {
                let mut stages = get_info();
                let user_input_rx = start_user_input();
                let curr_stage = current_stage(&mut stages);
                if curr_stage >= stages.len() {
                    println!("You finished");
                } else {
                    let handler = start_file_listener(user_input_rx, stages, curr_stage);
                    // Wait for thread to finish.
                    let _ = handler.join();
                }

                println!("Goodbye!");
            }
            _ => {
                eprintln!(
                    "\
Unknown command. Usage:
<placeholder> run <file name>.txt
<placeholder> learn"
                );
                process::exit(1);
            }
        }
    }
}

fn read_file(path: &String) {
    let source_str =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Error Reading File. Path: {}", path));

    //let source_str = String::from("\"str1🔥\" == \"str2🔥\"\n1 + 1");

    let mut vm = VM::new();
    match vm.interpret(source_str) {
        Ok(_) => {}
        Err(err) => match err {
            InterpretError::CompileError => {
                println!("i ran");
                process::exit(65);
            }
            InterpretError::RuntimeError => process::exit(70),
        },
    }
}
