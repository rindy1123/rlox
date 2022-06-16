use crate::expr::{Accept, Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::lang_error::LangError;
use crate::scanner::literal_type::{self, LiteralType};
use crate::scanner::token::*;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Expr) {
        let value = self.evaluate(&Box::new(expr));
        print!("{}", stringify_literal(value))
    }

    fn evaluate(&self, expr: &Box<Expr>) -> LiteralType {
        expr.accept(self)
    }
}

impl Visitor<LiteralType> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary) -> LiteralType {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);
        match expr.operator.token_type {
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Greater => literal_type::comparison::gt(left, right),
            TokenType::GreaterEqual => literal_type::comparison::ge(left, right),
            TokenType::Less => literal_type::comparison::lt(left, right),
            TokenType::LessEqual => literal_type::comparison::le(left, right),
            TokenType::BangEqual => literal_type::convert_bool_to_literal_bool(left != right),
            TokenType::EqualEqual => literal_type::convert_bool_to_literal_bool(left == right),
            _ => panic!("invalid Expression"),
        }
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> LiteralType {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&self, expr: &Unary) -> LiteralType {
        let right = self.evaluate(&expr.right);
        match expr.operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => panic!("invalid Expression"),
        }
    }

    fn visit_literal_expr(&self, expr: &Literal) -> LiteralType {
        expr.value.clone()
    }
}

fn stringify_literal(value: LiteralType) -> String {
    match value {
        LiteralType::Nil => "nil".to_string(),
        LiteralType::Num(n) => n.to_string(),
        LiteralType::Str(string) => string,
        LiteralType::True => "true".to_string(),
        LiteralType::False => "false".to_string(),
        // FIXME
        _ => panic!("FIXME"),
    }
}
