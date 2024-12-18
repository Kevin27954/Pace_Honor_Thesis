use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

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
    Structs(Rc<RefCell<Structs>>),
    Instance(Rc<RefCell<StructsInstance>>),
}

#[derive(Debug, Clone)]
pub struct Structs {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub is_marked: bool,
}

impl Structs {
    pub fn new(name: String) -> Self {
        Structs {
            name,
            fields: HashMap::new(),
            is_marked: false,
        }
    }
}

impl Default for Structs {
    fn default() -> Self {
        Structs {
            name: String::new(),
            fields: HashMap::new(),
            is_marked: false,
        }
    }
}

impl Display for Structs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_str = String::new();
        display_str.push_str(&self.name);
        display_str.push('{');
        display_str.push('\t');

        for (key, value) in &self.fields {
            display_str.push_str(&key.to_string());
            display_str.push(':');
            display_str.push_str(&value.to_string());
            display_str.push_str(", ");
        }
        display_str.push('\t');
        display_str.push('}');

        write!(f, "{}", display_str)
    }
}

impl PartialEq for Structs {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Debug, PartialEq)]
pub struct StructsInstance {
    origin: Rc<RefCell<Structs>>,
    pub fields: HashMap<String, Value>,
    pub is_marked: bool,
}

impl StructsInstance {
    pub fn new(origin: Rc<RefCell<Structs>>) -> Self {
        use std::borrow::Borrow;
        let origin_hashmap = origin.clone();
        let origin_hashmap: &RefCell<Structs> = origin_hashmap.borrow();
        let origin_hashmap: &Structs = &origin_hashmap.borrow();
        let fields = origin_hashmap.fields.clone();
        StructsInstance {
            origin,
            fields,
            is_marked: false,
        }
    }
}

impl Display for StructsInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::borrow::Borrow;
        let origin: &RefCell<Structs> = self.origin.borrow();

        let mut display_str = String::new();
        display_str.push_str(&origin.borrow().name);
        display_str.push_str(" instance");
        display_str.push('{');
        display_str.push('\t');

        for (key, value) in &self.fields {
            display_str.push_str(&key.to_string());
            display_str.push(':');
            display_str.push_str(&value.to_string());
            display_str.push_str(", ");
        }
        display_str.push('\t');
        display_str.push('}');

        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Clone)]
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

impl Default for StrObj {
    fn default() -> Self {
        StrObj {
            name: String::new(),
            is_marked: false,
        }
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
                Obj::Structs(structs) => {
                    format!("{}", structs.borrow())
                }
                Obj::Instance(instance) => {
                    format!("{}", instance.borrow())
                }
            },
        };

        str.push_str(&concat_str);
        write!(f, "{}", str)
    }
}
