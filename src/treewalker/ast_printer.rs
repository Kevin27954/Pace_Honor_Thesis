use super::{
    expr_types::{Expr, Primary, Unary},
    statements::Stmt,
};

pub fn print_ast(stmt: &Stmt) -> String {
    match stmt {
        Stmt::VarDecl(var, val) => match val {
            Some(val) => {
                format!("{} = {}", var.lexeme, print_expr(&val))
            }
            None => {
                format!("{}", var.lexeme)
            }
        },
        Stmt::Expression(expr) => print_expr(&expr),
    }
}

pub fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Primary(primary) => primary_string(primary),
        Expr::Unary(unary) => match unary {
            Unary::UnaryExpr(operator, unary) => {
                let temp = unary.as_ref();
                parenthesize(&operator.lexeme, &[&temp.clone()])
            }
        },
        Expr::Binary(left, operator, right) => {
            let slice = &[left.as_ref(), right.as_ref()];
            parenthesize(&operator.lexeme, slice)
        }
        Expr::Group(expr) => parenthesize(&String::from("group"), &[expr.as_ref()]),
        Expr::Variable(var) => var.lexeme.to_string(),
    }
}

fn parenthesize(name: &String, exprs: &[&Expr]) -> String {
    let mut s = String::new();
    s.push('(');
    s.push_str(&name);

    for i in 0..exprs.len() {
        s.push(' ');
        s.push_str(&print_expr(&exprs[i]));
    }

    s.push(')');
    return s;
}

fn primary_string(primary: &Primary) -> String {
    match primary {
        Primary::None => return format!("None"),
        Primary::Boolean(bool) => return format!("{}", bool),
        Primary::Literal(literal) => return format!("{}", literal),
    }
}
