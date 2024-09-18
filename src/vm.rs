use core::panic;
use std::collections::HashMap;

use crate::{
    compiler::{
        chunk::{Chunk, OpCode},
        values::{FunctionObj, Value, ValueObj},
        Parser,
    },
    debug::disaseemble_code,
};

static DEBUG: bool = true;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct CallFrame {
    // Might want this to be a ref, we won't be modifying it
    function: FunctionObj,
    ic: usize,
    // This is just an index
    slots: usize,
}

pub struct VM {
    frame: Vec<CallFrame>,
    frame_count: usize,

    //chunk: Chunk,
    stack: Vec<Value>,

    globals: HashMap<String, Value>,
    //ic: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            // Here just to have a field. Will be replaced in interpret
            //chunk,
            frame: Vec::new(),
            frame_count: 0,

            globals: HashMap::new(),

            stack: Vec::new(),
            //ic: 0,
        }
    }

    pub fn interpret(&mut self, source: String) -> Result<Value, InterpretError> {
        // Idea: Given a source code, compile and run it.
        let mut chunk = Chunk::new();

        let mut parser = Parser::new(&mut chunk);
        let parser_res = parser.compile(source);

        if let Some(function_obj) = parser_res {
            //
            // Consider without clone also.
            //
            self.stack.push(Value::ValueObj(ValueObj::Function(Box::new(
                function_obj.clone(),
            ))));

            let frame = CallFrame {
                function: function_obj.clone(),
                ic: 0,
                slots: 0,
            };

            self.frame.push(frame);
            self.frame_count += 1;
        } else {
            return Err(InterpretError::CompileError);
        }

        Ok(self.run()?)
    }

    fn get_mut_frame(&mut self) -> &mut CallFrame {
        &mut self.frame[self.frame_count - 1]
    }

    fn get_frame(&self) -> &CallFrame {
        &self.frame[self.frame_count - 1]
    }

    fn run(&mut self) -> Result<Value, InterpretError> {
        println!("\n=== VM ===");
        loop {
            if DEBUG {
                print!("Stack:       [");
                for value in &self.stack {
                    print!("{}, ", value);
                }
                println!("]");
                let frame = self.get_frame();
                //let ic = frame.ic - frame.function.chunk.code.len();
                disaseemble_code(&frame.function.chunk, frame.ic);
            }

            match self.get_op_code() {
                Some(instruction) => match instruction {
                    OpCode::OpReturn => {}
                    OpCode::OpPop => {
                        let value = self.pop_stack();
                        println!("{}", value);
                    }

                    OpCode::OpJumpIfFalse(jump) => {
                        //if self.is_falsey(self.peek_stack(0)) {
                        if self.is_falsey(self.peek_stack(0)) {
                            let frame = self.get_mut_frame();
                            frame.ic += jump as usize;
                        }
                    }
                    OpCode::OpJump(jump) => {
                        let frame = self.get_mut_frame();
                        frame.ic += jump as usize;
                        //self.ic += jump as usize;
                    }
                    OpCode::OpLoop(loop_start) => {
                        //self.ic -= loop_start as usize;
                        let frame = self.get_mut_frame();
                        frame.ic -= loop_start as usize;
                    }

                    OpCode::OpConstant(idx) => {
                        let frame = self.get_frame();
                        self.push_stack(frame.function.chunk.get_const(idx));
                    }
                    OpCode::OpDefineGlobal(idx) => {
                        let frame = self.get_frame();
                        if let Value::ValueObj(ValueObj::String(var_name)) =
                            frame.function.chunk.get_const(idx)
                        {
                            // self.globals_vec.get(var_name.counter);
                            let value = self.pop_stack();
                            self.globals.insert(var_name.to_string(), value);
                        }
                    }
                    OpCode::OpGetGlobal(idx) => {
                        let frame = self.get_frame();
                        if let Value::ValueObj(ValueObj::String(var_name)) =
                            frame.function.chunk.get_const(idx)
                        {
                            match self.globals.get(var_name.as_ref()) {
                                Some(value) => {
                                    self.push_stack(value.clone());
                                }
                                None => {
                                    self.runtime_error(
                                        format!("Undefined Variable {}", var_name).as_str(),
                                    );
                                    return Err(InterpretError::RuntimeError);
                                }
                            }
                        }
                    }
                    OpCode::OpSetGlobal(idx) => {
                        let frame = self.get_frame();
                        if let Value::ValueObj(ValueObj::String(var_name)) =
                            frame.function.chunk.get_const(idx)
                        {
                            if self.globals.contains_key(var_name.as_ref()) {
                                self.globals
                                    .insert(var_name.to_string(), self.peek_stack(0));
                            } else {
                                self.runtime_error(
                                    format!("Undefined Variable {}", var_name).as_str(),
                                );
                                return Err(InterpretError::RuntimeError);
                            }
                        }
                    }
                    OpCode::OpGetLocal(idx) => {
                        //let value = self.stack[idx as usize].clone();
                        let frame = self.get_frame();
                        let maybe = idx as usize + frame.ic;
                        //let value = frame.slots[idx as usize];
                        let value = self.stack[maybe].clone();
                        self.push_stack(value);
                    }
                    OpCode::OpSetLocal(idx) => {
                        //self.stack[idx as usize] = self.peek_stack(0);
                        let frame = self.get_frame();
                        let maybe = idx as usize + frame.ic;
                        self.stack[maybe] = self.peek_stack(0);
                    }

                    OpCode::OpNegate => {
                        match self.peek_stack(0) {
                            Value::Number(_) => {}
                            _ => {
                                self.runtime_error(
                                    "Can't perform - (negate) operator on non number",
                                );
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
                            Value::ValueObj(ValueObj::String(mut left_string)),
                        ) => {
                            // Modifies in place.
                            let res = left_string.as_mut();
                            // Reserves ahead of time.
                            res.reserve(right_string.len());
                            res.push_str(right_string.as_str());
                            self.push_stack(Value::ValueObj(ValueObj::String(left_string)))
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
                },
                // LOOP STOPPER
                None => {
                    break;
                }
            }
        }

        Ok(Value::None)
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
                ValueObj::Function(function) => {
                    unimplemented!("Function not implemented yet");
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
                ValueObj::Function(function) => {
                    unimplemented!("Function not implemented yet");
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

        let frame = &self.frame[self.frame_count - 1];

        let instruction = frame.ic - frame.function.chunk.code.len();
        // Calls the corresponding line array for the instruction
        let line = &frame.function.chunk.line[instruction];
        //eprintln!("[line {}]: {}", line, message);
        eprintln!("[line {}]: {}", line, message);
    }

    fn get_op_code(&mut self) -> Option<OpCode> {
        //let frame = &mut self.frame[self.frame_count - 1];
        if let Some(frame) = self.frame.get_mut(self.frame_count - 1) {
            if frame.ic >= frame.function.chunk.code.len() {
                return None;
            }

            let code = frame.function.chunk.code[frame.ic];
            frame.ic += 1;
            return Some(code);
        }

        None
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    // TODO We might need a mut ref on one of them
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
