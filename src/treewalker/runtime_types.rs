use std::fmt::{Display, Formatter, Result};

use super::functions::{NativeFunctions, RuntimeFunctions};

#[derive(PartialEq, Clone, Debug)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
    NativeFunction(NativeFunctions),
    RuntimeFunctions(RuntimeFunctions),
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::String(string) => {
                write!(f, "{}", string)
            }
            Self::Number(number) => {
                write!(f, "{}", number)
            }
            Self::Boolean(bool) => {
                write!(f, "{}", bool)
            }
            Self::None => {
                write!(f, "none")
            }
            Self::NativeFunction(function) => {
                write!(f, "{}", function)
            }
            Self::RuntimeFunctions(function) => {
                write!(f, "{}", function)
            }
        }
    }
}

impl RuntimeValue {
    pub fn err_format(&self) -> String {
        match self {
            Self::String(string) => {
                format!("String '{}'", string)
            }
            Self::Number(number) => {
                format!("Number '{}'", number)
            }
            Self::Boolean(bool) => {
                format!("Boolean '{}'", bool)
            }
            Self::None => {
                format!("None 'none'")
            }
            Self::NativeFunction(function) => {
                format!("Function '{}'", function)
            }
            Self::RuntimeFunctions(function) => {
                format!("Function '{}'", function)
            }
        }
    }
}
