use chunk::Chunk;

use crate::{
    scanner::{Scanner, Token, TokenType},
    vm::InterpretResult,
};

pub mod chunk;
pub mod common;
pub mod values;

enum CompileError {
    CompileError,
}

pub struct Parser {
    previous: Option<Token>,
    current: Option<Token>,
    has_error: bool,
    // Can possibly replace with Result/Option type
    panic_error: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            // Inital state is None, All subsequent values are not null.
            previous: None,
            current: None,
            has_error: false,
            panic_error: false,
        }
    }

    pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
        let mut scanner = Scanner::new(source);
        let res = self.advance(&mut scanner);
        if let Err(_) = res {
            self.has_error = true;
        }

        todo!("Call on expression");
        //self.expression();

        !self.has_error
    }

    // The key is to ignore all errors afterwards, Not have a return error.
    // Thus result should work because it should ignore all later code due to an error and we can
    // just syncrhonize at the approiate place - (places where we are calling the parsing method
    fn advance(&mut self, scanner: &mut Scanner) -> Result<(), CompileError> {
        self.previous = self.current.take();

        loop {
            self.current = Some(scanner.scan_token());
            if let Some(token) = &self.current {
                if token.token_type != TokenType::Error {
                    break;
                }
            }

            self.has_error = true;
            self.error(&self.current, "You got some dogshit symbols")?
        }

        Ok(())
    }

    fn consume(
        &mut self,
        scanner: &mut Scanner,
        token_type: TokenType,
    ) -> Result<(), CompileError> {
        if let Some(token) = &self.current {
            if token.token_type == token_type {
                self.advance(scanner);
                return Ok(());
            }
        }

        self.error(&self.current, "Unexpected Token")
    }

    fn error(&self, opt_token: &Option<Token>, message: &str) -> Result<(), CompileError> {
        if let Some(token) = opt_token {
            print!("[line{}] Error", token.line);

            if token.token_type == TokenType::EOF {
                print!(" at end of file");
            } else if token.token_type == TokenType::Error {
                // The message would be passed?
                // But don't we still want to display the Token??
                // Or are we not going to store the token? when we get an error?
            } else {
                print!(" at {}", token.lexeme);
            }

            println!(": {message}");
        }

        Err(CompileError::CompileError)
    }
}
