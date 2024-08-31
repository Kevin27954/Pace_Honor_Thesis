use std::hash::Hash;

use super::token::Literal;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Expr {
    Assignment(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Unary),
    Primary(Primary),
    Group(Box<Expr>),
    Variable(Token),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Unary {
    UnaryExpr(Token, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Primary {
    Literal(Literal),
    Boolean(bool),
    None,
}

impl Eq for Expr {}

