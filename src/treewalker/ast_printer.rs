use super::expr_types::{Expr, Primary, Unary};
use super::token::Literal;

pub fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Primary(primary) => primary_string(primary),
        Expr::Unary(unary) => match unary {
            Unary::UnaryExpr(operator, unary) => {
                let temp = unary.as_ref();
                parenthesize(&operator.lexeme, &[&Expr::Unary(temp.clone())])
            }
            Unary::Primary(primary) => primary_string(primary),
        },
        Expr::Binary(left, operator, right) => {
            let slice = &[left.as_ref(), right.as_ref()];
            parenthesize(&operator.lexeme, slice)
        }
        Expr::Group(expr) => parenthesize(&String::from("group"), &[expr.as_ref()]),

        _ => {
            todo!("Implement this")
        }
    }
}

/*
!!!false
! !!false

!!false
! false
*/

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

fn primary_string(primary: &Primary) -> String {
    match primary {
        Primary::None => return format!("None"),
        Primary::Boolean(bool) => return format!("{}", bool),
        Primary::Literal(literal) => match literal {
            Literal::Number(number) => {
                return format!("{:?}", number);
            }
            Literal::String(string) => {
                return format!("{}", string);
            }
        },
    }
}
