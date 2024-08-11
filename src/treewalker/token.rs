use std::fmt::Display;

use super::token_types::TokenType;

enum Number {
    Integer,
    Float,
}

#[derive(Debug)]
pub enum Literal {
    Number,
    String,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub litearl: Option<Literal>,
    pub line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} {} {:?}",
            self.line, self.token_type, self.lexeme, self.litearl
        )
    }
}
