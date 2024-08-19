use super::{
    expr_types::{Expr, Primary, Unary},
    runtime_types::RuntimeValue,
    token::{Literal, Number},
    token_types::TokenType,
};

// Might need an enum for this
// For now we just print it out?
pub fn evaluate_expr(expr: &Expr) -> RuntimeValue {
    match expr {
        Expr::Primary(primary) => match primary {
            Primary::Literal(literal) => match literal {
                Literal::Number(number) => match number {
                    Number::Float(float) => RuntimeValue::Number(*float),
                },
                Literal::String(string) => RuntimeValue::String(string.to_string()),
            },
            Primary::Boolean(bool) => RuntimeValue::Boolean(*bool),
            Primary::None => RuntimeValue::None,
        },
        Expr::Group(expr) => evaluate_expr(expr.as_ref()),
        Expr::Unary(unary) => match unary {
            Unary::UnaryExpr(operator, expr) => {
                let value = evaluate_expr(expr.as_ref());

                match operator.token_type {
                    TokenType::BANG => {
                        return RuntimeValue::Boolean(!is_truthy(value));
                    }
                    TokenType::MINUS => {
                        if let RuntimeValue::Number(num) = value {
                            return RuntimeValue::Number(-num);
                        }
                    }
                    _ => {}
                }

                // Should never touch this
                eprintln!("Error in Parser. Couldn't evaluate Unary");
                return RuntimeValue::None;
            }
        },
        Expr::Binary(left, operator, right) => {
            let left_val = evaluate_expr(left.as_ref());
            let right_val = evaluate_expr(right.as_ref());

            match &operator.token_type {
                TokenType::SLASH => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Number(left / right);
                    }
                }
                TokenType::STAR => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Number(left * right);
                    }
                }
                TokenType::MINUS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Number(left - right);
                    }
                }
                TokenType::PLUS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (&left_val, &right_val)
                    {
                        return RuntimeValue::Number(left - right);
                    } else if let (RuntimeValue::String(left), RuntimeValue::String(right)) =
                        (&left_val, &right_val)
                    {
                        return RuntimeValue::String(left.clone() + right);
                    }
                }
                TokenType::GREATER => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(left > right);
                    }
                }
                TokenType::GREATER_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(left >= right);
                    }
                }
                TokenType::LESS => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(left <= right);
                    }
                }
                TokenType::LESS_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(left <= right);
                    }
                }
                TokenType::BANG_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(!(left == right));
                    }
                }
                TokenType::EQUAL_EQUAL => {
                    if let (RuntimeValue::Number(left), RuntimeValue::Number(right)) =
                        (left_val, right_val)
                    {
                        return RuntimeValue::Boolean(left == right);
                    }
                }
                _ => {
                    todo!()
                }
            }

            return RuntimeValue::None;
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
