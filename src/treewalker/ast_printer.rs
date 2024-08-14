use super::expr_types::{Expr, ExprLitearl};
use super::token::Literal;

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Unary(token, expr) => {
            let exprs: &[&Expr] = &[expr.as_ref()];
            parenthesize(&token.lexeme, exprs)
        }
        Expr::Binary(expr_left, token, expr_right) => {
            let exprs: &[&Expr] = &[expr_left.as_ref(), expr_right.as_ref()];
            parenthesize(&token.lexeme, exprs)
        }
        Expr::Litearl(token) => {
            if let Some(token) = token {
                match token {
                    ExprLitearl::Boolean(bool) => return format!("{}", bool),
                    ExprLitearl::Literal(literal) => match literal {
                        Literal::Number(number) => {
                            return format!("{:?}", number);
                        }
                        Literal::String(string) => {
                            return format!("{}", string);
                        }
                    },
                }
            }

            String::from("nil")
        }
        Expr::Grouping(expr) => {
            let exprs: &[&Expr] = &[expr.as_ref()];
            parenthesize(&String::from("group"), exprs)
        }
    }
}

fn parenthesize(name: &String, exprs: &[&Expr]) -> String {
    let mut s = String::new();
    s.push('(');
    s.push_str(&name);

    for i in 0..exprs.len() {
        s.push(' ');
        s.push_str(&print_ast(&exprs[i]));
    }

    s.push(')');
    s
}
