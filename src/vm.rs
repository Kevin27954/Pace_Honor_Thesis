use core::panic;

use crate::{
    compiler::{
        chunk::{Chunk, OpCode},
        values::{Value, ValueObj},
        Parser,
    },
    debug::{disaseemble_code, disassemble_chunk},
};

static DEBUG: bool = true;

pub enum InterpretError {
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

    pub fn interpret(&mut self, source: String) -> Result<Value, InterpretError> {
        // Idea: Given a source code, compile and run it.
        let mut chunk = Chunk::new();

        let mut parser = Parser::new(&mut chunk);
        if !parser.compile(source) {
            return Err(InterpretError::CompileError);
        }

        self.chunk = chunk;

        Ok(self.run()?)
    }

    fn run(&mut self) -> Result<Value, InterpretError> {
        println!("\n=== VM ===");
        loop {
            if DEBUG {
                println!("Stack:       {:?}", self.stack);
                disaseemble_code(&self.chunk, self.ic);
            }

            let instruction = self.get_op_code();
            match instruction {
                // For now this is the loop stopper
                OpCode::OpReturn => {
                    let value = self.pop_stack();
                    println!("{}", value);
                    return Ok(value);
                }
                OpCode::OpConstant(idx) => {
                    self.push_stack(self.chunk.get_const(idx));
                }
                OpCode::OpNegate => {
                    match self.peek_stack(0) {
                        Value::Number(_) => {}
                        _ => {
                            self.runtime_error("Can't perform - (negate) operator on non number");
                            return Err(InterpretError::RuntimeError);
                        }
                    };

                    match self.pop_stack() {
                        Value::Number(number) => self.push_stack(Value::Number(-number)),
                        _ => {}
                    };
                }
                OpCode::OpAdd => match (self.pop_stack(), self.pop_stack()) {
                    (
                        Value::ValueObj(ValueObj::String(right_string)),
                        Value::ValueObj(ValueObj::String(left_string)),
                    ) => {
                        // Took control of the ownership of left_string
                        let mut res = *left_string;
                        res.push_str(right_string.as_str());
                        self.push_stack(Value::ValueObj(ValueObj::String(Box::new(res))))
                        // Popped Box<String> are dropped after this loop is done.
                    }
                    (Value::Number(right_num), Value::Number(left_num)) => {
                        self.push_stack(Value::Number(left_num + right_num))
                    }
                    _ => {
                        self.runtime_error("Operands must be either 2 String or 2 Number");
                        return Err(InterpretError::RuntimeError);
                    }
                },
                OpCode::OpSubtract | OpCode::OpMultiply | OpCode::OpDivide => {
                    self.binary_operators(instruction)?
                }

                OpCode::OpTrue => self.push_stack(Value::Boolean(true)),
                OpCode::OpFalse => self.push_stack(Value::Boolean(false)),
                OpCode::OpNone => self.push_stack(Value::None),
                OpCode::OpNot => {
                    let value = self.pop_stack();
                    self.push_stack(Value::Boolean(self.is_falsey(value)));
                }

                OpCode::OpGreater => {
                    let right = self.pop_stack();
                    let left = self.pop_stack();

                    self.push_stack(Value::Boolean(self.is_greater(left, right)?))
                }
                OpCode::OpEqual => {
                    let right = self.pop_stack();
                    let left = self.pop_stack();

                    let boolean = left == right;
                    self.push_stack(Value::Boolean(boolean))
                }
                OpCode::OpLess => {
                    let right = self.pop_stack();
                    let left = self.pop_stack();

                    let value: bool;
                    if left == right {
                        value = false
                    } else {
                        value = !self.is_greater(left, right)?;
                    }

                    self.push_stack(Value::Boolean(value))
                }
            };
        }
    }

    fn binary_operators(&mut self, operator: OpCode) -> Result<(), InterpretError> {
        let b = match self.pop_stack() {
            Value::Number(num) => num,
            Value::Boolean(bool) => {
                self.runtime_error(
                    format!("{} not supported on boolean value: {}", operator, bool).as_str(),
                );
                return Err(InterpretError::RuntimeError);
            }
            Value::None => {
                self.runtime_error(format!("{} not supported on none value", operator).as_str());
                return Err(InterpretError::RuntimeError);
            }
            Value::ValueObj(value_obj) => match value_obj {
                ValueObj::String(string) => {
                    self.runtime_error(
                        format!("{} not supported on string value: {}", operator, string).as_str(),
                    );
                    return Err(InterpretError::RuntimeError);
                }
            },
        };

        let a = match self.pop_stack() {
            Value::Number(num) => num,
            Value::Boolean(bool) => {
                self.runtime_error(
                    format!("{} not supported on boolean value: {}", operator, bool).as_str(),
                );
                return Err(InterpretError::RuntimeError);
            }
            Value::None => {
                self.runtime_error(format!("{} not supported on none value", operator).as_str());
                return Err(InterpretError::RuntimeError);
            }
            Value::ValueObj(value_obj) => match value_obj {
                ValueObj::String(string) => {
                    self.runtime_error(
                        format!("{} not supported on string value: {}", operator, string).as_str(),
                    );
                    return Err(InterpretError::RuntimeError);
                }
            },
        };

        match operator {
            //OpCode::OpAdd => self.push_stack(Value::Number(a + b)),
            OpCode::OpSubtract => self.push_stack(Value::Number(a - b)),
            OpCode::OpMultiply => self.push_stack(Value::Number(a * b)),
            OpCode::OpDivide => self.push_stack(Value::Number(a / b)),
            _ => {
                self.runtime_error(format!("{} is not a Binary Operator", operator).as_str());
                return Err(InterpretError::RuntimeError);
            }
        }

        Ok(())
    }

    fn is_greater(&self, left: Value, right: Value) -> Result<bool, InterpretError> {
        match left {
            Value::Number(num_left) => match right {
                Value::Number(num_right) => return Ok(num_left > num_right),
                _ => {}
            },
            _ => {}
        };

        self.runtime_error(
            format!(
                "Can't perform < and > on different types: '{}' and '{}'",
                left, right
            )
            .as_str(),
        );

        Err(InterpretError::RuntimeError)
    }

    fn is_falsey(&self, value: Value) -> bool {
        match value {
            Value::None | Value::Boolean(false) => true,
            Value::Boolean(true) | Value::Number(_) | Value::ValueObj(_) => false,
        }
    }

    fn runtime_error(&self, message: &str) {
        // Get's the instruction index
        //let instruction = self.ic - self.chunk.code.len() - 1;
        let instruction = self.ic - 1;
        // Calls the corresponding line array for the instruction
        let line = &self.chunk.line[instruction];
        //eprintln!("[line {}]: {}", line, message);
        eprintln!("[line {}]: {}", line, message);
    }

    fn get_op_code(&mut self) -> OpCode {
        let code = self.chunk.code[self.ic];
        self.ic += 1;
        code
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn peek_stack(&self, idx: usize) -> Value {
        self.stack[self.stack.len() - 1 - idx].clone()
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
