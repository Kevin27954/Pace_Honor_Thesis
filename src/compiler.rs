use chunk::{Chunk, OpCode};
use values::{Value, ValueObj};

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

struct Local {
    name: Token,
    depth: LocalState,
}

#[derive(PartialEq)]
enum LocalState {
    Uninit,
    // The depth
    Init(usize),
}

struct Compiler {
    locals: Vec<Local>,
    local_count: usize,
    scope_depth: usize,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }
}

pub struct Parser<'a> {
    previous: Option<Token>,
    current: Option<Token>,

    // Resolves the variables scope
    compiler: Compiler,

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

            compiler: Compiler::new(),

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

        self.skip_empty_line();
        while !self.match_token_type(TokenType::EOF) {
            // Because we use '\n' as the terminator, we need to care extra about empty random new
            // lines.
            self.declaration();
            self.skip_empty_line();
        }

        if self.has_error {
            disassemble_chunk(self.chunk, "Parser Errors".to_string());
        }

        !self.has_error
    }

    fn declaration(&mut self) {
        if self.match_token_type(TokenType::Let) {
            self.var_decl();
        } else if self.match_token_type(TokenType::Do) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.statement();
        }

        if self.panic_error {
            self.synchronize();
        }
    }

    // ****************************     Delcarations     ***************************

    fn block(&mut self) {
        let mut curr_token_type = self.grab_curr_token_type().unwrap();
        self.skip_empty_line();
        while curr_token_type != TokenType::End && curr_token_type != TokenType::EOF {
            self.declaration();
            self.skip_empty_line();
            curr_token_type = self.grab_curr_token_type().unwrap();
        }

        self.consume(TokenType::End, "Expected Closing End keyword here");
    }

    fn var_decl(&mut self) {
        let idx = self.parse_variable();

        if self.match_token_type(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_opcode(OpCode::OpNone);
        }

        self.consume(TokenType::NewLine, "Expected New Line after Expression");

        self.define_var(idx);
    }

    fn statement(&mut self) {
        self.expression_stmt();
    }

    // ****************************     Statements     ***************************

    fn expression_stmt(&mut self) {
        self.expression();
        self.consume(TokenType::NewLine, "Expected New Line after Expression");
        // Pop afterwards cause no one is able to use when it finishes computing. No one SHOULD be
        // ablt to use it either.
        self.emit_opcode(OpCode::OpPop);
    }

    fn parse_precedence(&mut self, prec: u8) {
        self.advance();
        let can_assign = prec <= PRECEDENCE.assignment;
        let mut temp_token: Option<Token> = None;

        if let Some(ref token) = self.previous {
            let prefix = get_parse_rule(token.token_type);
            if let None = prefix.prefix_rule {
                self.error(token, "Expected Expression");
                self.panic_error = true;
                self.has_error = true;
                return;
            }

            temp_token = Some(token.clone());
            // Only used for this instance, so it is fine to unwrap.
            self.call_rule(prefix.prefix_rule.unwrap(), can_assign);
        }

        if let Some(token) = temp_token {
            if can_assign && self.match_token_type(TokenType::Equal) {
                self.error(&token.clone(), "Invalid Assignemnt");
            }
        }

        // grab_<>_token_type() is to handle borrow checker
        // I can't take thw values as the rules will need to use them.
        while prec <= get_parse_rule(self.grab_curr_token_type().unwrap()).precedence {
            self.advance();
            let infix = get_parse_rule(self.grab_prev_token_type().unwrap());
            if let Some(infix_rule) = infix.infix_rule {
                self.call_rule(infix_rule, can_assign);
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

                TokenType::EqualEqual => self.emit_opcode(OpCode::OpEqual),
                TokenType::BangEqual => {
                    self.emit_opcode(OpCode::OpEqual);
                    self.emit_opcode(OpCode::OpNot);
                }
                TokenType::Greater => self.emit_opcode(OpCode::OpGreater),
                TokenType::GreaterEqual => {
                    self.emit_opcode(OpCode::OpLess);
                    self.emit_opcode(OpCode::OpNot);
                }
                TokenType::Less => self.emit_opcode(OpCode::OpLess),
                TokenType::LessEqual => {
                    self.emit_opcode(OpCode::OpGreater);
                    self.emit_opcode(OpCode::OpNot);
                }
                _ => unreachable!(),
            }
        }
    }

    fn literal(&mut self) {
        if let Some(ref token) = self.previous {
            match token.token_type {
                TokenType::False => self.emit_opcode(OpCode::OpFalse),
                TokenType::True => self.emit_opcode(OpCode::OpTrue),
                TokenType::None => self.emit_opcode(OpCode::OpNone),
                _ => {}
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
                TokenType::Bang => self.emit_opcode(OpCode::OpNot),
                _ => {}
            }
        }
    }

    fn number(&mut self) {
        if let Some(ref token) = self.previous {
            let number: f64 = token.lexeme.parse().expect("Not a number");
            let idx = self.add_value(Value::Number(number));
            self.emit_opcode(OpCode::OpConstant(idx as u8));
        }
    }

    fn string(&mut self) {
        if let Some(ref token) = self.previous {
            // TODO consider using str if it doens't need to be mutated
            let clean_str = &token.lexeme[1..token.lexeme.len() - 1];
            let idx = self.add_value(Value::ValueObj(ValueObj::String(Box::new(
                // This clones the string when converting &str to String
                clean_str.to_string(),
            ))));

            self.emit_opcode(OpCode::OpConstant(idx as u8));
        }
    }

    fn variable(&mut self, can_assign: bool) {
        self.name_variable(can_assign);
    }

    // ****************************     Helpers     ***************************

    fn name_variable(&mut self, can_assign: bool) {
        if let Some(ref token) = self.previous {
            let op_get_code: OpCode;
            let op_set_code: OpCode;

            let idx = self.resolve_local(token);
            // Global
            if idx == -1 {
                let idx = self.make_identifier_constant(token.clone());
                op_get_code = OpCode::OpGetGlobal(idx as u8);
                op_set_code = OpCode::OpSetGlobal(idx as u8);
            } else {
                op_get_code = OpCode::OpGetLocal(idx as u8);
                op_set_code = OpCode::OpSetLocal(idx as u8);
            }

            if can_assign && self.match_token_type(TokenType::Equal) {
                self.expression();
                self.emit_opcode(op_set_code);
            } else {
                self.emit_opcode(op_get_code);
            }
        }
    }

    fn define_var(&mut self, idx: usize) {
        if self.compiler.scope_depth > 0 {
            self.compiler.locals[self.compiler.local_count - 1].depth =
                LocalState::Init(self.compiler.scope_depth);
            return;
        }

        self.emit_opcode(OpCode::OpDefineGlobal(idx as u8))
    }

    // TODO Look into a way to return usize without doing clone().unwrap()?
    fn parse_variable(&mut self) -> usize {
        self.consume(TokenType::Identifier, "Expected an Identifier name here");

        self.declare_var();
        if self.compiler.scope_depth > 0 {
            return 0;
        }

        let token = self.previous.clone().unwrap();
        self.make_identifier_constant(token)
    }

    fn make_identifier_constant(&mut self, token: Token) -> usize {
        //let token = self.previous.take().unwrap();
        self.add_value(Value::ValueObj(ValueObj::String(Box::new(token.lexeme))))
    }

    // Only for local varables
    fn declare_var(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }

        if let Some(ref token) = self.previous {
            for i in (0..self.compiler.locals.len()).rev() {
                let local = &self.compiler.locals[i];
                match local.depth {
                    LocalState::Init(depth)
                        if local.depth != LocalState::Uninit
                            && depth < self.compiler.scope_depth =>
                    {
                        break
                    }
                    _ => {}
                }
                //if local.depth != State::Uninit && local.depth < self.compiler.scope_depth {
                //    return;
                //}

                if self.is_eq_token_name(token, &local.name) {
                    self.error(
                        token,
                        format!("Variable {} already exist in this scope", token.lexeme).as_str(),
                    )
                }
            }

            self.add_local(token.clone());
        }
    }

    fn resolve_local(&self, name: &Token) -> i32 {
        for i in (0..self.compiler.locals.len()).rev() {
            let local = &self.compiler.locals[i];
            if self.is_eq_token_name(name, &local.name) {
                if local.depth == LocalState::Uninit {
                    self.error(name, "Can't read local variable in it's own init field.");
                }
                return i as i32;
            }
        }

        -1
    }

    fn add_local(&mut self, token: Token) {
        let local = Local {
            name: token,
            depth: LocalState::Uninit,
        };

        self.compiler.local_count += 1;
        self.compiler.locals.push(local);
    }

    fn add_value(&mut self, value: Value) -> usize {
        self.chunk.add_value(value)
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

    fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while let Some(local) = self.compiler.locals.last() {
            //if local.depth <= self.compiler.scope_depth {
            //    break;
            //}
            match local.depth {
                LocalState::Init(depth) if depth <= self.compiler.scope_depth => break,
                _ => {}
            }

            self.compiler.locals.pop();
            self.emit_opcode(OpCode::OpPop);
            self.compiler.local_count -= 1;
        }
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
            self.has_error = true;
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

    fn synchronize(&mut self) {
        self.panic_error = false;

        while self.grab_curr_token_type().unwrap() != TokenType::EOF {
            if self.grab_prev_token_type().unwrap() == TokenType::NewLine {
                return;
            }

            match self.grab_curr_token_type().unwrap() {
                TokenType::If
                | TokenType::Let
                | TokenType::Function
                | TokenType::Struct
                | TokenType::Return
                | TokenType::For
                | TokenType::While => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn call_rule(&mut self, parse_fn: ParseFn, can_assign: bool) {
        match parse_fn {
            ParseFn::Unary => self.unary(),
            ParseFn::Number => self.number(),
            ParseFn::Grouping => self.group(),
            ParseFn::Binary => self.binary(),
            ParseFn::Literal => self.literal(),
            ParseFn::String => self.string(),
            ParseFn::Variable => self.variable(can_assign),
        };
    }

    fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if self.grab_curr_token_type().unwrap() != token_type {
            return false;
        }
        self.advance();
        true
    }

    fn is_eq_token_name(&self, left: &Token, right: &Token) -> bool {
        if left.lexeme.len() != right.lexeme.len() {
            return false;
        }

        left.lexeme == right.lexeme
    }

    fn grab_curr_token_type(&self) -> Option<TokenType> {
        if let Some(ref token) = self.current {
            return Some(token.token_type);
        }
        None
    }

    fn grab_prev_token_type(&self) -> Option<TokenType> {
        if let Some(ref token) = self.previous {
            return Some(token.token_type);
        }
        None
    }

    fn skip_empty_line(&mut self) {
        // We look at curr instead of prev because the functions will advance() later. So curr will
        // be the token that we start parsing on.
        while self.grab_curr_token_type().unwrap() == TokenType::NewLine {
            self.advance();
        }
    }
}
