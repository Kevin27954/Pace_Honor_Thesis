pub fn center_text(text: &str, offset: u16) -> String {
    let size = termsize::get().unwrap();
    // It should fit in unless the user is being a troll
    let mid_text_idx: u16 = ((text.len() as u16) - offset) / 2;

    let num_empty_space = if (size.cols / 2) - mid_text_idx > 0 {
        " ".repeat(((size.cols / 2) - mid_text_idx).into())
    } else {
        String::new()
    };

    String::from(num_empty_space + text)
}

pub fn print_hint(hint: &str) {
    let mut dog = String::from("\n  /^ ^\\\n / 0 0 \\\n V\\ Y /V\n  / - \\ \n /    |\nV__) ||\n");
    center_dog(&mut dog);

    let new_hint = center_hint(hint);

    print!("\x1B[2J\x1B[20B");
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
            println!("Fillamount: {fill_amount}");
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

    println!("{new_hint}");

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

pub fn print_msg(success: bool, msg: &str) {
    print!("\x1B[2J\x1B[H");
    println!("This would be the progress bar to a quiz");

    if success {
        succes_output(msg);
    } else {
        failure_output(msg);
    }
}

fn failure_output(msg: &str) {
    print!(
        "\
There seems to be an error in your program:

{}{}{}
Please fix it to move on.
",
        "\x1B[38;5;161m", msg, "\x1B[0m",
    );
}

fn succes_output(msg: &str) {
    print!(
        "\
{}Congratulations You Passed!{}

Your program output:
==============================

{}
==============================

Remember to remove the text:

{}// STILL LEARNING
{}
To move on to the next stage
",
        // I might need to perhaps get a fucntion for this so it is not just all over the place and
        // more organized. These code means absolutely nothing to me.
        "\x1B[38;5;113m\x1B[48;5;33m",
        "\x1B[0m",
        msg,
        "\x1B[38;5;27m",
        "\x1B[0m",
    );
}
