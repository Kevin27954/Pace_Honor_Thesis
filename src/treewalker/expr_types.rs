use super::token::Literal;
use super::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    //Equality(Equality),
    //Comparison(Comparison),
    //Term(Term),
    //Factor(Factor),
    Unary(Unary),
    Primary(Primary),
    Group(Box<Expr>),
}

// Token = '!=', '=='

//#[derive(Debug)]
//pub enum Equality {
//    EqExpr(Comparison, Option<Token>, Option<Comparison>),
//}
//
//// Token = '<', '>', '<=', '>='
//#[derive(Debug)]
//pub enum Comparison {
//    CmpExpr(Term, Option<Token>, Option<Term>),
//}
//
//// Token = '+', '-'
//#[derive(Debug)]
//pub enum Term {
//    TermExpr(Factor, Option<Token>, Option<Factor>),
//}
//
//// Token = '/', '*'
//#[derive(Debug)]
//pub enum Factor {
//    FactorExpr(Unary, Option<Token>, Option<Unary>),
//}

// Token = '!', '-'
#[derive(Debug, Clone)]
pub enum Unary {
    UnaryExpr(Token, Box<Expr>),
    //Primary(Primary),
}

// Wrapped in option for nil/None type
#[derive(Debug, Clone)]
pub enum Primary {
    Literal(Literal),
    Boolean(bool),
    None,
}