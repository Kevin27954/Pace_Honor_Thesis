use std::collections::HashMap;

use super::{errors::RuntimeError, runtime_types::RuntimeValue, token::Token};

#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    runtime_env: HashMap<String, RuntimeValue>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        RuntimeEnv {
            runtime_env: HashMap::new(),
        }
    }

    pub fn define_var(&mut self, var: String, val: RuntimeValue) {
        self.runtime_env.insert(var, val);
    }

    pub fn get_val(&self, var: &Token) -> Result<RuntimeValue, RuntimeError> {
        // Might need to clone if it gets complicated
        let val = self.runtime_env.get(&var.lexeme);
        if let None = val {
            return Err(RuntimeError::UndeclaredVariable(var.clone()));
        } else {
            return Ok(val.unwrap().clone());
        }
    }

    // This is for testing purposes only, will be deleted
    pub fn return_runtime_env(&self) -> HashMap<String, RuntimeValue> {
        self.runtime_env.clone()
    }
}
