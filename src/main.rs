use std::{
    env, fs, process,
    sync::{Arc, RwLock},
};

use biteling::{current_stage, start_file_listener, start_user_input};
use stage_problems::StageInfo;
use vm::{InterpretError, VM};

mod biteling;
mod compiler;
mod debug;
mod expr_prec;
mod init_stages;
mod native_functions;
mod printer;
mod scanner;
mod stage_problems;
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
                let stages = Arc::new(RwLock::new(StageInfo::new()));

                print!("\x1B[2J\x1B7\x1B[H");
                stages.clone().read().unwrap().print_progress_bar();
                print!("\x1B8");

                let curr_stage = current_stage(stages.clone());
                let user_input_rx = start_user_input(stages.clone());
                if curr_stage >= stages.read().unwrap().total_stages() {
                    println!("You finished");
                } else {
                    let handler = start_file_listener(user_input_rx, stages.clone(), curr_stage);
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
                process::exit(65);
            }
            InterpretError::RuntimeError => process::exit(70),
        },
    }
}
