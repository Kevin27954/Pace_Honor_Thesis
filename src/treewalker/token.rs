use std::fmt::Display;

use super::token_types::TokenType;

pub struct Token<T> {
    token_type: TokenType,
    lexeme: String,
    litearl: T,
    line: u32,
}

impl<T> Display for Token<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} {} {}",
            self.line, self.token_type, self.lexeme, self.litearl
        )
    }
}
