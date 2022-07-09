use crate::environment::Environment;
use crate::expr::{self, Accept as AcceptExpr, Binary, Expr, Grouping, Literal, Unary};
use crate::global_function;
use crate::lang_error::LangError;
use crate::scanner::literal_type::{self, LiteralType};
use crate::scanner::token::*;
use crate::stmt::{self, Accept as AcceptStmt, Stmt};
use crate::user_definable_object::UserDefinableObject;
use substring::Substring;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Environment::new(None);
        let clock_function = UserDefinableObject::Callable(Box::new(global_function::Clock::new()));
        globals.define("clock".to_string(), clock_function);
        Interpreter {
            environment: globals,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LangError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LangError> {
        statement.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), LangError> {
        self.environment = environment;
        let result = || -> Result<(), LangError> {
            for statement in statements {
                self.execute(statement)?;
            }
            Ok(())
        }();
        // here is executing inside the block
        // which obviously means self.environment has a parent environment
        // so the interpreter panics if it has no parent environment
        self.environment = self.environment.get_parent_environment().unwrap();
        result
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<UserDefinableObject, LangError> {
        expr.clone().accept(self)
    }
}

impl stmt::Visitor<Result<(), LangError>> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Result<(), LangError> {
        let previous_environment = Some(Box::new(self.environment.clone()));
        self.execute_block(&stmt.statements, Environment::new(previous_environment))?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> Result<(), LangError> {
        self.evaluate(&Box::new(stmt.clone().expression))?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Result<(), LangError> {
        let condition = self
            .evaluate(&Box::new(stmt.condition.clone()))?
            .fetch_value();
        if literal_type::is_truthy(condition) {
            self.execute(&stmt.then_statement)?;
        } else if let Some(else_statement) = stmt.else_statement.clone() {
            self.execute(&else_statement)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Result<(), LangError> {
        let value = self.evaluate(&Box::new(stmt.clone().expression))?;
        println!("{}", stringify_object(value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &stmt::Var) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        let value = self.evaluate(&Box::new(cloned_stmt.initializer))?;
        self.environment.define(cloned_stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Result<(), LangError> {
        loop {
            let condition = self
                .evaluate(&Box::new(stmt.condition.clone()))?
                .fetch_value();
            if !literal_type::is_truthy(condition) {
                break;
            }
            self.execute(&stmt.body)?
        }
        Ok(())
    }
}

impl expr::Visitor<Result<UserDefinableObject, LangError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<UserDefinableObject, LangError> {
        let left = self.evaluate(&expr.left)?.fetch_value();
        let right = self.evaluate(&expr.right)?.fetch_value();
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
        Ok(UserDefinableObject::Value(value))
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<UserDefinableObject, LangError> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            let evaluated_arg = self.evaluate(&Box::new(arg.clone()))?.fetch_value();
            arguments.push(evaluated_arg);
        }
        let function = if let UserDefinableObject::Callable(c) = callee {
            c
        } else {
            return Err(LangError::RuntimeError(
                "Can only call functions and classes.".to_string(),
                expr.clone().paren,
            ));
        };
        if function.arity() != arguments.len() {
            let error_message = format!(
                "Expected {} arguments but got {}.",
                function.arity(),
                arguments.len()
            );
            return Err(LangError::RuntimeError(error_message, expr.clone().paren));
        }
        let ret = function.call(self, arguments);
        Ok(UserDefinableObject::Value(ret))
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<UserDefinableObject, LangError> {
        self.evaluate(&expr.expression)
    }

    fn visit_logical_expr(
        &mut self,
        expr: &expr::Logical,
    ) -> Result<UserDefinableObject, LangError> {
        let left = self.evaluate(&expr.left)?.fetch_value();
        if let TokenType::Or = expr.operator.token_type {
            if literal_type::is_truthy(left.clone()) {
                return Ok(UserDefinableObject::Value(left));
            }
        } else {
            if !literal_type::is_truthy(left.clone()) {
                return Ok(UserDefinableObject::Value(left));
            }
        }

        self.evaluate(&expr.right)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<UserDefinableObject, LangError> {
        let right = self.evaluate(&expr.right)?.fetch_value();
        let value = match expr.operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => panic!("invalid Expression"),
        };
        if let LiteralType::Error(message) = value {
            return Err(LangError::RuntimeError(message, expr.clone().operator));
        }
        Ok(UserDefinableObject::Value(value))
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<UserDefinableObject, LangError> {
        let value = UserDefinableObject::Value(expr.value.clone());
        Ok(value)
    }

    fn visit_variable_expr(
        &mut self,
        expr: &expr::Variable,
    ) -> Result<UserDefinableObject, LangError> {
        self.environment.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<UserDefinableObject, LangError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(expr.clone().name, value.clone())?;
        Ok(value)
    }
}

fn stringify_object(value: UserDefinableObject) -> String {
    match value {
        UserDefinableObject::Value(v) => stringify_literal(v),
        _ => panic!(""),
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
