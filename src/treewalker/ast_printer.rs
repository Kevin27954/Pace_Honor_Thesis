use super::{
    expr_types::{Expr, Primary, Unary},
    statements::Stmt,
};

pub struct ASTPrinter {
    pub scope_level: usize,
}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter { scope_level: 0 }
    }

    pub fn print_ast(&mut self, stmt: &Stmt) -> String {
        match stmt {
            Stmt::VarDecl(var, val) => match val {
                Some(val) => {
                    format!("(let (= {} {}))", var.lexeme, self.print_expr(&val))
                }
                None => {
                    format!("(let {})", var.lexeme)
                }
            },
            Stmt::Expression(expr) => self.print_expr(&expr),
            Stmt::Block(stmts) => {
                let mut ast = String::new();
                ast.push_str(format!("{}(do", self.tab_space()).as_str());

                self.scope_level += 1;
                for stmt in stmts {
                    ast.push_str(
                        format!("\n{}{}", self.tab_space(), self.print_ast(&stmt)).as_str(),
                    );
                }

                self.scope_level -= 1;
                ast.push_str(format!("\n{}end", self.tab_space()).as_str());

                ast
            }
            Stmt::IfStmt(expr, if_block, else_block) => {
                let mut ast = String::new();
                ast.push_str(format!("{}(if {}", self.tab_space(), self.print_expr(expr)).as_str());
                self.scope_level += 1;
                ast.push_str(
                    format!("\n{}{}", self.tab_space(), self.print_ast(if_block)).as_str(),
                );
                ast.push_str(format!("{})", self.tab_space()).as_str());

                if let Some(block) = else_block.as_ref() {
                    ast.push_str(format!("\n{}(else", self.tab_space()).as_str());
                    ast.push_str(
                        format!("\n{}{})", self.tab_space(), self.print_ast(block)).as_str(),
                    );
                }
                self.scope_level -= 1;

                ast
            }
            Stmt::WhileStmt(expr, while_body) => {
                let mut ast = String::new();
                ast.push_str("(while ");
                ast.push_str(format!("{}", self.print_expr(expr)).as_str());
                ast.push_str(format!("\n{}))", self.print_ast(while_body)).as_str());

                ast
            }
        }
    }

    pub fn print_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Primary(primary) => self.primary_string(primary),
            Expr::Unary(unary) => match unary {
                Unary::UnaryExpr(operator, unary) => {
                    let temp = unary.as_ref();
                    self.parenthesize(&operator.lexeme, &[&temp.clone()])
                }
            },
            Expr::Binary(left, operator, right) => {
                let slice = &[left.as_ref(), right.as_ref()];
                self.parenthesize(&operator.lexeme, slice)
            }
            Expr::Group(expr) => self.parenthesize(&String::from("group"), &[expr.as_ref()]),
            Expr::Variable(var) => var.lexeme.to_string(),
            Expr::Assignment(var, expr) => self.parenthesize(&var.lexeme, &[expr.as_ref()]),
            Expr::Logical(left, operator, right) => {
                let slice = &[left.as_ref(), right.as_ref()];
                self.parenthesize(&operator.lexeme, slice)
            }
        }
    }

    fn parenthesize(&self, name: &String, exprs: &[&Expr]) -> String {
        let mut s = String::new();
        s.push('(');
        s.push_str(&name);

        for i in 0..exprs.len() {
            s.push(' ');
            s.push_str(&self.print_expr(&exprs[i]));
        }

        s.push(')');
        return s;
    }

    fn primary_string(&self, primary: &Primary) -> String {
        match primary {
            Primary::None => return format!("None"),
            Primary::Boolean(bool) => return format!("{}", bool),
            Primary::Literal(literal) => return format!("{}", literal),
        }
    }

    fn tab_space(&self) -> String {
        let mut tabs = String::new();
        for _ in 1..self.scope_level {
            tabs.push('\t');
        }
        tabs
    }
}
