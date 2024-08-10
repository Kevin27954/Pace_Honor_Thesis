pub fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: i32, error_at: String, message: String) {
    println!("[line {}] Error {}: {}", line, error_at, message);
}
