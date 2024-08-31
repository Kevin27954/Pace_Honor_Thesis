use super::{expr_types::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    VarDecl(Token, Option<Expr>),
    Block(Vec<Stmt>),
    IfStmt(Expr, Box<Stmt>, Box<Option<Stmt>>),
    WhileStmt(Expr, Box<Stmt>),
    RuntimeFunctions(String, Vec<Token>, Box<Stmt>),
    Return(Token, Option<Expr>),
}
