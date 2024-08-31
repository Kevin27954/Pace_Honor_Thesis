use crate::treewalker::errors::error;
use crate::treewalker::token::Number;

use super::errors::CompileErrors;
use super::token::{Literal, Token};
use super::token_types::{get_keywords, TokenType};

pub struct Scanner {
    source: Vec<char>,
    line: u32,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Scanner {
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> (Vec<Token>, bool) {
        let mut tokens: Vec<Token> = vec![];
        let mut has_error = false;
        while !self.is_end() {
            // Should never fail as it loops to .len()
            let char = self.source[self.current];
            let res = self.scan_token(&char, &mut tokens);

            if let Err(err) = res {
                has_error = true;
                error(self.line, format!("{}", err));
            }

            self.start = self.current;
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            litearl: None,
            line: self.line,
        });

        return (tokens, has_error);
    }

    fn is_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn match_next(&mut self, want: char) -> Option<bool> {
        if self.current >= self.source.len() {
            return None;
        }

        // It is impossible to get None as it is checked beforehand in previous statement
        if *self.source.get(self.current).unwrap() != want {
            return Some(false);
        }

        self.current += 1;
        Some(true)
    }

    fn peek(&self) -> Option<&char> {
        if self.is_end() {
            return Some(&'\0');
        }
        return self.source.get(self.current);
    }

    fn peek_next(&self) -> Option<&char> {
        return self.source.get(self.current + 1);
    }

    fn consume_next(&mut self) {
        self.current += 1;
    }

    fn is_digit(&self, token: Option<&char>) -> bool {
        if let Some(c) = token {
            return c.is_ascii_digit();
        } else {
            return false;
        }
    }

    fn is_alpha_numeric(&self, token: Option<&char>) -> bool {
        if let Some(char) = token {
            return char.is_alphanumeric() || char.eq(&'_');
        } else {
            return false;
        }
    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, token: TokenType) {
        let mut lexeme: String = self.source[self.start..self.current].iter().collect();

        if token == TokenType::NEW_LINE {
            lexeme = String::new();
        }

        tokens.push(Token {
            token_type: token,
            lexeme,
            litearl: None,
            line: self.line,
        })
    }

    fn add_token_with_litearl(
        &mut self,
        tokens: &mut Vec<Token>,
        token: TokenType,
        literal: Literal,
    ) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        tokens.push(Token {
            token_type: token,
            lexeme,
            litearl: Some(literal),
            line: self.line,
        })
    }

    fn scan_token(&mut self, token: &char, tokens: &mut Vec<Token>) -> Result<(), CompileErrors> {
        self.current += 1;
        match token {
            '\n' => {
                self.add_token(tokens, TokenType::NEW_LINE);
                if self.peek() != Some(&'\0') {
                    self.line += 1;
                }
            }
            ' ' | '\r' | '\t' => {}
            '(' => self.add_token(tokens, TokenType::LEFT_PAREN),
            ')' => self.add_token(tokens, TokenType::RIGHT_PAREN),
            '{' => self.add_token(tokens, TokenType::LEFT_BRACE),
            '}' => self.add_token(tokens, TokenType::RIGHT_BRACE),
            ',' => self.add_token(tokens, TokenType::COMMA),
            '.' => self.add_token(tokens, TokenType::DOT),
            '-' => self.add_token(tokens, TokenType::MINUS),
            '+' => self.add_token(tokens, TokenType::PLUS),
            '*' => self.add_token(tokens, TokenType::STAR),
            ';' => self.add_token(tokens, TokenType::SEMICOLON),
            ':' => self.add_token(tokens, TokenType::COLON),
            '!' => {
                if let Some(true) = self.match_next('=') {
                    self.add_token(tokens, TokenType::BANG_EQUAL)
                } else {
                    self.add_token(tokens, TokenType::BANG)
                }
            }
            '>' => {
                if let Some(true) = self.match_next('=') {
                    self.add_token(tokens, TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(tokens, TokenType::GREATER)
                }
            }
            '<' => {
                if let Some(true) = self.match_next('=') {
                    self.add_token(tokens, TokenType::LESS_EQUAL)
                } else {
                    self.add_token(tokens, TokenType::LESS)
                }
            }
            '=' => {
                if let Some(true) = self.match_next('=') {
                    self.add_token(tokens, TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(tokens, TokenType::EQUAL)
                }
            }
            '/' => {
                if let Some(true) = self.match_next('/') {
                    while self.peek() != Some(&'\n') && self.peek() != Some(&'\0') {
                        self.consume_next();
                    }
                    self.add_token(tokens, TokenType::COMMENT);
                } else {
                    self.add_token(tokens, TokenType::SLASH)
                }
            }
            '"' => {
                // If error, just return.
                let litearl = self.parse_string()?;
                self.add_token_with_litearl(tokens, TokenType::STRING, Literal::String(litearl))
            }
            c => {
                if c.is_ascii_digit() {
                    let number = self.parse_number()?;
                    self.add_token_with_litearl(
                        tokens,
                        TokenType::NUMBER,
                        Literal::Number(Number::Float(number)),
                    )
                } else if self.is_alpha_numeric(Some(token)) {
                    let identifier = self.parse_keywords();
                    let keywords = get_keywords();

                    let res = keywords.get(&identifier);
                    if let Some(token_type) = res {
                        self.add_token(tokens, token_type.clone());
                    } else {
                        self.add_token(tokens, TokenType::IDENTIFIER);
                    }
                } else {
                    return Err(CompileErrors::UnknownCharacter(*token));
                }
            }
        }
        Ok(())
    }

    fn parse_string(&mut self) -> Result<String, CompileErrors> {
        let mut contains_new_line = false;
        while self.peek().unwrap() != &'"' && !self.is_end() {
            if let Some('\n') = self.peek() {
                self.line += 1;
                contains_new_line = true;
            }
            self.consume_next();
        }

        // Consumes the ending '"' if exist
        self.consume_next();

        if self.is_end() {
            return Err(CompileErrors::UnterminatedString);
        } else if contains_new_line {
            return Err(CompileErrors::MultiLineStringError);
        }

        let litearl: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        Ok(litearl)
    }

    fn parse_number(&mut self) -> Result<f64, CompileErrors> {
        while self.is_digit(self.peek()) {
            self.consume_next();
        }

        let peek = self.peek();

        if let Some('.') = peek {
            if self.is_digit(self.peek_next()) {
                self.consume_next();
                while self.is_digit(self.peek()) {
                    self.consume_next();
                }
            }
        }

        let litearl: String = self.source[self.start..self.current].iter().collect();

        match litearl.parse::<f64>() {
            Ok(number) => Ok(number),
            // Should never fail, as I'm giving it only numbers
            Err(_err) => {
                unreachable!("Error should never occur as it is guarantee to be a number");
                //Err(CompileErrors::NumberParseError(litearl));
            }
        }
    }

    fn parse_keywords(&mut self) -> String {
        while self.is_alpha_numeric(self.peek()) {
            self.consume_next();
        }

        let identifier: String = self.source[self.start..self.current].iter().collect();

        identifier
    }
}
