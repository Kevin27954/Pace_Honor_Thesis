use std::{cell::RefCell, mem, rc::Rc};

use chunk::{Chunk, OpCode};
use values::{FunctionObj, Obj, Value};

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

enum FunctionType {
    FunctionType,
    ScriptType,
}

struct Compiler {
    function: FunctionObj,
    function_type: FunctionType,

    locals: Vec<Local>,
    local_count: usize,
    scope_depth: usize,
}

impl Compiler {
    fn new(function_type: FunctionType) -> Self {
        // Holds the function
        let first_idx_holder = Local {
            name: Token {
                line: 0,
                lexeme: "".to_string(),
                token_type: TokenType::None,
            },
            depth: LocalState::Init(0),
        };

        let mut locals: Vec<Local> = Vec::new();
        locals.push(first_idx_holder);

        Compiler {
            function: FunctionObj::new(),
            function_type,

            locals,
            local_count: 1,
            scope_depth: 0,
        }
    }
}

pub struct Parser {
    previous: Option<Token>,
    current: Option<Token>,

    // Resolves the variables scope
    compiler: Compiler,

    scanner: Option<Scanner>,

    pub has_error: bool,
    // Can possibly replace with Result/Option type
    panic_error: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            // Inital state is None, All subsequent values are not null.
            // Current and Peek/Next, makes more sense so far
            previous: None,
            current: None,

            compiler: Compiler::new(FunctionType::ScriptType),

            scanner: None,

            has_error: false,
            panic_error: false,
        }
    }

    //pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
    pub fn compile(&mut self, source: String) -> Option<FunctionObj> {
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
            // From here on out, we will treat the global scope as "main()"
            if let Some(function_name) = self.compiler.function.name.clone() {
                disassemble_chunk(self.current_chunk(), function_name);
            } else {
                disassemble_chunk(self.current_chunk(), "<script>".to_string());
            }
        }

        self.emit_opcode(OpCode::OpNone);
        self.emit_opcode(OpCode::OpReturn);

        // Now inside vm.interpret, we no longer need to .clone(). Saves computation and memory
        let function_obj = mem::replace(
            &mut self.compiler.function,
            FunctionObj {
                arity: 0,
                chunk: Chunk::new(),
                name: Some(String::new()),
            },
        );

        match self.has_error {
            true => None,
            false => Some(function_obj),
        }
    }

    fn declaration(&mut self) {
        if self.match_token_type(TokenType::Function) {
            self.fn_decl();
        } else if self.match_token_type(TokenType::Let) {
            self.var_decl();
        } else {
            self.statement();
        }

        if self.panic_error {
            self.synchronize();
        }
    }

    // ****************************     Delcarations     ***************************

    fn fn_decl(&mut self) {
        let idx = self.parse_variable();
        let fn_name = self.previous.clone().unwrap();

        if self.compiler.scope_depth > 0 {
            self.compiler.locals[self.compiler.local_count - 1].depth =
                LocalState::Init(self.compiler.scope_depth);
        }

        self.function(FunctionType::FunctionType, fn_name.lexeme);

        self.define_var(idx);
    }

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

    fn function(&mut self, function_type: FunctionType, fn_name: String) {
        let user_fn_obj = FunctionObj {
            arity: 0,
            chunk: Chunk::new(),
            name: Some(fn_name),
        };

        let mut func_compiler = Compiler::new(function_type);
        func_compiler.function = user_fn_obj;

        // Stores the original Compiler, and sets a new compiler to fill.
        let main_fn_compiler = mem::replace(&mut self.compiler, func_compiler);
        self.begin_scope();

        self.consume(TokenType::LeftParen, "Expected '(' after function name");

        let curr_token_type = self.grab_curr_token_type().unwrap();
        if curr_token_type != TokenType::RightParen {
            loop {
                self.compiler.function.arity += 1;

                let idx = self.parse_variable();
                self.define_var(idx);

                if !self.match_token_type(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters");
        self.consume(TokenType::Do, "Expected 'do' before function body");

        self.block();

        // Why don't we need to end scope?
        // Because the compiler ends it's own scope once it gets removed from parser.

        self.emit_opcode(OpCode::OpNone);
        self.emit_opcode(OpCode::OpReturn);

        // Stores the Function Compiler, and sets originl back in place.
        let user_fn_obj = mem::replace(&mut self.compiler, main_fn_compiler);
        let user_fn = Value::Obj(Obj::Function(Rc::new(user_fn_obj.function)));

        let idx = self.add_value(user_fn);
        self.emit_opcode(OpCode::OpConstant(idx));
    }

    fn if_block(&mut self) {
        self.skip_empty_line();

        let mut curr_token_type = self.grab_curr_token_type().unwrap();
        self.skip_empty_line();
        while curr_token_type != TokenType::End
            && curr_token_type != TokenType::Else
            && curr_token_type != TokenType::EOF
        {
            self.declaration();
            self.skip_empty_line();
            curr_token_type = self.grab_curr_token_type().unwrap();
        }

        match curr_token_type {
            TokenType::Else => {}
            TokenType::End => {
                self.advance();
            }
            _ => {
                self.consume(TokenType::End, "Expected Closing End keyword here");
            }
        }
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
        if self.match_token_type(TokenType::If) {
            self.if_stmt();
        } else if self.match_token_type(TokenType::Do) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else if self.match_token_type(TokenType::Return) {
            self.return_stmt();
        } else if self.match_token_type(TokenType::While) {
            self.while_stmt();
        } else if self.match_token_type(TokenType::For) {
            self.for_stmt();
        } else {
            self.expression_stmt();
        }
    }

    // ****************************     Statements     ***************************

    fn return_stmt(&mut self) {
        match self.compiler.function_type {
            FunctionType::ScriptType => match self.previous {
                Some(ref token) => self.error(token, "Can't have return values at top level"),
                _ => {}
            },
            _ => {}
        }

        if self.match_token_type(TokenType::NewLine) {
            self.emit_opcode(OpCode::OpNone);
            self.emit_opcode(OpCode::OpReturn);
        } else {
            self.expression();
            self.consume(TokenType::NewLine, "Expected New Line after Return Value");
            self.emit_opcode(OpCode::OpReturn);
        }
    }

    fn for_stmt(&mut self) {
        self.begin_scope();

        if self.match_token_type(TokenType::Comma) {
        } else if self.match_token_type(TokenType::Let) {
            let idx = self.parse_variable();
            if self.match_token_type(TokenType::Equal) {
                self.expression();
            } else {
                self.emit_opcode(OpCode::OpNone);
            }

            self.consume(TokenType::Comma, "Expected Comma seperator here");
            self.define_var(idx);
        } else {
            self.expression();
            self.consume(TokenType::Comma, "Expected Comma seperator here");
        }

        let mut loop_start = self.current_chunk().code.len();

        // 0 should be fine since there shouldn't be a possibliity that emit_jump_code returns a
        //   number equal to 0
        let mut jumps = 0;
        // The Condition
        if !self.match_token_type(TokenType::Comma) {
            self.expression();
            self.consume(TokenType::Comma, "Expected Comma serperator here");

            jumps = self.emit_jump_code(OpCode::OpJumpIfFalse(255));
            self.emit_opcode(OpCode::OpPop);
        }

        // The Increment
        let curr_token_type = self.grab_curr_token_type().unwrap();
        if curr_token_type != TokenType::Do {
            let body_jump = self.emit_jump_code(OpCode::OpJump(255));
            let increment_start = self.current_chunk().code.len();
            self.expression();
            self.emit_opcode(OpCode::OpPop);

            let loop_offset = self.current_chunk().code.len() - loop_start + 1;
            self.emit_opcode(OpCode::OpLoop(loop_offset as u8));

            loop_start = increment_start;
            self.patch_jump_code(body_jump);
        }

        self.statement();

        let loop_offset = self.current_chunk().code.len() - loop_start + 1;
        self.emit_opcode(OpCode::OpLoop(loop_offset as u8));

        if jumps != 0 {
            self.patch_jump_code(jumps);
            self.emit_opcode(OpCode::OpPop);
        }

        self.end_scope();
    }

    fn while_stmt(&mut self) {
        let loop_start = self.current_chunk().code.len();
        self.expression();

        let offset = self.emit_jump_code(OpCode::OpJumpIfFalse(255));
        self.emit_opcode(OpCode::OpPop);

        self.statement();
        let loop_offset = self.current_chunk().code.len() - loop_start + 1;
        self.emit_opcode(OpCode::OpLoop(loop_offset as u8));

        self.patch_jump_code(offset);
        self.emit_opcode(OpCode::OpPop);
    }

    fn if_stmt(&mut self) {
        self.expression();
        self.consume(TokenType::Then, "Expected then after the condition");

        let if_jump = self.emit_jump_code(OpCode::OpJumpIfFalse(255));
        self.emit_opcode(OpCode::OpPop);

        self.parse_if_blocks();

        let else_jump = self.emit_jump_code(OpCode::OpJump(255));

        self.patch_jump_code(if_jump);
        self.emit_opcode(OpCode::OpPop);

        if self.match_token_type(TokenType::Else) {
            self.parse_if_blocks();
        }
        self.patch_jump_code(else_jump);
    }

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

    fn call(&mut self, _can_assign: bool) {
        let arg_count = self.argument_list();
        self.emit_opcode(OpCode::OpCall(arg_count));
    }

    fn parse_and(&mut self) {
        let and_jump = self.emit_jump_code(OpCode::OpJumpIfFalse(255));
        self.emit_opcode(OpCode::OpPop);
        self.parse_precedence(PRECEDENCE.and);

        self.patch_jump_code(and_jump);
    }

    fn parse_or(&mut self) {
        let if_jump = self.emit_jump_code(OpCode::OpJumpIfFalse(255));
        let else_jump = self.emit_jump_code(OpCode::OpJump(255));

        self.patch_jump_code(if_jump);
        self.emit_opcode(OpCode::OpPop);

        self.parse_precedence(PRECEDENCE.or);
        self.patch_jump_code(else_jump);
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
            self.emit_opcode(OpCode::OpConstant(idx));
        }
    }

    fn string(&mut self) {
        if let Some(ref token) = self.previous {
            // TODO consider using str if it doens't need to be mutated
            let clean_str = &token.lexeme[1..token.lexeme.len() - 1];
            let idx = self.add_value(Value::Obj(Obj::String(
                // This clones the string when converting &str to String
                Rc::new(RefCell::new(clean_str.to_string())),
            )));

            self.emit_opcode(OpCode::OpConstant(idx));
        }
    }

    fn variable(&mut self, can_assign: bool) {
        self.name_variable(can_assign);
    }

    // ****************************     Helpers     ***************************

    fn argument_list(&mut self) -> u8 {
        let mut arg_count: u8 = 0;

        let curr_token_type = self.grab_curr_token_type().unwrap();
        if curr_token_type != TokenType::RightParen {
            loop {
                self.expression();

                if arg_count == 255 {
                    if let Some(ref token) = self.previous {
                        self.error(token, "Can't have more than 255 arguments");
                    }
                }

                arg_count += 1;

                if !self.match_token_type(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after arguments");

        arg_count
    }

    fn parse_if_blocks(&mut self) {
        self.begin_scope();
        self.if_block();
        self.end_scope();
    }

    fn name_variable(&mut self, can_assign: bool) {
        if let Some(ref token) = self.previous {
            let op_get_code: OpCode;
            let op_set_code: OpCode;

            let idx = self.resolve_local(token);
            if idx == -1 {
                // Global
                let idx = self.make_identifier_constant(token.clone());
                op_get_code = OpCode::OpGetGlobal(idx as usize);
                op_set_code = OpCode::OpSetGlobal(idx as usize);
            } else {
                op_get_code = OpCode::OpGetLocal(idx as usize);
                op_set_code = OpCode::OpSetLocal(idx as usize);
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

        self.emit_opcode(OpCode::OpDefineGlobal(idx))
    }

    fn parse_variable(&mut self) -> usize {
        self.consume(TokenType::Identifier, "Expected an Identifier name here");

        // Defines the Local Variable here
        self.declare_var();

        if self.compiler.scope_depth > 0 {
            return 0;
        }

        let token = self.previous.clone().unwrap();
        self.make_identifier_constant(token)
    }

    fn make_identifier_constant(&mut self, token: Token) -> usize {
        self.add_value(Value::Obj(Obj::String(Rc::new(RefCell::new(token.lexeme)))))
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
        self.current_chunk().add_value(value)
    }

    fn emit_opcode(&mut self, code: OpCode) {
        if let Some(ref token) = self.previous.clone() {
            // Potential Error in the future here, I'm referencing self.chunk rather than getting
            // chunk, is there a potential error? Self.chunk is current chunk...
            self.current_chunk().write_code(code, token.line);
        }
    }

    fn emit_jump_code(&mut self, code: OpCode) -> usize {
        self.emit_opcode(code);
        self.current_chunk().code.len() - 1
    }

    fn patch_jump_code(&mut self, offset: usize) {
        let jumps = self.current_chunk().code.len() - offset - 1;

        match self.current_chunk().code.get_mut(offset) {
            Some(code) => match code {
                OpCode::OpJump(jump) => {
                    *jump = jumps as u8;
                }
                OpCode::OpJumpIfFalse(jump) => {
                    *jump = jumps as u8;
                }
                _ => {}
            },
            None => {}
        }
    }

    fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while let Some(local) = self.compiler.locals.last() {
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
            // This might not be wanted since Newline is considered a terminator. But it is also a
            // token that we can randomly have. This makes it difficult when it comes to
            // syncrhonization blocks of code, like Structs
            //
            //if self.grab_prev_token_type().unwrap() == TokenType::NewLine {
            //    return;
            //}

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
            ParseFn::And => self.parse_and(),
            ParseFn::Or => self.parse_or(),
            ParseFn::Call => self.call(can_assign),
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

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiler.function.chunk
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
