use super::token::Literal;
use super::token::Token;

pub enum Expr {
    Litearl(Option<ExprLitearl>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

pub enum ExprLitearl {
    Literal(Literal),
    Boolean(bool),
}
