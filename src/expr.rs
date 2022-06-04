use crate::scanner::token::{LiteralType, Token};
pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
}

pub trait Accept<T> {
    fn accept(&self, visitor: &impl Visitor<T>) -> T;
}

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

impl<T> Accept<T> for Expr {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Binary(e) => e.accept(visitor),
            Expr::Grouping(e) => e.accept(visitor),
            Expr::Literal(e) => e.accept(visitor),
            Expr::Unary(e) => e.accept(visitor),
        }
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Binary {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl<T> Accept<T> for Binary {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Grouping {
        Grouping { expression }
    }
}

impl<T> Accept<T> for Grouping {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    pub value: LiteralType,
}

impl Literal {
    pub fn new(value: LiteralType) -> Literal {
        Literal { value }
    }
}

impl<T> Accept<T> for Literal {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Unary {
        Unary { operator, right }
    }
}

impl<T> Accept<T> for Unary {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}
