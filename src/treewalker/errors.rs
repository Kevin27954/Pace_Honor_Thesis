use std::error::Error;

pub fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: u32, error_at: String, message: String) {
    println!("[line {}] Error {}: {}", line, error_at, message);
}

impl Error for CompileErrors {}

#[derive(Debug)]
pub enum CompileErrors {
    MultiLineStringError,
    UnterminatedString,
    UnknownCharacter(char),
    UnknownError(String),
}

impl std::fmt::Display for CompileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MultiLineStringError => {
                write!(f, "We don't Support Multi-Line Strings")
            }
            Self::UnterminatedString => {
                write!(f, "String wasn't terminated")
            }
            Self::UnknownCharacter(char) => {
                write!(f, "Unknown character: {}", char)
            }
            Self::UnknownError(err) => {
                write!(f, "An Unkown Error had occured: {:?}", err)
            }
        }
    }
}
