use super::{expr_types::Expr, token::Token};

pub enum Stmt {
    Expression(Expr),
    VarDecl(Token, Option<Expr>),
}
