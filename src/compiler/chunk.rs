use std::fmt::Display;

use super::values::Value;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpReturn,
    OpPop,

    // Literal
    OpTrue,
    OpFalse,
    OpNone,

    // Equality
    OpGreater,
    OpLess,
    OpEqual,

    // Primary
    OpConstant(u8),
    OpDefineGlobal(u8),
    OpGetGlobal(u8),
    OpSetGlobal(u8),
    OpGetLocal(u8),
    OpSetLocal(u8),

    // Unary
    OpNegate,
    OpNot,

    // Binary
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

    pub fn get_const(&self, idx: u8) -> Value {
        self.values[idx as usize].clone()
    }
}
