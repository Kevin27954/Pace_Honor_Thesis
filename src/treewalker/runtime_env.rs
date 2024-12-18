use std::collections::{HashMap, LinkedList};

use super::{
    errors::RuntimeError,
    functions::{clock, print, NativeFunctions},
    runtime_types::RuntimeValue,
    token::Token,
};

#[derive(Debug)]
pub struct RuntimeEnv {
    pub runtime_env: LinkedList<HashMap<String, RuntimeValue>>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        let mut env = LinkedList::new();
        let mut global: HashMap<String, RuntimeValue> = HashMap::new();

        global.insert(
            "clock".to_string(),
            RuntimeValue::NativeFunction(NativeFunctions {
                arity: 0,
                name: "clock".to_string(),
                function: clock,
            }),
        );

        global.insert(
            "print".to_string(),
            RuntimeValue::NativeFunction(NativeFunctions {
                arity: 16,
                name: "print".to_string(),
                function: print,
            }),
        );

        env.push_front(global);
        RuntimeEnv { runtime_env: env }
    }

    pub fn get_global(&self) -> &HashMap<String, RuntimeValue> {
        &self.runtime_env.back().unwrap()
    }

    pub fn assign_global(&mut self, name: String, val: RuntimeValue) -> Result<(), RuntimeError> {
        let global = self.runtime_env.back_mut().unwrap();
        global.insert(name, val);

        Ok(())
    }

    pub fn add_scope(&mut self) {
        let local_scope: HashMap<String, RuntimeValue> = HashMap::new();
        self.runtime_env.push_front(local_scope)
    }

    pub fn pop_scope(&mut self) {
        self.runtime_env.pop_front();
    }

    pub fn define_var(&mut self, var: String, val: RuntimeValue) {
        self.runtime_env
            .front_mut()
            .unwrap()
            .insert(var, val.clone());
    }

    pub fn assign_var(&mut self, var: &Token, val: RuntimeValue) -> Result<(), RuntimeError> {
        let mut iter = self.runtime_env.iter_mut();

        while let Some(env) = iter.next() {
            if env.contains_key(&var.lexeme) {
                env.insert(var.lexeme.clone(), val);
                return Ok(());
            }
        }

        Err(RuntimeError::UndeclaredVariable(var.clone()))
    }

    pub fn get_val(&self, var: &Token) -> Result<RuntimeValue, RuntimeError> {
        let mut iter = self.runtime_env.iter();

        while let Some(env) = iter.next() {
            if let Some(val) = env.get(&var.lexeme) {
                return Ok(val.clone());
            }
        }

        return Err(RuntimeError::UndeclaredVariable(var.clone()));
    }

    pub fn get_at(&self, distance: usize, token: &Token) -> Result<RuntimeValue, RuntimeError> {
        let mut iter = self.runtime_env.iter();

        for _ in 1..distance {
            iter.next();
        }

        if let Some(env) = iter.next() {
            if let Some(val) = env.get(&token.lexeme) {
                return Ok(val.clone());
            } else {
                unreachable!("Supposiblity unreach");
            }
        } else {
            unreachable!("shouldn't be ");
        }
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        token: &Token,
        val: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        let mut iter = self.runtime_env.iter_mut().rev();

        for _ in 0..distance {
            iter.next();
        }

        if let Some(env) = iter.next() {
            env.insert(token.lexeme.clone(), val);
        }

        Ok(())
    }

    // This is for testing purposes only, will be deleted
    pub fn return_runtime_env(&self) -> LinkedList<HashMap<String, RuntimeValue>> {
        self.runtime_env.clone()
    }
}
