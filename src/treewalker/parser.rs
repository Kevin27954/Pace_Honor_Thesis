use crate::treewalker::expr_types::Primary;

use super::{
    errors::{parse_error, CompileErrors},
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

    pub fn parse(&mut self) -> (Vec<Expr>, bool) {
        let mut exprs: Vec<Expr> = Vec::new();
        let mut has_error = false;
        while !self.is_end() {
            while !self.is_end()
                && (self.peek().unwrap().token_type == TokenType::COMMENT
                    || self.peek().unwrap().token_type == TokenType::NEW_LINE)
            {
                self.advance();
            }

            // Scenario when it is all comments
            if self.is_end() {
                break;
            }

            match self.parse_token() {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    has_error = true;
                    parse_error(err);
                }
            }

            if self.peek().unwrap().token_type == TokenType::NEW_LINE {
                self.advance();
            }
        }

        (exprs, has_error)
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

    fn parse_token(&mut self) -> Result<Expr, CompileErrors> {
        return self.equality();
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
                        Box::new(Expr::Primary(primary)),
                    )));
                }
                Expr::Unary(unary) => {
                    return Ok(Expr::Unary(Unary::UnaryExpr(
                        operator,
                        Box::new(Expr::Unary(unary)),
                    )));
                }
                Expr::Group(group) => {
                    return Ok(Expr::Unary(Unary::UnaryExpr(
                        operator,
                        Box::new(Expr::Group(group)),
                    )));
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
            TokenType::NUMBER | TokenType::STRING => {
                let litearl = self.previous().litearl.to_owned().unwrap();
                return Ok(Expr::Primary(Primary::Literal(litearl)));
            }
            TokenType::LEFT_PAREN => {
                let left_paren = self.previous();

                if let Some(token) = self.peek() {
                    if token.token_type == TokenType::RIGHT_PAREN {
                        return Err(CompileErrors::EmptyParentheses(self.advance()));
                    } else if token.token_type == TokenType::EOF {
                        return Err(CompileErrors::UnterminatedParenthesis(left_paren));
                    }
                }

                let res = Ok(Expr::Group(Box::new(self.parse_token()?)));

                if self.peek().unwrap().token_type != TokenType::RIGHT_PAREN {
                    // Might need synchronize here -> THINK
                    self.synchronize();
                    return Err(CompileErrors::UnterminatedParenthesis(left_paren));
                }

                // consumes the right parenthesis
                self.advance();
                return res;
            }
            _ => {
                // Shouldn't consume token is doesn't match
                self.current -= 1;
                let err_token = self.peek().unwrap().clone();
                self.synchronize();
                return Err(CompileErrors::ExpectExpr(err_token));
            }
        }
    }

    // Sync consumes new line.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokenType::NEW_LINE {
                break;
            }
            match self.peek().unwrap().token_type {
                TokenType::IF
                | TokenType::LET
                | TokenType::FUNCTION
                | TokenType::STRUCT
                | TokenType::RETURN
                | TokenType::FOR
                | TokenType::WHILE => break,
                _ => {}
            }

            self.advance();
        }
    }
}
