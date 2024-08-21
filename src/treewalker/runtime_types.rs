use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Clone, Debug)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
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
        }
    }
}
