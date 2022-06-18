use crate::expr::Expr;

pub trait Visitor<T> {
    fn visit_expression_stmt(&self, stmt: &Expression) -> T;
    fn visit_print_stmt(&self, stmt: &Print) -> T;
}

pub trait Accept<T> {
    fn accept(&self, visitor: &impl Visitor<T>) -> T;
}

#[derive(Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

impl<T> Accept<T> for Stmt {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct Expression {
    pub expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Expression {
        Expression {
            expression,
        }
    }
}

impl<T> Accept<T> for Expression {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_expression_stmt(self)
    }
}

#[derive(Clone)]
pub struct Print {
    pub expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Print {
        Print {
            expression,
        }
    }
}

impl<T> Accept<T> for Print {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_print_stmt(self)
    }
}

