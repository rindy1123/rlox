use crate::expr::Expr;
use crate::scanner::token::Token;

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
}

pub trait Accept<T> {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T;
}

#[derive(Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

impl<T> Accept<T> for Stmt {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
            Stmt::Var(e) => e.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct Expression {
    pub expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Expression {
        Expression { expression }
    }
}

impl<T> Accept<T> for Expression {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_expression_stmt(self)
    }
}

#[derive(Clone)]
pub struct Print {
    pub expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Print {
        Print { expression }
    }
}

impl<T> Accept<T> for Print {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_print_stmt(self)
    }
}

#[derive(Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Expr,
}

impl Var {
    pub fn new(name: Token, initializer: Expr) -> Var {
        Var { name, initializer }
    }
}

impl<T> Accept<T> for Var {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_var_stmt(self)
    }
}
