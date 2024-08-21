use crate::treewalker::{expr_types::Primary, token_types::get_keywords};

use super::{
    errors::{parse_error, CompileErrors},
    expr_types::{Expr, Unary},
    statements::Stmt,
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

    pub fn parse(&mut self) -> (Vec<Stmt>, bool) {
        let mut exprs: Vec<Stmt> = Vec::new();
        let mut has_error = false;
        while !self.is_end() {
            while self.match_type(&[TokenType::COMMENT, TokenType::NEW_LINE]) {}
            // Scenario when it is all comments
            if self.is_end() {
                break;
            }

            match self.parse_decl() {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    self.synchronize();
                    has_error = true;
                    parse_error(err);
                }
            }
        }

        (exprs, has_error)
    }

    fn parse_decl(&mut self) -> Result<Stmt, CompileErrors> {
        let stmt: Stmt;

        if self.match_type(&[TokenType::LET]) {
            stmt = self.parse_var_decl()?;
        } else if self.match_type(&[TokenType::DO]) {
            stmt = self.parse_block()?;
        } else if self.match_type(&[TokenType::END]) {
            return Err(CompileErrors::ExpectKeywordDo(self.previous()));
        } else {
            stmt = self.parse_stmt()?;
        }

        let token = self.peek().unwrap().clone();
        match token.token_type {
            // Expects one of the following after each Statement;
            TokenType::NEW_LINE | TokenType::EOF | TokenType::COMMENT => {
                self.advance();
            }
            _ => {
                return Err(CompileErrors::ExpectNewLine(token));
            }
        }

        return Ok(stmt);
    }

    fn parse_block(&mut self) -> Result<Stmt, CompileErrors> {
        let mut stmts: Vec<Stmt> = Vec::new();
        let start_do_token = self.previous();

        // If there is a comment, consume it
        self.match_type(&[TokenType::COMMENT]);

        if !self.match_type(&[TokenType::NEW_LINE]) {
            return Err(CompileErrors::ExpectNewLine(self.peek().unwrap().clone()));
        }

        while !self.match_type(&[TokenType::END]) && !self.is_end() {
            while self.match_type(&[TokenType::COMMENT, TokenType::NEW_LINE]) {}
            if self.is_end() {
                return Err(CompileErrors::UnterminatedDo(start_do_token));
            }
            if self.match_type(&[TokenType::END]) {
                break;
            }

            // So we can parse out all the inner scope
            // But only run the ones with correct syntax
            match self.parse_decl() {
                Ok(expr) => stmts.push(expr),
                Err(err) => {
                    self.synchronize();
                    parse_error(err);
                }
            }
        }

        if self.previous().token_type != TokenType::END {
            return Err(CompileErrors::UnterminatedDo(start_do_token));
        }

        return Ok(Stmt::Block(stmts));
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, CompileErrors> {
        if !self.match_type(&[TokenType::IDENTIFIER]) {
            let token = self.peek().unwrap().clone();

            if get_keywords().contains_key(&token.lexeme) {
                return Err(CompileErrors::KeywordAsIdentifier(token));
            }

            return Err(CompileErrors::InvalidIdentifier(token));
        }

        let identifier = self.previous();

        let mut init: Option<Expr> = None;
        if self.match_type(&[TokenType::EQUAL]) {
            init = Some(self.equality()?);
        }

        return Ok(Stmt::VarDecl(identifier, init));
    }

    fn parse_stmt(&mut self) -> Result<Stmt, CompileErrors> {
        // Incase there are other statements to parse
        return self.parse_expr_statement();
    }

    fn parse_expr_statement(&mut self) -> Result<Stmt, CompileErrors> {
        return Ok(Stmt::Expression(self.assignment()?));
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

    fn assignment(&mut self) -> Result<Expr, CompileErrors> {
        let expr = self.equality()?;

        if self.match_type(&[TokenType::EQUAL]) {
            let token = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                return Ok(Expr::Assignment(var, Box::new(value)));
            }

            return Err(CompileErrors::ExpectExpr(token));
        }

        return Ok(expr);
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

                let res = Ok(Expr::Group(Box::new(self.equality()?)));

                if self.peek().unwrap().token_type != TokenType::RIGHT_PAREN {
                    return Err(CompileErrors::UnterminatedParenthesis(left_paren));
                }

                // consumes the right parenthesis
                self.advance();
                return res;
            }
            TokenType::IDENTIFIER => {
                return Ok(Expr::Variable(self.previous()));
            }
            _ => {
                // Shouldn't consume token is doesn't match
                self.current -= 1;
                let mut err_token: Token = self.peek().unwrap().clone();
                // An edge case for: '1 - //comment'
                if err_token.token_type == TokenType::COMMENT
                    || err_token.token_type == TokenType::NEW_LINE
                {
                    err_token = self.previous();
                }

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
