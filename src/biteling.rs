use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use once_cell::sync::Lazy;

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
static INIT_STAGE_PATH: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/git/HonorThesis/init_stage/",
        std::env::var("HOME").expect("HOME environment variable not set")
    )
});

pub fn start_user_input() -> mpsc::Receiver<UserInput> {
    let (tx, rx) = mpsc::channel::<UserInput>();

    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if &input.cmp(&String::from("quit\n")) == &std::cmp::Ordering::Equal {
            tx.send(UserInput::Quit).expect("Unable to send message");
            break;
        } else if &input.cmp(&String::from("hint\n")) == &std::cmp::Ordering::Equal {
            tx.send(UserInput::Hint).expect("Unable to send message");
        } else {
            //tx.send(UserInput::Other(input))
            tx.send(UserInput::Other).expect("Unable to send message");
        }
    });

    rx
}

pub fn start_file_listener(
    user_input_rx: Receiver<UserInput>,
    stages: Vec<(String, bool)>,
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

            if curr_stage >= stages.len() {
                println!("You finished");
                break;
            }

            let file_path = Path::new(FILE_DIR).join(&stages[curr_stage].0);
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
                    match create_file(&stages[curr_stage].0) {
                        Ok(_) => {}
                        Err(_err) => {}
                    };
                }
            };

            if is_modified {
                // Replace with built version in bin path on user device
                let status = Command::new(PATH.as_str())
                    .arg("run")
                    .arg(file_path_str)
                    .status()
                    .expect("Failed to run command");
                if status.success() {
                    curr_stage += 1;
                    println!("You passed");
                } else {
                    println!("You failed");
                }
            }
        }
    })
}

pub fn current_stage(stages: &mut Vec<(String, bool)>) -> usize {
    for i in 0..stages.len() {
        let file_name = &stages[i].0;

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
            match create_file(file_name) {
                Ok(_) => {
                    return i;
                }
                Err(err) => {
                    eprintln!("{err}");
                    break;
                }
            }
        }

        let status = Command::new(PATH.as_str())
            .arg("run")
            .arg(file_path_str)
            .status()
            .expect("Failed to run command.");

        if status.success() {
            stages[i].1 = true;
            println!("You passed");
        } else {
            return i;
        }
    }

    stages.len()
}

pub fn get_info() -> Vec<(String, bool)> {
    vec![
        (String::from("1_print.txt"), false),
        (String::from("2_number.txt"), false),
        (String::from("3_string.txt"), false),
        (String::from("4_math.txt"), false),
    ]
}

fn create_file(file_name: &str) -> Result<(), io::Error> {
    let file_path = Path::new(FILE_DIR).join(file_name);
    let file_path_str = file_path.to_str().unwrap();
    let mut file = File::create(file_path_str)?;

    println!("{}", format!("{}{}", *INIT_STAGE_PATH, file_name));
    let data =
        fs::read(format!("{:?}{}", *INIT_STAGE_PATH, file_name)).expect("Unable to read file");
    println!("{}", String::from_utf8(data.clone()).unwrap());
    file.write(&data)?;

    Ok(())
}

/*

public class Conditional extends Statement {
    Statement thenBlock
    Statenemnt elseBlock
    Expression name
}

public class Vairable extends Expression {
    String variable name
}

public class Declarations {
    Vec<Declaration>
}

*/
