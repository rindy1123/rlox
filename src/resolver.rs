use std::collections::HashMap;

use crate::{
    expr::{self, Accept as AcceptExpr, Expr},
    interpreter::Interpreter,
    lang_error::{self, LangError},
    scanner::token::Token,
    stmt::{self, Accept as AcceptStmt, Stmt},
};

pub struct Resolver {
    pub interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

#[derive(Clone, Debug)]
enum FunctionType {
    None,
    Function,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_statements(&mut self, statements: Vec<Stmt>) -> Result<(), LangError> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: Stmt) -> Result<(), LangError> {
        stmt.accept(self)
    }

    fn resolve_expression(&mut self, mut expr: Expr) -> Result<(), LangError> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) -> Result<(), LangError> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        let mut scope = self.scopes.pop().unwrap();
        let result = if scope.contains_key(&name.lexeme) {
            report_error(
                name.line,
                "Already a variable with this name in this scope.".to_string(),
            )
        } else {
            Ok(())
        };
        scope.insert(name.lexeme, false);
        self.scopes.push(scope);
        result
    }

    fn define(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme, true);
        self.scopes.push(scope);
    }

    fn resolve_local_variable(&mut self, name: Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name.id, i);
            }
        }
    }

    fn resolve_function(
        &mut self,
        function: stmt::Function,
        function_type: FunctionType,
    ) -> Result<(), LangError> {
        let enclosing_function = self.current_function.clone();
        self.current_function = function_type;
        self.begin_scope();
        for param in function.params {
            self.declare(param.clone())?;
            self.define(param.clone());
        }
        self.resolve_statements(function.body)?;
        self.end_scope();

        self.current_function = enclosing_function;
        Ok(())
    }
}

impl stmt::Visitor<Result<(), LangError>> for Resolver {
    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Result<(), LangError> {
        self.begin_scope();
        self.resolve_statements(stmt.clone().statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Result<(), LangError> {
        self.declare(stmt.name.clone())?;
        self.define(stmt.name.clone());
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> Result<(), LangError> {
        self.resolve_expression(stmt.clone().expression)
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        self.declare(cloned_stmt.name.clone())?;
        self.define(cloned_stmt.name.clone());

        self.resolve_function(cloned_stmt, FunctionType::Function)
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        self.resolve_expression(cloned_stmt.condition)?;
        self.resolve_statement(*cloned_stmt.then_statement)?;
        if stmt.else_statement.is_none() {
            return Ok(());
        }
        self.resolve_statement(*stmt.clone().else_statement.unwrap())
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Result<(), LangError> {
        self.resolve_expression(stmt.clone().expression)
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Result<(), LangError> {
        let result = if let FunctionType::None = self.current_function {
            report_error(
                stmt.keyword.line,
                "Can't return from top-level code.".to_string(),
            )
        } else {
            Ok(())
        };
        self.resolve_expression(stmt.clone().value)?;
        result
    }

    fn visit_var_stmt(&mut self, stmt: &stmt::Var) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        self.declare(cloned_stmt.name.clone())?;
        self.resolve_expression(cloned_stmt.initializer)?;
        self.define(cloned_stmt.name);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Result<(), LangError> {
        let cloned_stmt = stmt.clone();
        self.resolve_expression(cloned_stmt.condition)?;
        self.resolve_statement(*cloned_stmt.body)
    }
}

impl expr::Visitor<Result<(), LangError>> for Resolver {
    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<(), LangError> {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.value)?;
        self.resolve_local_variable(cloned_expr.name);
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Result<(), LangError> {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.left)?;
        self.resolve_expression(*cloned_expr.right)
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<(), LangError> {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.callee)?;
        for arg in cloned_expr.arguments {
            self.resolve_expression(arg)?;
        }
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Result<(), LangError> {
        self.resolve_expression(*expr.clone().expression)
    }

    fn visit_literal_expr(&mut self, _expr: &expr::Literal) -> Result<(), LangError> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Result<(), LangError> {
        self.resolve_expression(*expr.clone().left)?;
        self.resolve_expression(*expr.clone().right)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Result<(), LangError> {
        self.resolve_expression(*expr.clone().right)
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<(), LangError> {
        let result = if !self.scopes.is_empty() {
            let variable = self.scopes.last().unwrap().get(&expr.name.lexeme);
            if variable.is_some() && !variable.unwrap() {
                return report_error(
                    expr.name.line,
                    "Can't read local variable in its own initializer.".to_string(),
                );
            }
            Ok(())
        } else {
            Ok(())
        };

        self.resolve_local_variable(expr.clone().name);
        result
    }
}

fn report_error(line: u32, message: String) -> Result<(), LangError> {
    lang_error::error(line, message);
    Err(LangError::ParseError)
}
