use super::token::Literal;
use super::token::Token;

/*

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

* */

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Equality(Box<Equality<'a>>),
    Comparison(Box<Comparison<'a>>),
    Term(Box<Term<'a>>),
    Factor(Box<Factor<'a>>),
    Unary(Box<Unary<'a>>),
    Primary(Box<Primary<'a>>),
}

// Token = '!=', '=='
pub enum Equality<'a> {
    EqExpr(Comparison<'a>, Option<Token>, Option<Comparison<'a>>),
}

// Token = '<', '>', '<=', '>='
pub enum Comparison<'a> {
    CmpExpr(Term<'a>, Option<Token>, Option<Term<'a>>),
}

// Token = '+', '-'
pub enum Term<'a> {
    TermExpr(Factor<'a>, Option<Token>, Option<Factor<'a>>),
}

// Token = '/', '*'
pub enum Factor<'a> {
    FactorExpr(Unary<'a>, Option<Token>, Option<Unary<'a>>),
}

// Token = '!', '-'
pub enum Unary<'a> {
    UnaryExpr(Token, Box<Unary<'a>>),
    Primary(Option<Primary<'a>>),
}

// Wrapped in option for nil/None type
pub enum Primary<'a> {
    Literal(Literal),
    Boolean(bool),
    Group(Expr<'a>),
}
