use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use super::{runtime_types::RuntimeValue, token::Token, token_types::TokenType};

pub fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

pub fn parse_error(compile_err: CompileErrors) {
    match &compile_err {
        CompileErrors::UnterminatedParenthesis(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme,);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::EmptyParentheses(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::ExpectExpr(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::EndOfFile => {}
        _ => {
            eprintln!(
                "This error should not be called by parser_error\n{}",
                compile_err
            )
        }
    }
}

pub fn parse_runtime_err(runtime_err: RuntimeError) {
    match &runtime_err {
        RuntimeError::UnaryTypeMismatch(operator, _value) => {
            report(operator.line, "".to_string(), runtime_err.to_string())
        }
        RuntimeError::BinaryTypeMismatch(_left, operator, _right) => {
            report(operator.line, "".to_string(), runtime_err.to_string())
        }
        RuntimeError::DivideByZero(operator) => {
            report(operator.line, "".to_string(), runtime_err.to_string())
        }
        _ => {}
    }
}

pub fn report(line: u32, error_at: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, error_at, message);
}

impl Error for CompileErrors {}

#[derive(Debug)]
pub enum CompileErrors {
    MultiLineStringError,
    UnterminatedString,
    EndOfFile,
    UnknownCharacter(char),
    UnterminatedParenthesis(Token),
    EmptyParentheses(Token),
    ExpectExpr(Token),
    NumberParseError(String),
    UnknownError(Token, String),
}

impl std::fmt::Display for CompileErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MultiLineStringError => {
                write!(f, "We don't Support Multi-Line Strings")
            }
            Self::UnterminatedString => {
                write!(f, "Unterminated string")
            }
            Self::EndOfFile => {
                write!(f, "")
            }
            Self::UnknownCharacter(char) => {
                write!(f, "Unknown character: {}", char)
            }
            Self::NumberParseError(string) => {
                write!(f, "Unable to parse number {}.", string)
            }
            Self::UnterminatedParenthesis(_token) => {
                write!(f, "Parenthesis wasn't terminated")
            }
            Self::EmptyParentheses(_token) => {
                write!(f, "Empty Parenthesis")
            }
            Self::ExpectExpr(_token) => {
                write!(f, "Expected Expression")
            }
            Self::UnknownError(token, err) => {
                write!(
                    f,
                    "An Unkown Error had occured at line {} with token {}: {:?}",
                    token.line, token.lexeme, err
                )
            }
        }
    }
}

pub enum RuntimeError<'a> {
    UnaryTypeMismatch(&'a Token, RuntimeValue),
    BinaryTypeMismatch(RuntimeValue, &'a Token, RuntimeValue),
    DivideByZero(&'a Token),

    InvalidOperation(RuntimeValue),
    InvalidConverstion(RuntimeValue),

    UnknownError,
}

impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::UnaryTypeMismatch(token_type, value) => {
                write!(
                    f,
                    "Type Mismatch: Cannot apply operator {} to {}.",
                    token_type.lexeme,
                    value.err_format()
                )
            }
            Self::BinaryTypeMismatch(left, opeartor, right) => {
                write!(
                    f,
                    "Type Mismatch: Cannot apply operator {} to incompatible types: {} and {}",
                    opeartor.lexeme,
                    left.err_format(),
                    right.err_format()
                )
            }
            Self::DivideByZero(_) => {
                write!(
                    f,
                    "Cannot divide by zero. Results in infinity, Not a Number (NaN)."
                )
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
