use std::{cell::RefCell, fmt::Display, mem, rc::Rc};

use super::values::{FunctionObj, Obj, StrObj, Structs, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    OpReturn,
    OpPop,
    OpCall(u8),

    OpJumpIfFalse(u8),
    OpJump(u8),
    OpLoop(u8),

    // Literal
    OpTrue,
    OpFalse,
    OpNone,

    // Equality
    OpGreater,
    OpLess,
    OpEqual,

    // Primary
    OpConstant(usize),
    OpDefineGlobal(usize),
    OpGetGlobal(usize),
    OpSetGlobal(usize),
    OpGetLocal(usize),
    OpSetLocal(usize),

    // Unary
    OpNegate,
    OpNot,

    // Binary
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,

    // Class
    OpClass(usize),
    OpSetProperty(usize),
    OpGetProperty(usize),
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub line: Vec<usize>,
    pub values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            line: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn write_code(&mut self, code: OpCode, line: usize) -> usize {
        self.code.push(code);
        self.line.push(line);
        self.code.len() - 1
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn get_const(&mut self, idx: usize) -> Value {
        //match self.values[idx as usize] {
        //    Value::None | Value::Number(_) | Value::Boolean(_) => self.values[idx as usize].clone(),
        //    Value::Obj(ref mut obj) => {
        //        let new_obj = match obj {
        //            Obj::String(str) => {
        //                let mut str_obj = Rc::new(RefCell::new(StrObj::default()));
        //                mem::swap(str, &mut str_obj);
        //                //let temp: &RefCell<StrObj> = str.borrow();
        //                //let temp: &StrObj = &temp.borrow();
        //                //Obj::String(Rc::new(RefCell::new(temp.clone())))
        //                Obj::String(str_obj)
        //            }
        //            Obj::Function(func) => {
        //                let mut func_obj = Rc::new(RefCell::new(FunctionObj::new()));
        //                mem::swap(func, &mut func_obj);
        //                //let temp: &RefCell<FunctionObj> = func.borrow();
        //                //let temp: &FunctionObj = &temp.borrow();
        //                //Obj::Function(Rc::new(RefCell::new(temp.clone())))
        //                Obj::Function(func_obj)
        //            }
        //            Obj::NativeFn(native_func) => {
        //                //let temp: &RefCell<NativeFn> = native_func.borrow();
        //                //let temp: &NativeFn = &temp.borrow();
        //                //Obj::NativeFn(Rc::new(RefCell::new(temp.clone())))
        //                Obj::NativeFn(native_func.clone())
        //            }
        //            Obj::Structs(_struct_ref) => {
        //                unreachable!();
        //            }
        //            Obj::Instance(_instance_ref) => {
        //                unreachable!();
        //            }
        //        };
        //        Value::Obj(new_obj)
        //    }
        //}
        self.values[idx as usize].clone()
    }
}
