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
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
            tokens: vec![],
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn scan(&mut self) -> bool {
        let mut has_error = false;
        while !self.is_end() {
            // Should never fail as it loops to .len()
            let char = self.source.get(self.current).unwrap().clone();
            let res = self.scan_token(&char);
            match res {
                Ok(()) => {}
                Err(err) => {
                    has_error = true;
                    error(self.line, format!("{}", err));
                }
            }

            self.start = self.current;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            litearl: None,
            line: self.line,
        });
        return has_error;
    }

    fn is_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }
        false
    }

    fn match_next(&mut self, want: char, idx: usize) -> Option<bool> {
        if idx >= self.source.len() {
            return None;
        }

        // It is impossible to get None as it is checked beforehand in previous statement
        if *self.source.get(idx).unwrap() != want {
            return Some(false);
        }

        self.current += 1;
        Some(true)
    }

    fn peek(&self) -> Option<&char> {
        return self.source.get(self.current);
    }

    fn peek_next(&self) -> Option<&char> {
        return self.source.get(self.current + 1);
    }

    fn ignore_next(&mut self) {
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

    fn add_token(&mut self, token: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        self.tokens.push(Token {
            token_type: token,
            lexeme,
            litearl: None,
            line: self.line,
        })
    }

    fn add_token_with_litearl(&mut self, token: TokenType, literal: Literal) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        self.tokens.push(Token {
            token_type: token,
            lexeme,
            litearl: Some(literal),
            line: self.line,
        })
    }

    fn scan_token(&mut self, token: &char) -> Result<(), CompileErrors> {
        self.current += 1;
        match token {
            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => {}
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            '*' => self.add_token(TokenType::STAR),
            ';' => self.add_token(TokenType::SEMICOLON),
            '!' => {
                if let Some(true) = self.match_next('=', self.current) {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '>' => {
                if let Some(true) = self.match_next('=', self.current) {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '<' => {
                if let Some(true) = self.match_next('=', self.current) {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '=' => {
                if let Some(true) = self.match_next('=', self.current) {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '/' => {
                if let Some(true) = self.match_next('/', self.current) {
                    while let Some(c) = self.peek() {
                        if c.eq(&'\n') {
                            break;
                        }
                        self.ignore_next();
                    }
                    self.add_token(TokenType::COMMENT);
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            '"' => {
                // If error, just return.
                let litearl = self.parse_string()?;
                self.add_token_with_litearl(TokenType::STRING, Literal::String(litearl))
            }
            c => {
                if c.is_ascii_digit() {
                    let number = self.parse_number()?;
                    self.add_token_with_litearl(
                        TokenType::NUMBERS,
                        Literal::Number(Number::Float(number)),
                    )
                } else if self.is_alpha_numeric(Some(token)) {
                    // Literally can not fail
                    let identifier = self.parse_keywords().unwrap();
                    let keywords = get_keywords();

                    let res = keywords.get(&identifier);
                    if let Some(token_type) = res {
                        self.add_token_with_litearl(
                            token_type.clone(),
                            Literal::String(identifier),
                        );
                    } else {
                        self.add_token_with_litearl(
                            TokenType::IDENTIFIER,
                            Literal::String(identifier),
                        );
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
        while let Some(false) = self.match_next('"', self.current) {
            if let Some('\n') = self.peek() {
                self.line += 1;
                contains_new_line = true;
            }
            self.ignore_next();
        }

        if let None = self.peek() {
            self.line -= 1;
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
            self.ignore_next();
        }

        let peek = self.peek();

        if let Some('.') = peek {
            if self.is_digit(self.peek_next()) {
                self.ignore_next();
                while self.is_digit(self.peek()) {
                    self.ignore_next();
                }
            }
        }

        let litearl: String = self.source[self.start..self.current].iter().collect();

        match litearl.parse::<f64>() {
            Ok(number) => Ok(number),
            // Should never fail, as I'm giving it only numbers
            Err(err) => Err(CompileErrors::UnknownError(err.to_string())),
        }
    }

    fn parse_keywords(&mut self) -> Result<String, CompileErrors> {
        //todo!("Finish this parse_keywords");
        while self.is_alpha_numeric(self.peek()) {
            self.ignore_next();
        }

        let identifer: String = self.source[self.start..self.current].iter().collect();

        Ok(identifer)
    }
}
