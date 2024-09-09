use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Parenthesis
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Symbols
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    Colon,

    // Equality Operator
    // !
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Comparison Operators
    And,
    Or,

    // Literals
    Identifier,
    String,
    Number,

    // Boolean values
    True,
    False,
    None,

    // Conditional Statements
    If,
    Else,
    Then,

    // Functions
    Function,

    // Structs
    Struct,

    // Loops
    For,
    While,
    Do,

    // Ending Keyword for functions, loops, etc
    End,

    // Variable declaration
    Let,

    // Return statement
    Return,

    // Comments
    Comment,

    // Terminating Symbols
    NewLine,

    Error,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.line, self.token_type, self.lexeme)
    }
}

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut chars: Vec<char> = source.chars().collect();
        // Signify it is the end of file
        chars.push('\0');
        Scanner {
            source: chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.source[self.current] == '\0' || self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            self.line,
        )
    }

    fn make_error_token(&self, msg: String) -> Token {
        Token::new(TokenType::Error, msg, self.line)
    }

    pub fn advance(&mut self) -> char {
        let char = self.source[self.current];
        self.current += 1;
        char
    }

    fn peek_at(&self, idx: usize) -> char {
        self.source[idx]
    }

    pub fn peek(&self) -> char {
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn match_next(&mut self, character: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() == character {
            self.advance();
            return true;
        }
        false
    }

    fn skip_whitespace(&mut self) {
        loop {
            let char = self.peek();
            match char {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while !self.is_at_end() && self.peek() == '\n' {
                            self.advance();
                        }
                    }
                    break;
                }
                _ => {
                    break;
                }
            };
        }
    }

    fn make_string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }

        if self.is_at_end() {
            return self.make_error_token("Unterminated String".to_string());
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn make_number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_identifier(&mut self) -> Token {
        let mut char = self.peek();
        while char.is_alphabetic() || char == '_' || char.is_ascii_digit() {
            self.advance();
            char = self.peek();
        }

        let identifier_type = self.identifier_type();
        self.make_token(identifier_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        let char = self.peek_at(self.start);

        match char {
            'a' => return self.cmp_char(&['n', 'd'], TokenType::And),
            'd' => return self.cmp_char(&['o'], TokenType::Do),
            'i' => return self.cmp_char(&['f'], TokenType::If),
            'l' => return self.cmp_char(&['e', 't'], TokenType::Let),
            'n' => return self.cmp_char(&['o', 'n', 'e'], TokenType::None),
            'o' => return self.cmp_char(&['r'], TokenType::Or),
            'r' => return self.cmp_char(&['e', 't', 'u', 'r', 'n'], TokenType::Return),
            's' => return self.cmp_char(&['t', 'r', 'u', 'c', 't'], TokenType::Struct),
            'w' => return self.cmp_char(&['h', 'i', 'l', 'e'], TokenType::While),
            'e' => {
                if self.current - self.start > 1 {
                    match self.peek_at(self.start + 1) {
                        'l' => return self.cmp_char(&['l', 's', 'e'], TokenType::Else),
                        'n' => return self.cmp_char(&['n', 'd'], TokenType::End),
                        _ => {}
                    }
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.peek_at(self.start + 1) {
                        'h' => return self.cmp_char(&['h', 'e', 'n'], TokenType::Then),
                        'r' => return self.cmp_char(&['r', 'u', 'e'], TokenType::True),
                        _ => {}
                    }
                }
            }
            'f' => {
                if self.current - self.start > 1 {
                    match self.peek_at(self.start + 1) {
                        'a' => return self.cmp_char(&['a', 'l', 's', 'e'], TokenType::False),
                        'o' => return self.cmp_char(&['o', 'r'], TokenType::For),
                        'u' => {
                            return self.cmp_char(
                                &['u', 'n', 'c', 't', 'i', 'o', 'n'],
                                TokenType::Function,
                            )
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        TokenType::Identifier
    }

    fn cmp_char(&self, chars: &[char], token_type: TokenType) -> TokenType {
        if self.current - self.start != chars.len() + 1 {
            return TokenType::Identifier;
        }

        for i in 0..(self.current - self.start) - 1 {
            if chars[i] != self.peek_at(self.start + 1 + i) {
                return TokenType::Identifier;
            }
        }

        token_type
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let char = self.advance();
        if char.is_alphabetic() {
            return self.make_identifier();
        }
        if char.is_ascii_digit() {
            return self.make_number();
        }

        match char {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightParen),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '+' => return self.make_token(TokenType::Plus),
            '-' => return self.make_token(TokenType::Minus),
            '*' => return self.make_token(TokenType::Star),
            '/' => return self.make_token(TokenType::Slash),
            ';' => return self.make_token(TokenType::Semicolon),

            '\n' => {
                self.line += 1;
                return self.make_token(TokenType::NewLine);
            }

            '!' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::BangEqual);
                }
                return self.make_token(TokenType::Bang);
            }
            '=' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::EqualEqual);
                }
                return self.make_token(TokenType::Equal);
            }
            '>' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::GreaterEqual);
                }
                return self.make_token(TokenType::Greater);
            }
            '<' => {
                if self.match_next('=') {
                    return self.make_token(TokenType::LessEqual);
                }
                return self.make_token(TokenType::Less);
            }

            '"' => return self.make_string(),
            _ => {}
        }

        return self.make_token(TokenType::Error);
    }
}
