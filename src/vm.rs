use core::panic;

use crate::{
    compiler::{
        chunk::{Chunk, OpCode},
        values::Value,
        Parser,
    },
    debug::disaseemble_code,
};

static DEBUG: bool = true;

pub enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,

    ic: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            // Here just to have a field. Will be replaced in interpret
            chunk,

            stack: Vec::new(),
            ic: 0,
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        // Idea: Given a source code, compile and run it.
        let mut chunk = Chunk::new();

        let mut parser = Parser::new(&mut chunk);
        if !parser.compile(source) {
            return InterpretResult::CompileError;
        }

        self.chunk = chunk;

        return self.run();
    }

    fn get_op_code(&mut self) -> OpCode {
        let code = self.chunk.code[self.ic];
        self.ic += 1;
        code
    }

    fn run(&mut self) -> InterpretResult {
        println!("\n=== VM ===");
        loop {
            if DEBUG {
                println!("Stack:       {:?}", self.stack);
                disaseemble_code(&self.chunk, self.ic);
            }

            let instruction = self.get_op_code();
            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.pop_stack());
                    return InterpretResult::OK;
                }
                OpCode::OpConstant(idx) => {
                    self.push_stack(self.chunk.get_const(idx));
                }
                OpCode::OpNegate => {
                    let value = match self.pop_stack() {
                        Value::Number(number) => Value::Number(-number),
                        _ => {
                            panic!("Can't negate a non number");
                        }
                    };

                    self.push_stack(value);
                }
                OpCode::OpAdd | OpCode::OpSubtract | OpCode::OpMultiply | OpCode::OpDivide => {
                    self.binary_operators(instruction)
                }
            }
        }
    }

    fn binary_operators(&mut self, operator: OpCode) {
        let b = match self.pop_stack() {
            Value::Number(num) => num,
        };
        let a = match self.pop_stack() {
            Value::Number(num) => num,
        };

        match operator {
            OpCode::OpAdd => self.push_stack(Value::Number(a + b)),
            OpCode::OpSubtract => self.push_stack(Value::Number(a - b)),
            OpCode::OpMultiply => self.push_stack(Value::Number(a * b)),
            OpCode::OpDivide => self.push_stack(Value::Number(a / b)),
            _ => {
                panic!("{} is not a Binary Operator", operator);
            }
        }
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Value {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("Attempted to pop empty stack."),
        }
    }

    fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }
}
