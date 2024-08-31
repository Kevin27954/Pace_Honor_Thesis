use std::{fmt::Display, hash::Hash};

use super::token_types::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    // Up for consideration
    // Integer(i64),
    Float(f64),
}

impl Number {
    pub fn hash_f64(&self, value: &f64) -> u32 {
        let bits = value.to_bits();
        (bits ^ (bits >> 32)) as u32
    }
}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Number::Float(num) => self.hash_f64(num).hash(state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Literal {
    Number(Number),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(string) => {
                write!(f, "{}", string)
            }
            Literal::Number(float) => match float {
                Number::Float(float) => {
                    write!(f, "{}", float)
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub litearl: Option<Literal>,
    pub line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.litearl {
            Some(litearl) => {
                write!(
                    f,
                    "{}: {:?} {} {}",
                    self.line, self.token_type, self.lexeme, litearl
                )
            }
            None => {
                write!(
                    f,
                    "{}: {:?} {} {:?}",
                    self.line, self.token_type, self.lexeme, self.litearl
                )
            }
        }
    }
}
