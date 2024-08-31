use std::collections::{HashMap, LinkedList};

use crate::treewalker::errors::error;

use super::{
    expr_types::{Expr, Unary},
    interpreter::Interpreter,
    statements::Stmt,
    token::Token,
};

#[derive(Clone, PartialEq)]
enum ScopeType {
    NONE,
    FUNCTION,
}

#[derive(PartialEq)]
enum VarInfo {
    Uninit,
    Init,
    Called,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scope: LinkedList<HashMap<String, VarInfo>>,
    scope_type: ScopeType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scope: LinkedList::new(),
            scope_type: ScopeType::NONE,
        }
    }

    pub fn resolve(&mut self, stmts: &Vec<Stmt>) -> bool {
        let mut has_error = false;
        for stmt in stmts {
            if self.resolve_stmt(stmt) {
                has_error = true;
            }
        }
        has_error
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> bool {
        let mut has_error = false;
        match stmt {
            Stmt::Block(block) => {
                self.add_scope();
                has_error = self.resolve(block);
                self.pop_scope();
            }
            Stmt::VarDecl(identifer, initializer) => {
                self.declare(identifer.lexeme.to_string());
                if let Some(ref val) = initializer {
                    has_error = self.resolve_expr(val);
                }
                self.define(identifer.lexeme.to_string());
            }

            Stmt::RuntimeFunctions(identifier, _, _) => {
                self.declare(identifier.to_string());
                self.define(identifier.to_string());

                self.resolve_fn(&stmt, ScopeType::FUNCTION);
            }
            Stmt::Expression(expr) => has_error = self.resolve_expr(expr),

            Stmt::IfStmt(expr, if_body, else_body) => {
                self.resolve_expr(expr);
                self.resolve_stmt(if_body.as_ref());
                if let Some(else_block) = else_body.as_ref() {
                    self.resolve_stmt(else_block);
                }
            }
            Stmt::Return(token, expr) => {
                if self.scope_type == ScopeType::NONE {
                    error(token.line, "Can't return at top levl scope".to_string());
                    has_error = true;
                }

                if let Some(expr) = expr {
                    self.resolve_expr(expr);
                }

                return has_error;
            }

            Stmt::WhileStmt(expr, body) => {
                self.resolve_expr(expr);
                self.resolve_stmt(body.as_ref());
            }
            Stmt::StructStmt(token, fields) => {
                self.declare(token.lexeme.to_string());
                self.define(token.lexeme.to_string());
            }
        }

        has_error
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Variable(token) => match self.scope.front() {
                Some(env) => {
                    if let Some(is_init) = env.get(&token.lexeme) {
                        if !self.scope.is_empty() && &VarInfo::Uninit == is_init {
                            error(
                                token.line,
                                "Can't use an uninitalized variable to initalize itself"
                                    .to_string(),
                            );
                            return true;
                        }
                    }

                    self.resolve_local(expr, token);
                }
                None => {}
            },
            Expr::Dot(user_struct, _field) => {
                self.resolve_expr(user_struct.as_ref());
            }
            Expr::Set(user_struct, _field, value) => {
                self.resolve_expr(user_struct.as_ref());
                self.resolve_expr(value.as_ref());
            }

            Expr::Assignment(token, expr) => {
                self.resolve_expr(expr.as_ref());
                self.resolve_local(expr.as_ref(), &token)
            }
            Expr::Binary(left, _operator, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }

            Expr::Call(callee, _, params) => {
                self.resolve_expr(callee.as_ref());

                for param in params {
                    self.resolve_expr(param);
                }
            }
            Expr::Group(group) => {
                self.resolve_expr(group.as_ref());
            }

            Expr::Primary(_) => {}
            Expr::Logical(left, _operator, right) => {
                self.resolve_expr(left.as_ref());
                self.resolve_expr(right.as_ref());
            }
            Expr::Unary(unary) => {
                let Unary::UnaryExpr(_, expr) = unary;
                self.resolve_expr(expr.as_ref());
            }
        }
        return false;
    }

    fn resolve_fn(&mut self, func: &Stmt, scope_type: ScopeType) {
        let prev_scope = self.scope_type.clone();
        self.scope_type = scope_type;

        self.add_scope();

        match func {
            Stmt::RuntimeFunctions(_, params, body) => {
                for param in params {
                    self.declare(param.lexeme.to_string());
                    self.define(param.lexeme.to_string());
                }

                self.resolve_stmt(body.as_ref());
            }
            _ => {
                unreachable!();
            }
        }

        self.pop_scope();
        self.scope_type = prev_scope;
    }

    fn resolve_local(&mut self, expr: &Expr, token: &Token) {
        let length = self.scope.len();
        let mut iter = self.scope.iter_mut();

        for i in (0..length).rev() {
            if let Some(scope) = iter.next() {
                if scope.contains_key(&token.lexeme) {
                    scope.insert(token.lexeme.to_string(), VarInfo::Called);
                    self.interpreter.resolve(expr, self.scope.len() - i);
                    return;
                }
            }
        }
    }

    fn declare(&mut self, identifier: String) {
        if self.scope.is_empty() {
            return;
        }

        let env = self.scope.front_mut();
        match env {
            Some(var_map) => {
                if var_map.contains_key(&identifier) {
                    error(0, "Variable already exist in this scope".to_string());
                }

                var_map.insert(identifier, VarInfo::Uninit);
            }
            None => {}
        };
    }

    fn define(&mut self, identifier: String) {
        if self.scope.is_empty() {
            return;
        }

        let env = self.scope.front_mut();
        match env {
            Some(var_map) => {
                var_map.insert(identifier, VarInfo::Init);
            }
            None => {}
        };
    }

    fn add_scope(&mut self) {
        self.scope.push_front(HashMap::new());
    }

    fn pop_scope(&mut self) {
        let local_scope = self.scope.pop_front();
        if let Some(scope) = local_scope {
            for (i, (name, var_info)) in scope.iter().enumerate() {
                if var_info != &VarInfo::Called {
                    error(0, format!("Warning: {} is not being used.", name));
                }
            }
        }
    }
}
