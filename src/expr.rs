use crate::object::literal_type::LiteralType;
use crate::scanner::token::Token;

pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> T;
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_call_expr(&mut self, expr: &Call) -> T;
    fn visit_get_expr(&mut self, expr: &Get) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_logical_expr(&mut self, expr: &Logical) -> T;
    fn visit_set_expr(&mut self, expr: &Set) -> T;
    fn visit_this_expr(&mut self, expr: &This) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
    fn visit_variable_expr(&mut self, expr: &Variable) -> T;
}

pub trait Accept<T> {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T;
}

#[derive(Clone, Debug)]
pub enum Expr {
    Assign(Box<Assign>),
    Binary(Box<Binary>),
    Call(Box<Call>),
    Get(Box<Get>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Logical(Box<Logical>),
    Set(Box<Set>),
    This(This),
    Unary(Box<Unary>),
    Variable(Variable),
}

impl<T> Accept<T> for Expr {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expr::Assign(e) => e.accept(visitor),
            Expr::Binary(e) => e.accept(visitor),
            Expr::Call(e) => e.accept(visitor),
            Expr::Get(e) => e.accept(visitor),
            Expr::Grouping(e) => e.accept(visitor),
            Expr::Literal(e) => e.accept(visitor),
            Expr::Logical(e) => e.accept(visitor),
            Expr::Set(e) => e.accept(visitor),
            Expr::This(e) => e.accept(visitor),
            Expr::Unary(e) => e.accept(visitor),
            Expr::Variable(e) => e.accept(visitor),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Box<Expr>) -> Box<Assign> {
        Box::new(Assign { name, value })
    }
}

impl<T> Accept<T> for Assign {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_assign_expr(self)
    }
}

#[derive(Clone, Debug)]
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
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Box<Call> {
        Box::new(Call {
            callee,
            paren,
            arguments,
        })
    }
}

impl<T> Accept<T> for Call {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_call_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}

impl Get {
    pub fn new(object: Box<Expr>, name: Token) -> Box<Get> {
        Box::new(Get { object, name })
    }
}

impl<T> Accept<T> for Get {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_get_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Box<Grouping> {
        Box::new(Grouping { expression })
    }
}

impl<T> Accept<T> for Grouping {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub value: LiteralType,
}

impl Literal {
    pub fn new(value: LiteralType) -> Literal {
        Literal { value }
    }
}

impl<T> Accept<T> for Literal {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Box<Logical> {
        Box::new(Logical {
            left,
            operator,
            right,
        })
    }
}

impl<T> Accept<T> for Logical {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_logical_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

impl Set {
    pub fn new(object: Box<Expr>, name: Token, value: Box<Expr>) -> Box<Set> {
        Box::new(Set {
            object,
            name,
            value,
        })
    }
}

impl<T> Accept<T> for Set {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_set_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct This {
    pub keyword: Token,
}

impl This {
    pub fn new(keyword: Token) -> This {
        This { keyword }
    }
}

impl<T> Accept<T> for This {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_this_expr(self)
    }
}

#[derive(Clone, Debug)]
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
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Variable {
        Variable { name }
    }
}

impl<T> Accept<T> for Variable {
    fn accept(&mut self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_variable_expr(self)
    }
}
