use chunk::{Chunk, OpCode};
use values::Value;

use crate::{
    debug::disassemble_chunk,
    expr_prec::{get_parse_rule, ParseFn, PRECEDENCE},
    scanner::{Scanner, Token, TokenType},
};

pub mod chunk;
pub mod common;
pub mod values;

//enum CompileError {
//    CompileError,
//}

pub struct Parser<'a> {
    previous: Option<Token>,
    current: Option<Token>,

    chunk: &'a mut Chunk,
    scanner: Option<Scanner>,

    pub has_error: bool,
    // Can possibly replace with Result/Option type
    panic_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Parser {
            // Inital state is None, All subsequent values are not null.
            // Current and Peek/Next, makes more sense so far
            previous: None,
            current: None,

            chunk,
            scanner: None,

            has_error: false,
            panic_error: false,
        }
    }

    //pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
    pub fn compile(&mut self, source: String) -> bool {
        self.scanner = Some(Scanner::new(source));

        self.advance();
        self.expression();

        self.consume(TokenType::EOF, "End of File");

        self.emit_return();
        if self.has_error {
            disassemble_chunk(self.chunk, "Parser Errors".to_string());
        }

        !self.has_error
    }

    fn parse_precedence(&mut self, prec: u8) {
        self.advance();
        if let Some(ref token) = self.previous {
            let prefix = get_parse_rule(token.token_type);
            if let None = prefix.prefix_rule {
                self.error(token, "Expected Expression");
                self.panic_error = true;
                return;
            }

            // Only used for this instance, so it is fine to unwrap.
            self.call_rule(prefix.prefix_rule.unwrap());
        }

        // grab_<>_token() is to handle borrow checker
        // I can't take thw values as the rules will need to use them.
        while prec <= get_parse_rule(self.grab_curr_token().unwrap()).precedence {
            self.advance();
            let infix = get_parse_rule(self.grab_prev_token().unwrap());
            if let Some(infix_rule) = infix.infix_rule {
                self.call_rule(infix_rule);
            }
        }
    }

    fn expression(&mut self) {
        // The highest precedence
        self.parse_precedence(PRECEDENCE.assignment);
    }

    fn binary(&mut self) {
        if let Some(ref token) = self.previous {
            let operator = token.token_type;

            let rule = get_parse_rule(operator);
            // The numbers would be in the values table already after this.
            self.parse_precedence(rule.precedence);

            match operator {
                TokenType::Plus => self.emit_opcode(OpCode::OpAdd),
                TokenType::Minus => self.emit_opcode(OpCode::OpSubtract),
                TokenType::Star => self.emit_opcode(OpCode::OpMultiply),
                TokenType::Slash => self.emit_opcode(OpCode::OpDivide),
                _ => unreachable!(),
            }
        }
    }

    fn group(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expected ) here.");
    }

    fn unary(&mut self) {
        if let Some(ref token) = self.previous {
            let token_type = token.token_type;

            // Will emit the OpCode inside.
            // So we have left ordering rather than right.
            self.parse_precedence(PRECEDENCE.unary + 1);

            match token_type {
                TokenType::Minus => self.emit_opcode(OpCode::OpNegate),
                _ => {}
            }
        }
    }

    fn number(&mut self) {
        if let Some(token) = &self.previous {
            let number: f64 = token.lexeme.parse().expect("Not a number");
            let idx = self.chunk.add_value(Value::Number(number));
            self.chunk
                .write_code(OpCode::OpConstant(idx as u8), token.line);
        }
    }

    fn emit_opcode(&mut self, code: OpCode) {
        if let Some(ref token) = self.previous {
            // Potential Error in the future here, I'm referencing self.chunk rather than getting
            // chunk, is there a potential error? Self.chunk is current chunk...
            self.chunk.write_code(code, token.line);
        }
    }

    fn emit_return(&mut self) {
        self.emit_opcode(OpCode::OpReturn)
    }

    // The key is to ignore errors resulting from the first error. We would do that but I don't
    // want to risk messing things up so I won't add Result for now.

    //fn advance(&mut self) -> Result<(), CompileError> {
    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            if let Some(ref mut scanner) = self.scanner {
                self.current = Some(scanner.scan_token());
                if let Some(token) = &self.current {
                    if token.token_type != TokenType::Error {
                        break;
                    }

                    self.error(token, "You got some dogshit symbols");
                    self.panic_error = true;
                    self.has_error = true;
                }
            }
        }
    }

    //fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), CompileError> {
    fn consume(&mut self, token_type: TokenType, message: &str) {
        if let Some(token) = &self.current {
            if token.token_type == token_type {
                self.advance();
                return;
            }

            self.error(token, message);
            self.panic_error = true;
        }
    }

    //fn error(&self, opt_token: &Option<Token>, message: &str) -> Result<(), CompileError> {
    fn error(&self, token: &Token, message: &str) {
        if self.panic_error {
            return;
        }
        print!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            print!(" at end of file");
        } else if token.token_type == TokenType::Error {
            // The message would be passed?
            // But don't we still want to display the Token??
        } else {
            print!(" at {}", token);
        }

        println!(": {message}");
    }

    fn call_rule(&mut self, parse_fn: ParseFn) {
        match parse_fn {
            ParseFn::Unary => self.unary(),
            ParseFn::Number => self.number(),
            ParseFn::Grouping => self.group(),
            ParseFn::Expression => self.expression(),
            ParseFn::Binary => self.binary(),
        };
    }

    fn grab_curr_token(&self) -> Option<TokenType> {
        if let Some(ref token) = self.current {
            return Some(token.token_type);
        }
        None
    }

    fn grab_prev_token(&self) -> Option<TokenType> {
        if let Some(ref token) = self.previous {
            return Some(token.token_type);
        }
        None
    }
}
