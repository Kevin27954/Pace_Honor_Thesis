use std::{collections::HashMap, fmt::Display};

use super::{errors::RuntimeError, runtime_types::RuntimeValue, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub properties: Vec<Token>,
    pub fields: HashMap<String, RuntimeValue>,
}

impl Struct {
    pub fn arity(&self) -> usize {
        return self.properties.len();
    }

    pub fn new(&self, values: Vec<RuntimeValue>) -> RuntimeValue {
        let mut fields = HashMap::new();

        for i in 0..values.len() {
            fields.insert(self.properties[i].lexeme.clone(), values[i].clone());
        }

        let user_struct = Struct {
            name: self.name.clone(),
            properties: self.properties.clone(),
            fields,
        };
        RuntimeValue::Struct(user_struct)
    }

    pub fn get(&self, token: &Token) -> Result<RuntimeValue, RuntimeError> {
        if self.fields.contains_key(&token.lexeme) {
            return Ok(self.fields.get(&token.lexeme).unwrap().clone());
        };

        return Err(RuntimeError::UndeclaredVariable(token.clone()));
    }

    pub fn set(&mut self, token: &Token, value: RuntimeValue) -> Result<(), RuntimeError> {
        if self.fields.contains_key(&token.lexeme) {
            self.fields.insert(token.lexeme.to_string(), value);
            return Ok(());
        } else {
            return Err(RuntimeError::UndeclaredVariable(token.clone()));
        }
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
