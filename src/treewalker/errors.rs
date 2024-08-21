use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use super::{runtime_types::RuntimeValue, token::Token};

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
        CompileErrors::ExpectNewLine(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::InvalidIdentifier(ref token) => {
            report(token.line, "".to_string(), compile_err.to_string())
        }
        CompileErrors::KeywordAsIdentifier(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::ExpectKeywordDo(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        CompileErrors::UnterminatedDo(ref token) => {
            let err_at = format!("at \"{}\"", token.lexeme);
            report(token.line, err_at, compile_err.to_string())
        }
        _ => {
            eprintln!(
                "This error should not be called by parser_error.\nYou probably didn't implement it yet:\n{}",
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
        RuntimeError::UndeclaredVariable(token) => {
            report(token.line, "".to_string(), runtime_err.to_string())
        }
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
    UnknownCharacter(char),
    UnterminatedParenthesis(Token),
    UnterminatedDo(Token),
    // Combine into ExpectToken(expected, got)?
    ExpectNewLine(Token),
    ExpectExpr(Token),
    ExpectKeywordDo(Token),

    InvalidIdentifier(Token),
    KeywordAsIdentifier(Token),

    EmptyParentheses(Token),
}

impl std::fmt::Display for CompileErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::MultiLineStringError => {
                write!(f, "We don't support Multi-Line String")
            }
            Self::UnterminatedString => {
                write!(f, "Unterminated String")
            }
            Self::ExpectNewLine(_token) => {
                write!(f, "Expected New Line")
            }
            Self::ExpectKeywordDo(token) => {
                write!(f, "Expected Keyword 'do' before '{}'", token.lexeme)
            }
            Self::InvalidIdentifier(_token) => {
                write!(f, "Invalid Identifier name")
            }
            Self::KeywordAsIdentifier(token) => {
                write!(
                    f,
                    "Can't use Reserved Word '{}' as Identifier",
                    token.lexeme
                )
            }
            Self::UnknownCharacter(char) => {
                write!(f, "Unknown character: {}", char)
            }
            Self::UnterminatedParenthesis(_token) => {
                write!(f, "Parenthesis wasn't terminated")
            }
            Self::UnterminatedDo(_token) => {
                write!(f, "'do' wasn't accompanied by 'end'")
            }
            Self::EmptyParentheses(_token) => {
                write!(f, "Empty Parenthesis")
            }
            Self::ExpectExpr(_token) => {
                write!(f, "Expected Expression")
            }
        }
    }
}

pub enum RuntimeError {
    UnaryTypeMismatch(Token, RuntimeValue),
    BinaryTypeMismatch(RuntimeValue, Token, RuntimeValue),
    DivideByZero(Token),
    UndeclaredVariable(Token),
}

impl Display for RuntimeError {
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
            Self::UndeclaredVariable(var) => {
                write!(f, "Undeclared Variable: {}", var.lexeme)
            }
        }
    }
}
