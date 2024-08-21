use super::{expr_types::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    VarDecl(Token, Option<Expr>),
    Block(Vec<Stmt>),
}
