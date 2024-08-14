use std::fmt::Display;

use super::token_types::TokenType;

#[derive(Debug)]
pub enum Number {
    // Up for consideration
    // Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(String),
}

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
                    "{}: {:?} {} {:?}",
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
