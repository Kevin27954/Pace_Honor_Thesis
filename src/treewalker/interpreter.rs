use std::collections::HashMap;

use crate::treewalker::functions::RuntimeFunctions;

use super::{
    errors::{parse_runtime_err, RuntimeError},
    expr_types::{Expr, Primary, Unary},
    runtime_env::RuntimeEnv,
    runtime_types::RuntimeValue,
    statements::Stmt,
    token::{Literal, Number, Token},
    token_types::TokenType,
};

pub struct Interpreter {
    pub runtime_env: RuntimeEnv,
    symbol_table: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global = RuntimeEnv::new();
        Interpreter {
            runtime_env: global,
            symbol_table: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, expr: &Expr, levels: usize) {
        self.symbol_table.insert(expr.clone(), levels);
    }

    pub fn get_runtime_env(&self) -> &RuntimeEnv {
        &self.runtime_env
    }

    // returns TRUE if error
    pub fn interpret(&mut self, stmt: &Stmt) -> (RuntimeValue, bool) {
        match stmt {
            Stmt::Expression(expr) => match self.evaluate_expr(expr) {
                Ok(_value) => {}
                Err(runtime_err) => {
                    parse_runtime_err(runtime_err);
                    return (RuntimeValue::None, false);
                }
            },
            Stmt::VarDecl(var, val) => {
                let runtime_val = match val {
                    Some(expr) => match self.evaluate_expr(&expr) {
                        Ok(val) => val,
                        Err(runtime_err) => {
                            parse_runtime_err(runtime_err);
                            return (RuntimeValue::None, false);
                        }
                    },
                    None => RuntimeValue::None,
                };

                self.runtime_env
                    .define_var(var.lexeme.to_string(), runtime_val);
            }
            Stmt::Block(stmts) => {
                self.runtime_env.add_scope();

                for stmt in stmts {
                    let (val, error) = self.interpret(&stmt);
                    if error {
                        self.runtime_env.pop_scope();
                        return (val, error);
                    }
                }

                self.runtime_env.pop_scope();
            }
            Stmt::IfStmt(expr, if_block, else_block) => {
                let truthy: bool;
                match self.evaluate_expr(&expr) {
                    Ok(val) => {
                        truthy = self.is_truthy(val);
                    }
                    Err(err) => {
                        parse_runtime_err(err);
                        return (RuntimeValue::None, false);
                    }
                };

                if truthy {
                    return self.interpret(if_block.as_ref());
                } else {
                    if let Some(block) = else_block.as_ref() {
                        return self.interpret(block);
                    }
                }
            }
            Stmt::WhileStmt(expr, while_block) => {
                while let Ok(val) = self.evaluate_expr(expr) {
                    if !self.is_truthy(val) {
                        break;
                    }

                    let (val, is_return) = self.interpret(while_block.as_ref());
                    if is_return {
                        return (val, is_return);
                    }
                }
            }
            Stmt::RuntimeFunctions(name, params, body) => {
                let runtime_fn = RuntimeFunctions {
                    name: name.to_string(),
                    params: params.to_vec(),
                    block: body.as_ref().clone(),
                };

                self.runtime_env
                    .define_var(name.to_string(), RuntimeValue::RuntimeFunctions(runtime_fn));
            }
            Stmt::Return(_token, value) => {
                if let Some(expr) = value {
                    match self.evaluate_expr(expr) {
                        Ok(val) => {
                            return (val, true);
                        }
                        Err(err) => parse_runtime_err(err),
                    }
                } else {
                    return (RuntimeValue::None, true);
                }
            }
        }

        return (RuntimeValue::None, false);
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<RuntimeValue, RuntimeError> {
        match expr {
            Expr::Primary(primary) => match primary {
                Primary::Literal(literal) => match literal {
                    Literal::Number(number) => match number {
                        Number::Float(float) => Ok(RuntimeValue::Number(*float)),
                    },
                    Literal::String(string) => Ok(RuntimeValue::String(string.to_string())),
                },
                Primary::Boolean(bool) => Ok(RuntimeValue::Boolean(*bool)),
                Primary::None => Ok(RuntimeValue::None),
            },
            Expr::Group(expr) => self.evaluate_expr(expr.as_ref()),
            Expr::Variable(var) => self.look_up_var(var, expr),
            Expr::Unary(unary) => match unary {
                Unary::UnaryExpr(operator, expr) => {
                    let value = self.evaluate_expr(expr.as_ref())?;

                    let operator = operator.clone();
                    match operator.token_type {
                        TokenType::BANG => {
                            return Ok(RuntimeValue::Boolean(!self.is_truthy(value)));
                        }
                        TokenType::MINUS => {
                            if let RuntimeValue::Number(num) = value {
                                return Ok(RuntimeValue::Number(-num));
                            } else {
                                return Err(RuntimeError::UnaryTypeMismatch(operator, value));
                            }
                        }
                        _ => {}
                    }

                    unreachable!("Unary -> This part should never be reached")
                }
            },
            Expr::Logical(left, operator, right) => match operator.token_type {
                TokenType::AND => {
                    let left_val = self.evaluate_expr(left.as_ref())?;
                    if !self.is_truthy(left_val.clone()) {
                        return Ok(left_val);
                    }

                    let right_val = self.evaluate_expr(right.as_ref())?;
                    return Ok(right_val);
                }
                TokenType::OR => {
                    let left_val = self.evaluate_expr(left.as_ref())?;
                    if self.is_truthy(left_val.clone()) {
                        return Ok(left_val);
                    }

                    let right_val = self.evaluate_expr(right.as_ref())?;
                    return Ok(right_val);
                }
                _ => unreachable!(),
            },
            Expr::Binary(left, operator, right) => {
                let left_val = self.evaluate_expr(left.as_ref())?;
                let right_val = self.evaluate_expr(right.as_ref())?;

                let operator = operator.clone();
                match operator.token_type {
                    TokenType::SLASH => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            if right == &0.0 {
                                return Err(RuntimeError::DivideByZero(operator));
                            }

                            return Ok(RuntimeValue::Number(left / right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::STAR => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Number(left * right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::MINUS => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Number(left - right));
                        } else {
                            return Err(RuntimeError::BinaryTypeMismatch(
                                left_val, operator, right_val,
                            ));
                        }
                    }
                    TokenType::PLUS => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Number(left + right));
                        }
                        if let Some((left, right)) = self.extract_string_pair(&left_val, &right_val)
                        {
                            return Ok(RuntimeValue::String(left.clone() + right));
                        }
                        if let RuntimeValue::String(string) = &left_val {
                            return Ok(RuntimeValue::String(
                                string.clone() + &right_val.to_string(),
                            ));
                        }
                        if let RuntimeValue::String(string) = &right_val {
                            return Ok(RuntimeValue::String(left_val.to_string() + string));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::GREATER => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Boolean(left > right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::GREATER_EQUAL => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Boolean(left >= right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::LESS => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Boolean(left < right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::LESS_EQUAL => {
                        if let Some((left, right)) = self.extract_num_pair(&left_val, &right_val) {
                            return Ok(RuntimeValue::Boolean(left <= right));
                        }

                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                    TokenType::BANG_EQUAL => {
                        return Ok(RuntimeValue::Boolean(left_val != right_val));
                    }
                    TokenType::EQUAL_EQUAL => {
                        return Ok(RuntimeValue::Boolean(left_val == right_val));
                    }
                    _ => {}
                }

                unreachable!("Binary -> This part should never be reached")
            }
            Expr::Assignment(var, expr) => {
                let value = self.evaluate_expr(expr.as_ref())?;
                //self.runtime_env.assign_var(var, value.clone())?;
                self.assign_var(var, expr)?;
                return Ok(value);
            }
            Expr::Call(callee, _right_paren, assignments) => {
                let callee = self.evaluate_expr(callee)?;

                let mut values: Vec<RuntimeValue> = Vec::new();
                for assignment in assignments {
                    values.push(self.evaluate_expr(assignment)?);
                }

                match callee {
                    RuntimeValue::NativeFunction(ref func) => {
                        if func.name == "print".to_string() && values.len() > func.get_arity() {
                            unimplemented!("too many args ferronccjj");
                        } else if func.name != "print".to_string()
                            && func.get_arity() != values.len()
                        {
                            unimplemented!("Unequal arguments efasdfrro");
                        }
                    }
                    RuntimeValue::RuntimeFunctions(ref func) => {
                        if values.len() != func.get_arity() {
                            unimplemented!("Unequal arguments efasdfrro");
                        }
                    }
                    _ => {
                        unimplemented!("Not a function error");
                    }
                }

                let value = match callee {
                    RuntimeValue::NativeFunction(ref func) => func.call(values),
                    RuntimeValue::RuntimeFunctions(ref func) => func.call(self, values),
                    _ => unreachable!(),
                };

                Ok(value)
            }
        }
    }

    fn look_up_var(&self, token: &Token, expr: &Expr) -> Result<RuntimeValue, RuntimeError> {
        let option_distance = self.symbol_table.get(expr);

        if let Some(distance) = option_distance {
            self.runtime_env.get_at(*distance, token)
        } else {
            if let Some(val) = self.runtime_env.get_global().get(&token.lexeme) {
                return Ok(val.clone());
            } else {
                return Ok(RuntimeValue::None);
            }
        }
    }

    fn assign_var(&mut self, token: &Token, expr: &Expr) -> Result<(), RuntimeError> {
        let val = self.evaluate_expr(expr)?;
        let option_distance = self.symbol_table.get(expr);

        if let Some(distance) = option_distance {
            self.runtime_env.assign_at(*distance, token, val)?;
        } else {
            self.runtime_env.assign_global(token.lexeme.clone(), val)?;
        }

        Ok(())
    }

    fn is_truthy(&self, value: RuntimeValue) -> bool {
        match value {
            RuntimeValue::String(_) | RuntimeValue::Number(_) => true,
            RuntimeValue::Boolean(bool) => bool,
            RuntimeValue::None => false,
            RuntimeValue::NativeFunction(_) => true,
            RuntimeValue::RuntimeFunctions(_) => true,
        }
    }

    fn extract_num_pair<'a>(
        &self,
        left: &'a RuntimeValue,
        right: &'a RuntimeValue,
    ) -> Option<(&'a f64, &'a f64)> {
        match (left, right) {
            (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) => {
                Some((left_num, right_num))
            }
            _ => None,
        }
    }

    fn extract_string_pair<'a>(
        &self,
        left: &'a RuntimeValue,
        right: &'a RuntimeValue,
    ) -> Option<(&'a String, &'a String)> {
        match (left, right) {
            (RuntimeValue::String(left_string), RuntimeValue::String(right_string)) => {
                Some((left_string, right_string))
            }
            _ => None,
        }
    }
}
