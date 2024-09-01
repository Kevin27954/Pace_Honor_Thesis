use std::{borrow::BorrowMut, collections::HashMap, fmt::Display};

use super::{errors::RuntimeError, expr_types::Expr, runtime_types::RuntimeValue, token::Token};

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

    pub fn test(
        &mut self,
        expr: &Expr,
        name: &Token,
        value: RuntimeValue,
    ) -> Result<&mut Struct, RuntimeError> {
        let mut temp_expr = expr;
        let mut temp_token = Vec::new();
        while let Expr::Dot(inner_expr, token) = temp_expr {
            temp_expr = inner_expr;
            temp_token.push(token);
        }

        let mut struct_fields = Some(self.fields.borrow_mut());
        for token in temp_token.iter().rev() {
            struct_fields = match struct_fields.unwrap().get_mut(&token.lexeme) {
                Some(rs) => match rs {
                    RuntimeValue::Struct(s) => Some(s.fields.borrow_mut()),
                    _ => None,
                },
                None => return Err(RuntimeError::UndeclaredVariable(name.clone())),
            }
        }

        let temp = struct_fields
            .unwrap()
            .insert(name.lexeme.to_string(), value);

        if let None = temp {
            return Err(RuntimeError::UndeclaredVariable(name.clone()));
        }

        Ok(self)
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
