use core::str;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
    sync::{
        mpsc::{self, Receiver},
        {Arc, RwLock},
    },
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use once_cell::sync::Lazy;

use crate::printer::{center_text, print_hint, print_msg};
use crate::stage_problems::StageInfo;

pub enum UserInput {
    Quit,
    Hint,
    Other,
}

static FILE_DIR: &str = "exercises";

static PATH: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/git/HonorThesis/target/release/Thesis",
        std::env::var("HOME").expect("HOME environment variable not set")
    )
});

pub fn start_user_input(stages: Arc<RwLock<StageInfo>>) -> mpsc::Receiver<UserInput> {
    let (tx, rx) = mpsc::channel::<UserInput>();

    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if &input.cmp(&String::from("quit\n")) == &std::cmp::Ordering::Equal {
            tx.send(UserInput::Quit).expect("Unable to send message");
            break;
        } else if &input.cmp(&String::from("hint\n")) == &std::cmp::Ordering::Equal {
            print_hint(stages.read().unwrap().get_stage_hint());
            println!("\n\n\n\n\n\n");
        } else {
            // Show user how to quit

            print!("\x1B[1A\x1B[2K\x1B7\x1B[50B\x1B[2K");
            let text = center_text(
                "Type \x1B[38;5;196mquit\x1B[0m to exit or quit the program.",
                17,
            );
            print!("{text}");
            print!("\x1B8");

            io::stdout().flush().unwrap();
        }
    });

    rx
}

pub fn start_file_listener(
    user_input_rx: Receiver<UserInput>,
    stages: Arc<RwLock<StageInfo>>,
    mut curr_stage: usize,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut last_modified = SystemTime::now();
        loop {
            match user_input_rx.recv_timeout(Duration::from_millis(200)) {
                Ok(userinput) => match userinput {
                    UserInput::Quit => break,
                    _ => {}
                },
                Err(err) => match err {
                    mpsc::RecvTimeoutError::Timeout => {}
                    mpsc::RecvTimeoutError::Disconnected => {
                        eprintln!("idk when this happens?");
                        break;
                    }
                },
            }

            let read_lock = stages.read().unwrap();

            if curr_stage >= read_lock.total_stages() {
                println!("You finished");
                break;
            }

            let file_path =
                Path::new(FILE_DIR).join(&read_lock.get_stage_complete_at(curr_stage).0);
            let file_path_str = file_path.to_str().unwrap();
            let data = fs::metadata(file_path_str);

            let mut is_modified = false;

            match data {
                Ok(file_data) => {
                    let modified_time =
                        file_data.modified().expect("Unable to check modified time");
                    match last_modified.cmp(&modified_time) {
                        std::cmp::Ordering::Less => {
                            is_modified = true;
                            last_modified = modified_time;
                        }
                        _ => {}
                    }
                }
                Err(_err) => {
                    match create_file(stages.clone(), curr_stage) {
                        Ok(_) => {}
                        Err(_err) => {}
                    };
                }
            };

            if is_modified {
                // Replace with built version in bin path on user device
                let program_result = Command::new(PATH.as_str())
                    .arg("run")
                    .arg(file_path_str)
                    .output()
                    .expect("Failed to run command");

                let success = program_result.status.success();
                if !contains_str(file_path_str).unwrap() && success {
                    // Marks it complete so hint gives correct hint
                    stages.write().unwrap().set_stage_completed(curr_stage);
                    curr_stage += 1;
                }

                if success {
                    print_msg(success, str::from_utf8(&program_result.stdout).unwrap());
                } else {
                    print_msg(success, str::from_utf8(&program_result.stderr).unwrap());
                }
            }
        }
    })
}

pub fn current_stage(stages: Arc<RwLock<StageInfo>>) -> usize {
    let read_only = stages.read().unwrap();
    let total_stages = read_only.total_stages();
    drop(read_only);

    for i in 0..total_stages {
        let read_only = stages.read().unwrap();
        let file_name = read_only.get_stage_complete_at(i).0.clone();

        let is_dir = fs::read_dir(FILE_DIR);
        match is_dir {
            Ok(_files) => {}
            Err(_err) => {
                let _ = fs::create_dir(FILE_DIR);
            }
        }

        let file_path = Path::new(FILE_DIR).join(file_name);
        let file_path_str = file_path.to_str().unwrap();
        if !fs::metadata(file_path_str).is_ok() {
            match create_file(stages.clone(), i) {
                Ok(_) => {
                    return i;
                }
                Err(err) => {
                    eprintln!("{err}");
                    break;
                }
            }
        }

        let program_result = Command::new(PATH.as_str())
            .arg("run")
            .arg(file_path_str)
            .output()
            .expect("Failed to run command.");

        drop(read_only);

        let success = program_result.status.success();
        if success {
            print_msg(success, str::from_utf8(&program_result.stdout).unwrap());
            if contains_str(file_path_str).unwrap() {
                return i;
            }
            stages.write().unwrap().set_stage_completed(i);
        } else {
            print_msg(success, str::from_utf8(&program_result.stderr).unwrap());
            return i;
        }
    }

    stages.read().unwrap().total_stages()
}

fn create_file(stages: Arc<RwLock<StageInfo>>, problem_num: usize) -> Result<(), io::Error> {
    let file_path =
        Path::new(FILE_DIR).join(&stages.read().unwrap().get_stage_complete_at(problem_num).0);
    let file_path_str = file_path.to_str().unwrap();
    let mut file = File::create(file_path_str)?;

    file.write(stages.read().unwrap().get_problem(problem_num).as_bytes())?;

    Ok(())
}

fn contains_str(path: &str) -> Result<bool, io::Error> {
    let program = fs::read_to_string(path)?.to_lowercase();
    Ok(program.contains(&"// STILL LEARNING".to_lowercase()))
}
