use super::token::Literal;
use super::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Unary),
    Primary(Primary),
    Group(Box<Expr>),
}

// Token = '!=', '=='
//pub enum Equality {

// Token = '<', '>', '<=', '>='
//pub enum Comparison { }

// Token = '+', '-'
//pub enum Term { }

//// Token = '/', '*'
//pub enum Factor { }

// Token = '!', '-'
#[derive(Debug, Clone)]
pub enum Unary {
    UnaryExpr(Token, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Literal(Literal),
    Boolean(bool),
    None,
}
