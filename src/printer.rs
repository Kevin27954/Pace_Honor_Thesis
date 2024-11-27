use std::sync::{Arc, RwLock};

use crate::stage_problems::StageInfo;

pub fn center_text(text: &str, offset: u16) -> String {
    let size = termsize::get().unwrap();
    // It should fit in unless the user is being a troll

    let mut length = 0;
    for char in text.chars() {
        if char.is_ascii() {
            length += 1;
        } else {
            length += 3;
        }
    }

    let mid_text_idx: u16 = ((length) - offset) / 2;

    let num_empty_space = if (size.cols / 2) - mid_text_idx > 0 {
        " ".repeat(((size.cols / 2) - mid_text_idx).into())
    } else {
        String::new()
    };

    String::from(num_empty_space + text)
}

pub fn center_multi_line_text(text: &mut String) {
    let size = termsize::get().unwrap();

    let empty_space = if (size.cols / 2) - 22 > 0 {
        " ".repeat(((size.cols / 2) - 22).into())
    } else {
        String::new()
    };

    let chars = text.chars();
    let mut idx: Vec<usize> = vec![];
    let mut i = 0;
    for char in chars {
        if char == '\n' {
            idx.push(i + 1);
        }
        if !char.is_ascii() {
            i += 3;
        }
        i += 1;
    }

    let mut offset = 0;
    for i in 0..idx.len() - 1 {
        text.insert_str(idx[i] + offset, &empty_space);
        offset += empty_space.len();
    }
}

pub fn print_hint(hint: &str) {
    let mut dog = String::from("\n  /^ ^\\\n / 0 0 \\\n V\\ Y /V\n  / - \\ \n /    |\nV__) ||\n");
    center_dog(&mut dog);

    let new_hint = center_hint(hint);

    print!("\x1B[2J\x1B[10B");
    print!("{}\x1B[38;5;226m{}\x1B[0m", new_hint, dog);
}

fn center_hint(hint: &str) -> String {
    let size = termsize::get().unwrap();

    let empty_space = if (size.cols / 2) - 22 > 0 {
        " ".repeat(((size.cols / 2) - 22).into())
    } else {
        String::new()
    };

    let mut new_hint = String::new();

    new_hint.push('\n');
    new_hint.push('+');
    new_hint.push_str(&"=".repeat(42));
    new_hint.push('+');
    new_hint.push('\n');

    let chars: Vec<char> = hint.chars().collect();

    let mut counter = 0;
    let mut is_end = 0;
    for i in 0..chars.len() {
        if counter % 40 == 0 {
            is_end += 1;
            if is_end % 2 == 0 {
                new_hint.push(' ');
                new_hint.push('|');
                new_hint.push('\n');
            } else {
                new_hint.push('|');
                new_hint.push(' ');
            }
        }
        if chars[i] == '\n' {
            let fill_amount = 40 - (counter % 40);
            new_hint.push_str(&" ".repeat(fill_amount));
            new_hint.push(' ');
            new_hint.push('|');
            new_hint.push('\n');
            counter = 0;
            is_end = 0;
        } else {
            if !chars[i].is_ascii() {
                counter += 1;
            }
            counter += 1;
            new_hint.push(chars[i]);
        }
    }

    new_hint.push('+');
    new_hint.push_str(&"=".repeat(42));
    new_hint.push('+');
    new_hint.push('\n');

    let chars = new_hint.chars();
    let mut idx: Vec<usize> = vec![];
    let mut i = 0;
    for char in chars {
        if char == '\n' {
            idx.push(i + 1);
        }
        if !char.is_ascii() {
            i += 3;
        }
        i += 1;
    }

    let mut offset = 0;
    for i in 0..idx.len() - 1 {
        new_hint.insert_str(idx[i] + offset, &empty_space);
        offset += empty_space.len();
    }

    new_hint
}

fn center_dog(dog: &mut String) {
    let size = termsize::get().unwrap();

    let chars = dog.chars();
    let mut idx: Vec<usize> = vec![];
    for (i, char) in chars.enumerate() {
        if char == '\n' {
            idx.push(i + 1);
        }
    }

    let mut offset = 0;
    let empty_space = if (size.cols / 2) - 4 > 0 {
        " ".repeat(((size.cols / 2) - 4).into())
    } else {
        String::new()
    };

    for i in 0..idx.len() - 1 {
        dog.insert_str(idx[i] + offset, &empty_space);
        offset += empty_space.len();
    }
}

// =============================================

pub fn print_msg(success: bool, msg: &str, stages: Arc<RwLock<StageInfo>>) {
    print!("\x1B[2J\x1B[H");
    stages.read().unwrap().print_progress_bar();

    if success {
        success_output(msg);
    } else {
        failure_output(msg);
    }
}

fn failure_output(msg: &str) {
    print!(
        "\
\n
 \x1B[38;5;124m
      _____ ____  ____   ___  ____             _____ ____  ____   ___  ____             _____ ____  ____   ___  ____  
     | ____|  _ \\|  _ \\ / _ \\|  _ \\           | ____|  _ \\|  _ \\ / _ \\|  _ \\           | ____|  _ \\|  _ \\ / _ \\|  _ \\ 
     |  _| | |_) | |_) | | | | |_) |          |  _| | |_) | |_) | | | | |_) |          |  _| | |_) | |_) | | | | |_) |
     | |___|  _ <|  _ <| |_| |  _ <           | |___|  _ <|  _ <| |_| |  _ <           | |___|  _ <|  _ <| |_| |  _ < 
     |_____|_| \\_\\_| \\_\\\\___/|_| \\_\\          |_____|_| \\_\\_| \\_\\\\___/|_| \\_\\          |_____|_| \\_\\_| \\_\\\\___/|_| \\_\\
 \x1B[0m                               
\x1B[38;5;226m{}\x1B[0m

{}

\x1B[38;5;161m{}\x1B[0m
",
        center_text("Please fix the cause of this error below to move on", 0),
        create_bar(),
        msg,
    );
}

fn success_output(msg: &str) {
    let mut centered_text = format!(
        "\
\n
\x1B[38;5;160m##    ## ##     ## ########   #######   ######
\x1B[38;5;208m##   ##  ##     ## ##     ## ##     ## ##    ##
\x1B[38;5;220m##  ##   ##     ## ##     ## ##     ## ##    
\x1B[38;5;82m#####    ##     ## ##     ## ##     ##  ###### 
\x1B[38;5;33m##  ##   ##     ## ##     ## ##     ##       ##
\x1B[38;5;21m##   ##  ##     ## ##     ## ##     ## ##    ##
\x1B[38;5;56m##    ##  #######  ########   #######   ######  
\x1B[0m
        ",
    );

    center_multi_line_text(&mut centered_text);

    let mut reminder = String::from(
        "\
\n
+=======================================+
| Remember to remove the text:          |
|                                       |
| \x1B[38;5;60m// STILL LEARNING\x1B[0m                     |
|                                       |
| To move on to the next stage          |
+=======================================+
",
    );

    center_multi_line_text(&mut reminder);

    print!(
        "{centered_text} 
{}

{}

{}
{}
{msg}
        ",
        center_text(
            "🚀 \x1B[38;5;113mCongratulations! You Passed This Stage!\x1B[0m 🚀",
            20
        ),
        reminder,
        center_text("<---- Output", 0),
        create_bar(),
    );
}

fn create_bar() -> String {
    let size = termsize::get().unwrap();

    let mut bar = String::new();
    for _ in 0..size.cols {
        bar.push('=');
    }

    bar
}
