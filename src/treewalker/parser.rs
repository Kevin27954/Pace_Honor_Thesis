use crate::treewalker::{expr_types::Primary, token_types::get_keywords};

use super::{
    errors::{error, parse_error, CompileErrors},
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

        self.skip_comments_and_newlines();
        while !self.is_end() {
            match self.parse_decl() {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    self.synchronize();
                    has_error = true;
                    parse_error(err);
                }
            }

            self.skip_comments_and_newlines();
        }

        (exprs, has_error)
    }

    fn parse_decl(&mut self) -> Result<Stmt, CompileErrors> {
        let stmt: Stmt;

        if self.match_type(&[TokenType::LET]) {
            stmt = self.parse_var_decl()?;
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

    fn parse_stmt(&mut self) -> Result<Stmt, CompileErrors> {
        if self.match_type(&[TokenType::DO]) {
            return self.parse_do_block();
        }
        if self.match_type(&[TokenType::IF]) {
            return self.parse_if_stmt();
        }
        if self.match_type(&[TokenType::WHILE]) {
            return self.parse_while_stmt();
        }
        if self.match_type(&[TokenType::FOR]) {
            return self.parse_for_stmt();
        }

        return self.parse_expr_statement();
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, CompileErrors> {
        let init: Option<Stmt> = match self.match_type(&[TokenType::SEMICOLON]) {
            true => None,
            false => {
                let stmt = if self.match_type(&[TokenType::LET]) {
                    self.parse_var_decl()?
                } else {
                    self.parse_expr_statement()?
                };

                if !self.match_type(&[TokenType::SEMICOLON]) {
                    return Err(CompileErrors::ExpectSemiColon(self.peek().unwrap().clone()));
                }

                Some(stmt)
            }
        };

        let condition: Option<Expr> = match self.match_type(&[TokenType::SEMICOLON]) {
            true => None,
            false => {
                let expr = self.assignment()?;

                if !self.match_type(&[TokenType::SEMICOLON]) {
                    return Err(CompileErrors::ExpectSemiColon(self.peek().unwrap().clone()));
                }

                Some(expr)
            }
        };

        let operation: Option<Expr> = match self.match_type(&[TokenType::SEMICOLON]) {
            true => None,
            false => Some(self.assignment()?),
        };

        if !self.match_type(&[TokenType::DO]) {
            return Err(CompileErrors::ExpectKeywordDo(self.peek().unwrap().clone()));
        }

        let mut body = self.parse_do_block()?;

        if let Some(expr) = operation {
            if let Stmt::Block(ref mut stmts) = body {
                stmts.push(Stmt::Expression(expr));
            }
        }

        body = match condition {
            Some(expr) => Stmt::WhileStmt(expr, Box::new(body)),
            None => Stmt::WhileStmt(Expr::Primary(Primary::Boolean(true)), Box::new(body)),
        };

        if let Some(var_decl) = init {
            body = Stmt::Block(vec![var_decl, body]);
        }

        return Ok(body);
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
            init = Some(self.logcal_or()?);
        }

        return Ok(Stmt::VarDecl(identifier, init));
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, CompileErrors> {
        let start_if = self.previous();

        let expr = self.equality()?;
        if !self.match_type(&[TokenType::THEN]) {
            return Err(CompileErrors::ExpectThen(self.peek().unwrap().clone()));
        }
        self.match_type(&[TokenType::COMMENT]);
        if !self.match_type(&[TokenType::NEW_LINE]) {
            return Err(CompileErrors::ExpectNewLine(self.peek().unwrap().clone()));
        }

        let if_block = self.parse_then_block(&start_if)?;
        let mut else_block: Option<Stmt> = None;

        if self.previous().token_type == TokenType::ELSE {
            else_block = Some(self.parse_then_block(&start_if)?);
        }

        return Ok(Stmt::IfStmt(expr, Box::new(if_block), Box::new(else_block)));
    }

    fn parse_then_block(&mut self, start_if: &Token) -> Result<Stmt, CompileErrors> {
        let mut stmts: Vec<Stmt> = Vec::new();

        self.skip_comments_and_newlines();
        while !self.match_type(&[TokenType::END, TokenType::ELSE]) && !self.is_end() {
            stmts.push(self.parse_decl()?);
            self.skip_comments_and_newlines();
        }

        match self.previous().token_type {
            TokenType::ELSE | TokenType::END => {}
            _ => {
                return Err(CompileErrors::UnterminatedIf(start_if.clone()));
            }
        }

        return Ok(Stmt::Block(stmts));
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, CompileErrors> {
        let expr = self.assignment()?;

        if !self.match_type(&[TokenType::DO]) {
            return Err(CompileErrors::ExpectKeywordDo(self.peek().unwrap().clone()));
        }

        let while_body = self.parse_do_block()?;

        return Ok(Stmt::WhileStmt(expr, Box::new(while_body)));
    }

    fn parse_do_block(&mut self) -> Result<Stmt, CompileErrors> {
        let mut stmts: Vec<Stmt> = Vec::new();
        let start_do_token = self.previous();

        self.match_type(&[TokenType::COMMENT]);

        if !self.match_type(&[TokenType::NEW_LINE]) {
            return Err(CompileErrors::ExpectNewLine(self.peek().unwrap().clone()));
        }

        self.skip_comments_and_newlines();
        while !self.match_type(&[TokenType::END]) && !self.is_end() {
            stmts.push(self.parse_decl()?);
            self.skip_comments_and_newlines();
        }

        if self.previous().token_type != TokenType::END {
            return Err(CompileErrors::UnterminatedDo(start_do_token));
        }

        return Ok(Stmt::Block(stmts));
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
        let expr = self.logcal_or()?;

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

    fn logcal_or(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.logical_and()?;

        while self.match_type(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.logical_and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn logical_and(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.equality()?;

        while self.match_type(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
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
            match self.unary()? {
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

        return self.call();
    }

    fn call(&mut self) -> Result<Expr, CompileErrors> {
        let mut expr = self.primary()?;

        while self.match_type(&[TokenType::LEFT_PAREN]) {
            expr = self.finish_call(expr)?;
        }

        return Ok(expr);
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

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, CompileErrors> {
        let mut arguments: Vec<Expr> = Vec::new();

        if self.peek().unwrap().token_type != TokenType::RIGHT_PAREN {
            loop {
                if arguments.len() > 16 {
                    error(self.peek().unwrap().line, "Too many arguments".to_string());
                }

                arguments.push(self.assignment()?);
                if !self.match_type(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        if !self.match_type(&[TokenType::RIGHT_PAREN]) {
            unimplemented!("Expected right parenthesis, function not closed.")
        }

        return Ok(Expr::Call(Box::new(expr), self.previous(), arguments));
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

    fn skip_comments_and_newlines(&mut self) {
        while self.match_type(&[TokenType::COMMENT, TokenType::NEW_LINE]) {}
    }
}
