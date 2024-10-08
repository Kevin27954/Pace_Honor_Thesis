use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::chunk::Chunk;

/*
* Enum is 4 byte (Number, Boolean, None - each one is a TAG represented by int (u32) )
* Current max memory is f64 -> 8 bytes
* Padding due to wanting same size bytes, so + 4 byte to TAG
* Total = 16 Byte
*/

// The size is 16 bytes: Enum: 4 byte, Padding: 4 Byte, Largest Type: 8 byte
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // We can make this more efficent by making it f16 or f32 instead. However for simplicity we
    // won't do this
    Number(f64),
    Boolean(bool),
    None,
    //String(String),
    Obj(Obj),
}

// For when I want to optimize Global Variables
// TODO use after done
#[allow(dead_code)]
pub struct GlobalVar {
    var_name: String,
    global_idx: u8,
}

// This is only 8 bytes max: Enum (4byte) + Box (8byte)
#[derive(Debug, PartialEq, Clone)]
pub enum Obj {
    String(Rc<RefCell<StrObj>>),
    Function(Rc<RefCell<FunctionObj>>),
    NativeFn(Rc<RefCell<NativeFn>>),
}

#[derive(Debug)]
pub struct StrObj {
    pub name: String,
    pub is_marked: bool,
}

impl StrObj {
    pub fn new(name: String) -> Self {
        StrObj {
            name,
            is_marked: false,
        }
    }
}

impl PartialEq for StrObj {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Display for StrObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionObj {
    pub arity: u8,
    pub chunk: Chunk,
    pub name: Option<String>,
    pub is_marked: bool,
}

impl FunctionObj {
    pub fn new() -> Self {
        FunctionObj {
            arity: 0,
            chunk: Chunk::new(),
            // Consider doing &str
            name: None,
            is_marked: false,
        }
    }
}

impl Display for FunctionObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "<fn {}>", name)
        } else {
            write!(f, "<script>")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NativeFn {
    pub name: String,
    pub arity: u8,
    pub native_fn: fn(usize, &[Value]) -> Result<Value, &str>,
    pub is_marked: bool,
}

impl Display for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        let concat_str = match self {
            Self::Number(num) => {
                format!("{}", num)
            }
            Self::Boolean(bool) => {
                format!("{}", bool)
            }
            Self::None => {
                format!("none")
            }
            Self::Obj(value_obj) => match value_obj {
                Obj::String(string) => {
                    format!("{}", string.borrow())
                }
                Obj::Function(function) => {
                    format!("{}", function.borrow())
                }
                Obj::NativeFn(function) => {
                    format!("{}", function.borrow())
                }
            },
        };

        str.push_str(&concat_str);
        write!(f, "{}", str)
    }
}
