use super::{
    errors::{parse_runtime_err, RuntimeError},
    expr_types::{Expr, Primary, Unary},
    runtime_env::RuntimeEnv,
    runtime_types::RuntimeValue,
    statements::Stmt,
    token::{Literal, Number},
    token_types::TokenType,
};

pub struct Interpreter {
    runtime_env: RuntimeEnv,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            runtime_env: RuntimeEnv::new(),
        }
    }

    pub fn get_runtime_env(&self) -> &RuntimeEnv {
        &self.runtime_env
    }

    // returns TRUE if error
    pub fn interpret(&mut self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::Expression(expr) => {
                let result = self.evaluate_expr(expr);
                match result {
                    Ok(value) => println!("{}", value),
                    Err(runtime_err) => {
                        parse_runtime_err(runtime_err);
                        return true;
                    }
                }
            }
            Stmt::VarDecl(var, val) => {
                let runtime_val = match val {
                    Some(expr) => match self.evaluate_expr(&expr) {
                        Ok(val) => val,
                        Err(runtime_err) => {
                            parse_runtime_err(runtime_err);
                            return true;
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
                    if self.interpret(&stmt) {
                        self.runtime_env.pop_scope();
                        return true;
                    }
                }

                self.runtime_env.pop_scope();
            }
        }

        return false;
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
            Expr::Variable(var) => self.runtime_env.get_val(var),
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
                            return Ok(RuntimeValue::Boolean(left <= right));
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
                self.runtime_env.assign_var(var, value.clone())?;
                return Ok(value);
            }
        }
    }

    fn is_truthy(&self, value: RuntimeValue) -> bool {
        match value {
            RuntimeValue::String(_) | RuntimeValue::Number(_) => true,
            RuntimeValue::Boolean(bool) => bool,
            RuntimeValue::None => false,
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
