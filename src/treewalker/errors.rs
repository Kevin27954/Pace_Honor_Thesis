use std::error::Error;

pub fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

pub fn parse_err(message: String) {
    eprintln!("Error: {}", message);
}

pub fn report(line: u32, error_at: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, error_at, message);
}

impl Error for CompileErrors {}

#[derive(Debug)]
pub enum CompileErrors {
    MultiLineStringError,
    UnterminatedString,
    UnterminatedParenthesis,
    EmptyParentheses,
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
                write!(f, "Unterminated string")
            }
            Self::UnterminatedParenthesis => {
                write!(f, "Parenthesis wasn't terminated")
            }
            Self::EmptyParentheses => {
                write!(f, "Empty Parenthesis")
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
