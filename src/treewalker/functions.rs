use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

use super::runtime_types::RuntimeValue;

#[derive(Clone)]
pub struct NativeFunctions {
    pub arity: usize,
    pub name: String,
    pub function: fn(Vec<RuntimeValue>) -> RuntimeValue,
}

impl NativeFunctions {
    pub fn get_arity(&self) -> usize {
        self.arity
    }

    pub fn call(&self, args: Vec<RuntimeValue>) -> RuntimeValue {
        (self.function)(args)
    }
}

impl Display for NativeFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl Debug for NativeFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn>")
    }
}

impl PartialEq for NativeFunctions {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn clock(_args: Vec<RuntimeValue>) -> RuntimeValue {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH);
    match secs {
        Ok(sec) => RuntimeValue::Number(sec.as_secs_f64()),
        Err(_err) => RuntimeValue::None,
    }
}

pub fn print(args: Vec<RuntimeValue>) -> RuntimeValue {
    let output_str = args
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{output_str}");

    RuntimeValue::None
}
