use crate::expr::{Accept, Binary, Expr, Grouping, Literal, Unary, Visitor};
use crate::lang_error::LangError;
use crate::scanner::literal_type::{self, LiteralType};
use crate::scanner::token::*;
use substring::Substring;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Expr) -> Result<(), LangError> {
        let value = self.evaluate(&Box::new(expr.clone()))?;

        println!("{}", stringify_literal(value));
        Ok(())
    }

    fn evaluate(&self, expr: &Box<Expr>) -> Result<LiteralType, LangError> {
        expr.accept(self)
    }
}

impl Visitor<Result<LiteralType, LangError>> for Interpreter {
    fn visit_binary_expr(&self, expr: &Binary) -> Result<LiteralType, LangError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let value = match expr.operator.token_type {
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
        };
        if let LiteralType::Error(message) = value {
            return Err(LangError::RuntimeError(message, expr.clone().operator));
        }
        Ok(value)
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<LiteralType, LangError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&self, expr: &Unary) -> Result<LiteralType, LangError> {
        let right = self.evaluate(&expr.right)?;
        let value = match expr.operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => panic!("invalid Expression"),
        };
        if let LiteralType::Error(message) = value {
            return Err(LangError::RuntimeError(message, expr.clone().operator));
        }
        Ok(value)
    }

    fn visit_literal_expr(&self, expr: &Literal) -> Result<LiteralType, LangError> {
        Ok(expr.value.clone())
    }
}

fn stringify_literal(value: LiteralType) -> String {
    match value {
        LiteralType::Nil => "nil".to_string(),
        LiteralType::Num(n) => {
            let num_in_str = n.to_string();
            if num_in_str.ends_with(".0") {
                let str_len = num_in_str.len();
                return num_in_str.substring(0, str_len - 2).to_string();
            }
            num_in_str
        }
        LiteralType::Str(string) => string,
        LiteralType::True => "true".to_string(),
        LiteralType::False => "false".to_string(),
        LiteralType::Error(_) => panic!("Handle Error before stringifying"),
    }
}
