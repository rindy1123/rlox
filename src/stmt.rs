use crate::expr::Expr;
use crate::scanner::token::Token;

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> T;
    fn visit_class_stmt(&mut self, stmt: &Class) -> T;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_function_stmt(&mut self, stmt: &Function) -> T;
    fn visit_if_stmt(&mut self, stmt: &If) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_return_stmt(&mut self, stmt: &Return) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_while_stmt(&mut self, stmt: &While) -> T;
}

pub trait Accept<T> {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T;
}

#[derive(Clone)]
pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(Box<If>),
    Print(Print),
    Return(Return),
    Var(Var),
    While(Box<While>),
}

impl<T> Accept<T> for Stmt {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Stmt::Block(e) => e.accept(visitor),
            Stmt::Class(e) => e.accept(visitor),
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Function(e) => e.accept(visitor),
            Stmt::If(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
            Stmt::Return(e) => e.accept(visitor),
            Stmt::Var(e) => e.accept(visitor),
            Stmt::While(e) => e.accept(visitor),
        }
    }
}

#[derive(Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Block {
        Block { statements }
    }
}

impl<T> Accept<T> for Block {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_block_stmt(self)
    }
}

#[derive(Clone)]
pub struct Class {
    pub name: Token,
    pub methods: Vec<Function>,
}

impl Class {
    pub fn new(name: Token, methods: Vec<Function>) -> Class {
        Class { name, methods }
    }
}

impl<T> Accept<T> for Class {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_class_stmt(self)
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
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Function {
        Function { name, params, body }
    }
}

impl<T> Accept<T> for Function {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_function_stmt(self)
    }
}

#[derive(Clone)]
pub struct If {
    pub condition: Expr,
    pub then_statement: Box<Stmt>,
    pub else_statement: Option<Box<Stmt>>,
}

impl If {
    pub fn new(
        condition: Expr,
        then_statement: Box<Stmt>,
        else_statement: Option<Box<Stmt>>,
    ) -> Box<If> {
        Box::new(If {
            condition,
            then_statement,
            else_statement,
        })
    }
}

impl<T> Accept<T> for If {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_if_stmt(self)
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
pub struct Return {
    pub keyword: Token,
    pub value: Expr,
}

impl Return {
    pub fn new(keyword: Token, value: Expr) -> Return {
        Return { keyword, value }
    }
}

impl<T> Accept<T> for Return {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_return_stmt(self)
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

#[derive(Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, body: Box<Stmt>) -> Box<While> {
        Box::new(While { condition, body })
    }
}

impl<T> Accept<T> for While {
    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {
        visitor.visit_while_stmt(self)
    }
}
