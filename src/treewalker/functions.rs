use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    interpreter::Interpreter, runtime_types::RuntimeValue, statements::Stmt, token::Token,
};

#[derive(Clone)]
pub struct RuntimeFunctions {
    pub name: String,
    pub params: Vec<Token>,
    pub block: Stmt,
}

impl RuntimeFunctions {
    pub fn get_arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(&self, interperter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeValue {
        interperter.runtime_env.add_scope();

        for i in 0..args.len() {
            interperter
                .runtime_env
                .define_var(self.params[i].lexeme.to_string(), args[i].clone());
        }

        let (val, is_return) = interperter.interpret(&self.block);
        if is_return {
            interperter.runtime_env.pop_scope();
            return val;
        }

        interperter.runtime_env.pop_scope();
        RuntimeValue::None
    }
}

impl Display for RuntimeFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

impl Debug for RuntimeFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

impl PartialEq for RuntimeFunctions {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

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
