use super::token::Token;
use super::token_types::TokenType;

pub struct Scanner {
    source: Vec<char>,
    line: u32,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_end() {
            // Should never fail as it loops to .len()
            let char = self.source.get(self.current).unwrap().clone();
            let token = self.scan_token(&char);

            match token {
                Some(TokenType::NEW_LINE) => {
                    self.line += 1;
                }
                Some(TokenType::WHITE_SPACE) => {}
                Some(token) => {
                    let lexeme: String = self.source[self.start..self.current + 1].iter().collect();

                    tokens.push(Token {
                        token_type: token,
                        lexeme,
                        litearl: None,
                        line: self.line,
                    })
                }
                None => {
                    break;
                }
            }

            self.current += 1;
            self.start = self.current;
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            litearl: None,
            line: self.line,
        });

        return tokens;
    }

    fn is_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }
        false
    }

    fn match_next(&mut self, want: char, idx: usize) -> bool {
        if idx >= self.source.len() {
            return false;
        }

        // It is impossible to get None as it is checked beforehand in previous statement
        if *self.source.get(idx).unwrap() != want {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek_next(&self) -> Option<&char> {
        return self.source.get(self.current + 1);
    }

    fn ignore_next(&mut self) {
        self.current += 1;
    }

    fn scan_token(&mut self, token: &char) -> Option<TokenType> {
        match token {
            '\n' => Some(TokenType::NEW_LINE),
            ' ' | '\r' | '\t' => Some(TokenType::WHITE_SPACE),
            '(' => Some(TokenType::LEFT_PAREN),
            ')' => Some(TokenType::RIGHT_PAREN),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '-' => Some(TokenType::MINUS),
            '+' => Some(TokenType::PLUS),
            '*' => Some(TokenType::STAR),
            ';' => Some(TokenType::SEMICOLON),
            '!' => {
                if self.match_next('=', self.current + 1) {
                    Some(TokenType::BANG_EQUAL)
                } else {
                    Some(TokenType::BANG)
                }
            }
            '>' => {
                if self.match_next('=', self.current + 1) {
                    Some(TokenType::GREATER_EQUAL)
                } else {
                    Some(TokenType::GREATER)
                }
            }
            '<' => {
                if self.match_next('=', self.current + 1) {
                    Some(TokenType::LESS_EQUAL)
                } else {
                    Some(TokenType::LESS)
                }
            }
            '=' => {
                if self.match_next('=', self.current + 1) {
                    Some(TokenType::EQUAL_EQUAL)
                } else {
                    Some(TokenType::EQUAL)
                }
            }
            '/' => {
                if self.match_next('/', self.current + 1) {
                    while !self.match_next('\n', self.current + 1) {
                        self.ignore_next();
                    }
                    Some(TokenType::NEW_LINE)
                } else {
                    Some(TokenType::SLASH)
                }
            }
            _ => {
                eprintln!("Unknown character, {}, at line {}!", token, self.line);
                None
            }
        }
    }
}
