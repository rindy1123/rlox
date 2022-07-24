use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::Environment;
use crate::expr::{self, Accept as AcceptExpr, Binary, Expr, Grouping, Literal, Unary};
use crate::lang_error::LangError;
use crate::object::callable::global_function::Clock;
use crate::object::callable::lox_function::LoxFunction;
use crate::object::callable::CallableType;
use crate::object::literal_type::{self, LiteralType};
use crate::object::lox_class::LoxClass;
use crate::object::Object;
use crate::scanner::token::*;
use crate::stmt::{self, Accept as AcceptStmt, Stmt};

#[derive(Clone)]
pub struct Interpreter {
    pub environment: Rc<Environment>,
    globals: Rc<Environment>,
    locals: HashMap<u64, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Environment::new(None);
        let clock_function = Object::Callable(Clock::new());
        globals.define("clock".to_string(), clock_function);
        Interpreter {
            environment: globals.clone(),
            globals: globals.clone(),
            locals: HashMap::new(),
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

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<Environment>,
    ) -> Result<(), LangError> {
        let previous_environment = self.environment.clone();
        self.environment = environment.clone();
        let result = || -> Result<(), LangError> {
            for statement in statements {
                self.execute(statement)?;
            }
            Ok(())
        }();
        self.environment = previous_environment;
        result
    }

    fn evaluate(&mut self, expr: &Box<Expr>) -> Result<Object, LangError> {
        expr.clone().accept(self)
    }

    pub fn resolve(&mut self, token_id: u64, depth: usize) {
        self.locals.insert(token_id, depth);
    }

    fn look_up_variable(&self, name: Token) -> Result<Object, LangError> {
        if let Some(distance) = self.locals.get(&name.id) {
            return self.environment.get_at(*distance, name);
        }
        self.globals.get(&name)
    }
}

impl stmt::Visitor<Result<(), LangError>> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Result<(), LangError> {
        let previous_environment = Some(self.environment.clone());
        self.execute_block(&stmt.statements, Environment::new(previous_environment))?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Result<(), LangError> {
        self.environment
            .define(stmt.name.lexeme.clone(), Object::Value(LiteralType::Nil));
        let klass = LoxClass::new(stmt.name.lexeme.clone());
        self.environment.assign(stmt.name.clone(), klass)?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> Result<(), LangError> {
        self.evaluate(&Box::new(stmt.clone().expression))?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Result<(), LangError> {
        let identifier = stmt.clone().name.lexeme;
        let lox_function = LoxFunction::new(stmt.clone(), self.environment.clone());
        let callable_object = Object::Callable(lox_function);
        self.environment.define(identifier.clone(), callable_object);
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

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Result<(), LangError> {
        let value = if let Expr::Literal(literal) = stmt.clone().value {
            Ok(Object::Value(literal.value))
        } else {
            self.evaluate(&Box::new(stmt.clone().value))
        }?;

        Err(LangError::Return(value))
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

impl expr::Visitor<Result<Object, LangError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Object, LangError> {
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
        Ok(Object::Value(value))
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<Object, LangError> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = Vec::new();
        for arg in &expr.arguments {
            let evaluated_arg = self.evaluate(&Box::new(arg.clone()))?.fetch_value();
            arguments.push(evaluated_arg);
        }
        let function = if let Object::Callable(c) = callee {
            match c {
                CallableType::Function(id) => id,
            }
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
        let ret = function.call(self, arguments)?;
        Ok(ret)
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Object, LangError> {
        self.evaluate(&expr.expression)
    }

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Result<Object, LangError> {
        let left = self.evaluate(&expr.left)?.fetch_value();
        if let TokenType::Or = expr.operator.token_type {
            if literal_type::is_truthy(left.clone()) {
                return Ok(Object::Value(left));
            }
        } else {
            if !literal_type::is_truthy(left.clone()) {
                return Ok(Object::Value(left));
            }
        }

        self.evaluate(&expr.right)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Object, LangError> {
        let right = self.evaluate(&expr.right)?.fetch_value();
        let value = match expr.operator.token_type {
            TokenType::Minus => -right,
            TokenType::Bang => !right,
            _ => panic!("invalid Expression"),
        };
        if let LiteralType::Error(message) = value {
            return Err(LangError::RuntimeError(message, expr.clone().operator));
        }
        Ok(Object::Value(value))
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Object, LangError> {
        let value = Object::Value(expr.value.clone());
        Ok(value)
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<Object, LangError> {
        self.look_up_variable(expr.name.clone())
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<Object, LangError> {
        let value = self.evaluate(&expr.value)?;
        if let Some(distance) = self.locals.get(&expr.name.id) {
            self.environment
                .assign_at(*distance, expr.name.clone(), value.clone())?;
        } else {
            self.globals.assign(expr.clone().name, value.clone())?;
        }
        Ok(value)
    }
}

fn stringify_object(object: Object) -> String {
    match object {
        Object::Value(value) => value.to_string(),
        Object::Callable(callable) => callable.to_string(),
        Object::Class(class) => class.to_string(),
    }
}
