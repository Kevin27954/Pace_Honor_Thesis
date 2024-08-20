use super::{
    errors::{parse_runtime_err, RuntimeError},
    expr_types::{Expr, Primary, Unary},
    runtime_types::RuntimeValue,
    token::{Literal, Number},
    token_types::TokenType,
};

pub fn interpret(expr: &Expr) -> bool {
    let result = evaluate_expr(expr);
    let mut has_error = false;

    match result {
        Ok(value) => println!("{}", value),
        Err(runtime_err) => {
            has_error = true;
            parse_runtime_err(runtime_err)
        }
    }

    return has_error;
}

pub fn evaluate_expr(expr: &Expr) -> Result<RuntimeValue, RuntimeError> {
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
        Expr::Group(expr) => evaluate_expr(expr.as_ref()),
        Expr::Unary(unary) => match unary {
            Unary::UnaryExpr(operator, expr) => {
                let value = evaluate_expr(expr.as_ref())?;

                match operator.token_type {
                    TokenType::BANG => {
                        return Ok(RuntimeValue::Boolean(!is_truthy(value)));
                    }
                    TokenType::MINUS => {
                        if let RuntimeValue::Number(num) = value {
                            return Ok(RuntimeValue::Number(-num));
                        } else {
                            return Err(RuntimeError::UnaryTypeMismatch(&operator, value));
                        }
                    }
                    _ => {}
                }

                unreachable!("Unary -> This part should never be reached")
            }
        },
        Expr::Binary(left, operator, right) => {
            let left_val = evaluate_expr(left.as_ref())?;
            let right_val = evaluate_expr(right.as_ref())?;

            match &operator.token_type {
                TokenType::SLASH => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        if right == &0.0 {
                            return Err(RuntimeError::DivideByZero(operator));
                        }
                        return Ok(RuntimeValue::Number(left / right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::STAR => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Number(left * right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::MINUS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Number(left - right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::PLUS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Number(left + right));
                    } else if let (RuntimeValue::String(left), RuntimeValue::String(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::String(left.clone() + right));
                    } else if let RuntimeValue::String(string) = &left_val {
                        return Ok(RuntimeValue::String(
                            string.clone() + &right_val.to_string(),
                        ));
                    } else if let RuntimeValue::String(string) = &right_val {
                        return Ok(RuntimeValue::String(string.clone() + &left_val.to_string()));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::GREATER => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(left > right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::GREATER_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(left >= right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::LESS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(left <= right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::LESS_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(left <= right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::BANG_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(!(left == right)));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                TokenType::EQUAL_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return Ok(RuntimeValue::Boolean(left == right));
                    } else {
                        return Err(RuntimeError::BinaryTypeMismatch(
                            left_val, operator, right_val,
                        ));
                    }
                }
                _ => {}
            }

            unreachable!("Binary -> This part should never be reached")
        }
    }
}

fn is_truthy(value: RuntimeValue) -> bool {
    match value {
        RuntimeValue::String(_) | RuntimeValue::Number(_) => true,
        RuntimeValue::Boolean(bool) => bool,
        RuntimeValue::None => false,
    }
}
