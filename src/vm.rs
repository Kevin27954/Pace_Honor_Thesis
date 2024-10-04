use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    compiler::{
        chunk::OpCode,
        values::{FunctionObj, NativeFn, Obj, Value},
        Parser,
    },
    debug::disaseemble_code,
    native_functions::get_all_natives,
};

static DEBUG: bool = false;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct CallFrame {
    // Rc cause 2 places need this value, but non modify it
    function: Rc<FunctionObj>,
    ic: usize,
    // This is just an index
    slots: usize,
}

pub struct VM {
    frame: Vec<CallFrame>,
    frame_count: usize,

    stack: Vec<Value>,

    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            frame: Vec::new(),
            frame_count: 0,

            globals: HashMap::new(),

            stack: Vec::new(),
        };

        vm.insert_natives(get_all_natives());

        vm
    }

    pub fn interpret(&mut self, source: String) -> Result<Value, InterpretError> {
        let mut parser = Parser::new();
        let parser_res = parser.compile(source);

        if let Some(function_obj) = parser_res {
            let function = Rc::new(function_obj);

            self.stack
                .push(Value::Obj(Obj::Function(Rc::clone(&function))));

            self.add_call_frame(function, 0);
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
                for i in 0..self.stack.len() - 1 {
                    print!("{}, ", self.stack[i]);
                }
                print!("{}", self.stack[self.stack.len() - 1]);
                println!("]");
                let frame = self.get_frame();
                disaseemble_code(&frame.function.chunk, frame.ic);
            }

            match self.get_op_code() {
                Some(instruction) => {
                    match instruction {
                        OpCode::OpReturn => {
                            let value = self.pop_stack();

                            match value {
                                // Doesn't Support closures
                                Value::Obj(Obj::Function(_)) => {
                                    self.runtime_error("Can't return functions");
                                    return Err(InterpretError::RuntimeError);
                                }
                                _ => {}
                            };

                            if self.frame_count == 1 {
                                self.stack.clear();
                                //while self.stack.len() > self.get_frame().slots {
                                //    self.pop_stack();
                                //}
                                if DEBUG {
                                    println!("Stack:       {:?}", self.stack);
                                }
                                return Ok(Value::None);
                            }

                            while self.stack.len() > self.get_frame().slots {
                                self.pop_stack();
                            }

                            self.frame.pop();
                            self.frame_count -= 1;
                            self.push_stack(value);
                        }
                        OpCode::OpPop => {
                            self.pop_stack();
                        }
                        OpCode::OpCall(args_count) => match self.peek_stack(args_count as usize) {
                            Value::Obj(Obj::Function(func)) => {
                                self.add_call_frame(func, args_count as usize);
                            }
                            Value::Obj(Obj::NativeFn(func)) => {
                                let start = self.stack.len() - args_count as usize;

                                // Check this
                                if &func.name != "print" && func.arity != args_count {
                                    self.runtime_error(
                                        format!(
                                            "{} Expected {} arguments but got {}",
                                            func.name, func.arity, args_count
                                        )
                                        .as_str(),
                                    );

                                    return Err(InterpretError::RuntimeError);
                                }

                                let args: Vec<Value> = self.stack.drain(start..).collect();

                                let value_res = (func.native_fn)(args_count as usize, &args);

                                // Pops the function out from the stack
                                self.pop_stack();

                                match value_res {
                                    Ok(value) => self.push_stack(value),
                                    Err(msg) => {
                                        self.runtime_error(msg);
                                        return Err(InterpretError::RuntimeError);
                                    }
                                }
                            }
                            _ => {
                                self.runtime_error("Can only call Functions");
                                return Err(InterpretError::RuntimeError);
                            }
                        },

                        OpCode::OpJumpIfFalse(jump) => {
                            if self.is_falsey(self.peek_stack(0)) {
                                let frame = self.get_mut_frame();
                                frame.ic += jump as usize;
                            }
                        }
                        OpCode::OpJump(jump) => {
                            let frame = self.get_mut_frame();
                            frame.ic += jump as usize;
                        }
                        OpCode::OpLoop(loop_start) => {
                            let frame = self.get_mut_frame();
                            frame.ic -= loop_start as usize;
                        }

                        OpCode::OpConstant(idx) => {
                            let frame = self.get_frame();
                            self.push_stack(frame.function.chunk.get_const(idx));
                        }
                        OpCode::OpDefineGlobal(idx) => {
                            let frame = self.get_frame();
                            if let Value::Obj(Obj::String(var_name)) =
                                frame.function.chunk.get_const(idx)
                            {
                                let value = self.pop_stack();
                                let borrow_var_name: &RefCell<String> = var_name.borrow();
                                let name: String = borrow_var_name.borrow().to_string();
                                self.globals.insert(name, value);
                            }
                        }
                        OpCode::OpGetGlobal(idx) => {
                            let frame = self.get_frame();
                            if let Value::Obj(Obj::String(var_name)) =
                                frame.function.chunk.get_const(idx)
                            {
                                let borrow_var_name: &RefCell<String> = var_name.borrow();
                                let name: &String = &borrow_var_name.borrow();
                                match self.globals.get(name) {
                                    Some(value) => {
                                        self.push_stack(value.clone());
                                    }
                                    None => {
                                        self.runtime_error(
                                            format!("Undefined Variable {}", name).as_str(),
                                        );
                                        return Err(InterpretError::RuntimeError);
                                    }
                                }
                            }
                        }
                        OpCode::OpSetGlobal(idx) => {
                            let frame = self.get_frame();
                            if let Value::Obj(Obj::String(var_name)) =
                                frame.function.chunk.get_const(idx)
                            {
                                let borrow_var_name: &RefCell<String> = var_name.borrow();
                                let name: &String = &borrow_var_name.borrow();
                                if self.globals.contains_key(name) {
                                    self.globals.insert(name.to_string(), self.peek_stack(0));
                                } else {
                                    self.runtime_error(
                                        format!("Undefined Variable {}", name).as_str(),
                                    );
                                    return Err(InterpretError::RuntimeError);
                                }
                            }
                        }
                        OpCode::OpGetLocal(idx) => {
                            let frame = self.get_frame();
                            let frame_stack_idx = frame.slots + idx;

                            let value = self.stack[frame_stack_idx].clone();
                            self.push_stack(value);
                        }
                        OpCode::OpSetLocal(idx) => {
                            let frame = self.get_frame();
                            let frame_stack_idx = frame.slots + idx;

                            self.stack[frame_stack_idx] = self.peek_stack(0);
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
                                Value::Obj(Obj::String(right_rc)),
                                Value::Obj(Obj::String(left_rc)),
                            ) => {
                                let mut left_string = left_rc.borrow_mut();

                                let right_string: &RefCell<String> = right_rc.borrow();

                                left_string.reserve(right_string.borrow().len());
                                left_string.push_str(&right_string.borrow());

                                self.push_stack(Value::Obj(Obj::String(left_rc.clone())))
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
                    }
                }
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
            Value::Obj(value_obj) => match value_obj {
                Obj::String(string) => {
                    let str: &RefCell<String> = string.borrow();
                    self.runtime_error(
                        format!(
                            "{} not supported on string value: {}",
                            operator,
                            str.borrow()
                        )
                        .as_str(),
                    );
                    return Err(InterpretError::RuntimeError);
                }
                _ => {
                    self.runtime_error("{} not supported on Functions");
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
            Value::Obj(value_obj) => match value_obj {
                Obj::String(string) => {
                    let str: &RefCell<String> = string.borrow();
                    self.runtime_error(
                        format!(
                            "{} not supported on string value: {}",
                            operator,
                            str.borrow()
                        )
                        .as_str(),
                    );
                    return Err(InterpretError::RuntimeError);
                }
                _ => {
                    self.runtime_error("{} not supported on Functions");
                    return Err(InterpretError::RuntimeError);
                }
            },
        };

        match operator {
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
            Value::Boolean(true) | Value::Number(_) | Value::Obj(_) => false,
        }
    }

    fn add_call_frame(&mut self, function_obj: Rc<FunctionObj>, arg_count: usize) {
        let temp_func_obj = function_obj.as_ref();

        if arg_count != temp_func_obj.arity as usize {
            self.runtime_error(
                format!(
                    "Expected {} arguments but got {}",
                    temp_func_obj.arity, arg_count
                )
                .as_str(),
            );
            return;
        }

        let callframe = CallFrame {
            function: function_obj,
            ic: 0,
            slots: self.stack.len() - arg_count - 1,
        };

        self.frame.push(callframe);
        self.frame_count += 1;
    }

    fn runtime_error(&self, message: &str) {
        eprintln!("> Program Start");
        for i in 0..self.frame_count - 1 {
            let instruction = self.frame[i].ic - 1;
            eprint!(
                "| [line {}] in ",
                self.frame[i].function.chunk.line[instruction]
            );

            eprintln!("{}", self.frame[i].function);
        }

        let instruction = self.frame[self.frame_count - 1].ic - 1;
        eprintln!("> Error Occured Here:");
        eprint!(
            "| [line {}] in ",
            self.frame[self.frame_count - 1].function.chunk.line[instruction]
        );

        eprint!("{}: ", self.frame[self.frame_count - 1].function);
        eprintln!("{}\n", message);
    }

    fn insert_natives(&mut self, natives: Vec<NativeFn>) {
        for native_fn in natives {
            self.define_native_fn(native_fn);
        }
    }

    fn define_native_fn(&mut self, native_fn: NativeFn) {
        let native_fn_name = native_fn.name.clone();
        let native_fn_val = Value::Obj(Obj::NativeFn(native_fn));
        self.push_stack(native_fn_val);

        let native_fn_val = self.pop_stack();
        self.globals.insert(native_fn_name, native_fn_val);
    }

    fn get_op_code(&mut self) -> Option<OpCode> {
        if let Some(frame) = self.frame.get_mut(self.frame_count - 1) {
            let code = frame.function.chunk.code[frame.ic];
            frame.ic += 1;
            return Some(code);
        }

        None
    }

    #[inline]
    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    #[inline]
    fn peek_stack(&self, idx: usize) -> Value {
        self.stack[self.stack.len() - 1 - idx].clone()
    }

    #[inline]
    fn pop_stack(&mut self) -> Value {
        match self.stack.pop() {
            Some(val) => val,
            None => panic!("Attempted to pop empty stack."),
        }
    }
}
