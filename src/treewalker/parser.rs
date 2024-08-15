use crate::treewalker::expr_types::Primary;

use super::{
    errors::{error, CompileErrors},
    expr_types::{Expr, Unary},
    token::Token,
    token_types::TokenType,
};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn match_type(&mut self, want: &[TokenType]) -> bool {
        if self.is_end() {
            return false;
        }

        for token_type in want {
            if &self.peek().unwrap().token_type == token_type {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn is_end(&self) -> bool {
        return self.tokens.get(self.current).unwrap().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.current);
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut exprs: Vec<Expr> = Vec::new();
        while !self.is_end() {
            match self.equality() {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    error(0, format!("{}", err));
                }
            }
        }
        exprs
    }

    fn equality(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.comparison()?;

        while self.match_type(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.term()?;

        while self.match_type(&[
            TokenType::GREATER,
            TokenType::LESS,
            TokenType::GREATER_EQUAL,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.factor()?;

        while self.match_type(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.unary()?;

        while self.match_type(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, CompileErrors> {
        if self.match_type(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            match right {
                Expr::Primary(primary) => {
                    return Ok(Expr::Unary(Unary::UnaryExpr(
                        operator,
                        Box::new(Unary::Primary(primary)),
                    )));
                }
                Expr::Unary(unary) => {
                    return Ok(Expr::Unary(Unary::UnaryExpr(operator, Box::new(unary))));
                }
                _ => {}
            }
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, CompileErrors> {
        match self.advance().token_type {
            TokenType::TRUE => {
                return Ok(Expr::Primary(Primary::Boolean(true)));
            }
            TokenType::FALSE => {
                return Ok(Expr::Primary(Primary::Boolean(false)));
            }
            TokenType::NONE => {
                return Ok(Expr::Primary(Primary::None));
            }
            TokenType::NUMBERS | TokenType::STRING => {
                return Ok(Expr::Primary(Primary::Literal(
                    self.previous().litearl.to_owned().unwrap(),
                )));
            }
            TokenType::LEFT_PAREN => {
                let res = Ok(Expr::Group(Box::new(self.equality()?)));
                if self.advance().token_type != TokenType::RIGHT_PAREN {
                    return Err(CompileErrors::UnterminatedParenthesis);
                }
                return res;
            }
            _ => {
                return Err(CompileErrors::UnknownError(
                    "Unknown Token or Unimplemented Token".to_string(),
                ));
            }
        }
    }
}
