use crate::scanner::literal_type::LiteralType;
use crate::scanner::token::Token;

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
    fn visit_variable_expr(&self, expr: &Variable) -> T;
}

pub trait Accept<T> {
    fn accept(&self, visitor: &impl Visitor<T>) -> T;
}

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Variable(Box<Variable>),
}

impl<T> Accept<T> for Expr {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        match self {
            Expr::Binary(e) => e.accept(visitor),
            Expr::Grouping(e) => e.accept(visitor),
            Expr::Literal(e) => e.accept(visitor),
            Expr::Unary(e) => e.accept(visitor),
            Expr::Variable(e) => e.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Box<Binary> {
        Box::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl<T> Accept<T> for Binary {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Box<Grouping> {
        Box::new(Grouping { expression })
    }
}

impl<T> Accept<T> for Grouping {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralType,
}

impl Literal {
    pub fn new(value: LiteralType) -> Box<Literal> {
        Box::new(Literal { value })
    }
}

impl<T> Accept<T> for Literal {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Box<Unary> {
        Box::new(Unary { operator, right })
    }
}

impl<T> Accept<T> for Unary {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

#[derive(Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Box<Variable> {
        Box::new(Variable { name })
    }
}

impl<T> Accept<T> for Variable {
    fn accept(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_variable_expr(self)
    }
}
