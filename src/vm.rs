use core::panic;

use crate::{
    compiler::{
        chunk::{Chunk, OpCode},
        values::Value,
    },
    debug::{disaseemble_code, disassemble_chunk},
};

const DEBUG: bool = true;

pub enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,

    ic: usize,
}

// Gives the lifetime of VM struct a name we can use. Same impliciations as the VM struct in the
// struct declaration
impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk,
            stack: Vec::new(),
            ic: 0,
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
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
                disaseemble_code(self.chunk, self.ic);
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
