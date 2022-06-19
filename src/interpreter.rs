use crate::environment::Environment;
use crate::expr::{self, Accept as AcceptExpr, Binary, Expr, Grouping, Literal, Unary};
use crate::lang_error::LangError;
use crate::scanner::literal_type::{self, LiteralType};
use crate::scanner::token::*;
use crate::stmt::{self, Accept as AcceptStmt, Stmt};
use substring::Substring;

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ..Default::default()
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LangError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), LangError> {
        statement.accept(self)
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<LiteralType, LangError> {
        expr.clone().accept(self)
    }
}

impl stmt::Visitor<Result<(), LangError>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> Result<(), LangError> {
        self.evaluate(&Box::new(stmt.clone().expression))?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Result<(), LangError> {
        let value = self.evaluate(&Box::new(stmt.clone().expression))?;
        println!("{}", stringify_literal(value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &stmt::Var) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        let value = self.evaluate(&Box::new(cloned_stmt.initializer))?;
        self.environment.define(cloned_stmt.name.lexeme, value);
        Ok(())
    }
}

impl expr::Visitor<Result<LiteralType, LangError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<LiteralType, LangError> {
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

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LiteralType, LangError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<LiteralType, LangError> {
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

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LiteralType, LangError> {
        Ok(expr.value.clone())
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<LiteralType, LangError> {
        self.environment.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<LiteralType, LangError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(expr.clone().name, value.clone())?;
        Ok(value)
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
