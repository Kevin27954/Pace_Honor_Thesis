use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

mod gc;

use crate::{
    compiler::{
        chunk::OpCode,
        values::{FunctionObj, NativeFn, Obj, StrObj, Structs, StructsInstance, Value},
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
    function: Rc<RefCell<FunctionObj>>,
    ic: usize,
    // This is just an index
    slots: usize,
}

pub struct VM {
    frame: Vec<CallFrame>,
    frame_count: usize,

    stack: Vec<Value>,

    //stack_cap: usize,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            frame: Vec::new(),
            frame_count: 0,

            globals: HashMap::new(),

            //stack_cap: 0,
            stack: Vec::new(),
        };

        vm.insert_natives(get_all_natives());

        vm
    }

    pub fn interpret(&mut self, source: String) -> Result<Value, InterpretError> {
        let mut parser = Parser::new();
        let parser_res = parser.compile(source);

        if let Some(function_obj) = parser_res {
            let function = Rc::new(RefCell::new(function_obj));

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

        //let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
        //println!("{:?}", func.borrow().chunk.values);

        loop {
            if DEBUG {
                print!("Stack:       [");
                for i in 0..self.stack.len() - 1 {
                    print!("{}, ", self.stack[i]);
                }
                print!("{}", self.stack[self.stack.len() - 1]);
                println!("]");
                let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                disaseemble_code(&func.borrow().chunk, self.get_frame().ic);
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
                        OpCode::OpCall(args_count) => {
                            match self.peek_stack(args_count as usize) {
                                Value::Obj(Obj::Function(func)) => {
                                    self.add_call_frame(func, args_count as usize);
                                }
                                Value::Obj(Obj::NativeFn(func)) => {
                                    let start = self.stack.len() - args_count as usize;

                                    let native_func_obj: &RefCell<NativeFn> = func.borrow();
                                    let func = native_func_obj.borrow();
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
                                Value::Obj(Obj::Structs(struct_obj)) => {
                                    let length = self.stack.len();
                                    self.stack[length - (args_count as usize) - 1] =
                                        Value::Obj(Obj::Instance(Rc::new(RefCell::new(
                                            StructsInstance::new(struct_obj),
                                        ))));
                                }
                                _ => {
                                    self.runtime_error("Can only call Functions");
                                    return Err(InterpretError::RuntimeError);
                                }
                            }
                        }

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
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            self.push_stack(const_val);
                        }
                        OpCode::OpDefineGlobal(idx) => {
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            if let Value::Obj(Obj::String(var_name)) = const_val {
                                let value = self.pop_stack();

                                let borrow_var_name: &RefCell<StrObj> = var_name.borrow();
                                let mut name = borrow_var_name.borrow().name.to_string();

                                match value {
                                    Value::Obj(Obj::Structs(ref struct_obj)) => {
                                        let struct_obj: &RefCell<Structs> = struct_obj.borrow();
                                        name = struct_obj.borrow().name.to_string();
                                    }
                                    _ => {}
                                };

                                self.globals.insert(name, value);
                            }
                        }
                        OpCode::OpGetGlobal(idx) => {
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            if let Value::Obj(Obj::String(var_name)) = const_val {
                                let borrow_var_name: &RefCell<StrObj> = var_name.borrow();
                                let name: &StrObj = &borrow_var_name.borrow();

                                match self.globals.get(&name.name) {
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
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            if let Value::Obj(Obj::String(var_name)) = const_val {
                                let borrow_var_name: &RefCell<StrObj> = var_name.borrow();
                                let name: &String = &borrow_var_name.borrow().name;
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

                        OpCode::OpSetProperty(idx) => match self.peek_stack(1) {
                            Value::Obj(Obj::Instance(instance_obj)) => {
                                let func: &RefCell<FunctionObj> =
                                    self.get_frame().function.borrow();
                                let name = func.borrow_mut().chunk.get_const(idx);

                                let value = self.pop_stack();

                                let instance: &RefCell<StructsInstance> = instance_obj.borrow();
                                let mut instance = instance.borrow_mut();

                                match name {
                                    Value::Obj(Obj::String(str_obj)) => {
                                        let str: &RefCell<StrObj> = str_obj.borrow();
                                        let str = str.borrow();
                                        instance.fields.insert(str.name.to_string(), value);
                                    }
                                    _ => {
                                        unreachable!();
                                    }
                                }
                            }
                            _ => {
                                self.runtime_error("Can't set property on non Instance");
                                return Err(InterpretError::RuntimeError);
                            }
                        },
                        OpCode::OpGetProperty(idx) => {
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            let instance = self.peek_stack(0);

                            match const_val {
                                Value::Obj(Obj::String(ref name)) => {
                                    match instance {
                                        Value::Obj(Obj::Instance(instance)) => {
                                            let name: &RefCell<StrObj> = name.borrow();
                                            let name: &String = &name.borrow().name;

                                            let instance_obj: &RefCell<StructsInstance> =
                                                instance.borrow();
                                            let instance_fields: &HashMap<String, Value> =
                                                &instance_obj.borrow().fields;
                                            if instance_fields.contains_key(name) {
                                                self.pop_stack();
                                                if let Some(value) = instance_fields.get(name) {
                                                    self.push_stack(value.clone());
                                                }
                                            } else {
                                                self.runtime_error(
                                                    format!("Undefined property: {}", name)
                                                        .as_str(),
                                                );
                                                return Err(InterpretError::RuntimeError);
                                            }
                                        }
                                        _ => {
                                            self.runtime_error("Only instances of Structs are allowed to have properties");
                                            return Err(InterpretError::RuntimeError);
                                        }
                                    };
                                }
                                _ => {
                                    unreachable!();
                                }
                            }
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
                                let left_string: &RefCell<StrObj> = left_rc.borrow();
                                let mut left_string = left_string.borrow_mut();

                                let right_string: &RefCell<StrObj> = right_rc.borrow();
                                let right_string: &String = &right_string.borrow().name;

                                left_string.name.reserve(right_string.len());
                                left_string.name.push_str(&right_string.borrow());

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
                        OpCode::OpClass(idx) => {
                            let func: &RefCell<FunctionObj> = self.get_frame().function.borrow();
                            let const_val = func.borrow_mut().chunk.get_const(idx);
                            match const_val {
                                Value::Obj(Obj::String(str)) => {
                                    let string: &RefCell<StrObj> = str.borrow();
                                    let string: &String = &string.borrow().name;
                                    let value = Value::Obj(Obj::Structs(Rc::new(RefCell::new(
                                        Structs::new(string.to_string()),
                                    ))));
                                    self.push_stack(value);
                                }
                                _ => unreachable!(),
                            }
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
                    let str: &RefCell<StrObj> = string.borrow();
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
                    let str: &RefCell<StrObj> = string.borrow();
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

    fn add_call_frame(&mut self, function_obj: Rc<RefCell<FunctionObj>>, arg_count: usize) {
        let temp_func_obj = function_obj.clone();
        let temp_func_obj: &RefCell<FunctionObj> = temp_func_obj.borrow();
        let temp_func_obj = temp_func_obj.borrow();

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
            let func: &RefCell<FunctionObj> = self.frame[i].function.borrow();
            eprint!("| [line {}] in ", func.borrow().chunk.line[instruction]);

            eprintln!("{}", func.borrow());
        }

        let instruction = self.frame[self.frame_count - 1].ic - 1;
        let func: &RefCell<FunctionObj> = self.frame[self.frame_count - 1].function.borrow();
        eprintln!("> Error Occured Here:");
        eprint!("| [line {}] in ", func.borrow().chunk.line[instruction]);

        eprint!("{}: ", func.borrow());
        eprintln!("{}\n", message);
    }

    fn insert_natives(&mut self, natives: Vec<NativeFn>) {
        for native_fn in natives {
            self.define_native_fn(native_fn);
        }
    }

    fn define_native_fn(&mut self, native_fn: NativeFn) {
        let native_fn_name = native_fn.name.clone();
        let native_fn_val = Value::Obj(Obj::NativeFn(Rc::new(RefCell::new(native_fn))));
        self.push_stack(native_fn_val);

        let native_fn_val = self.pop_stack();
        self.globals.insert(native_fn_name, native_fn_val);
    }

    fn get_op_code(&mut self) -> Option<OpCode> {
        if let Some(frame) = self.frame.get_mut(self.frame_count - 1) {
            let func: &RefCell<FunctionObj> = frame.function.borrow();
            let code = func.borrow().chunk.code[frame.ic];
            frame.ic += 1;
            return Some(code);
        }

        None
    }

    #[inline]
    fn push_stack(&mut self, value: Value) {
        //if self.stack_cap < self.stack.capacity() {
        //    self.stack_cap = self.stack.capacity() * 2;
        //    self.collect_garbage();
        //}
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
