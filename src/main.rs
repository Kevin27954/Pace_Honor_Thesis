use std::{env, process};

use treewalker::run_file;
mod treewalker;

fn main() {
    if env::args().len() > 4 {
        println!("You entered too many arguments");
        process::exit(1);
    } else {
        // clt-tool run main.txt -v
        // cli-tool kevling -l

        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
        let cmd = &args[1];

        match cmd.as_str() {
            "run" => {
                run_file(&args[2]);
            }
            "kevling" => {
                todo!("Should start the ASCII adventure");
            }
            _ => {
                println!("Unknown command");
                process::exit(1);
            }
        }
    }
}
