use std::collections::HashMap;

use crate::{
    expr::{self, Accept as AcceptExpr, Expr},
    interpreter::Interpreter,
    lang_error,
    scanner::token::Token,
    stmt::{self, Accept as AcceptStmt, Stmt},
};

pub struct Resolver {
    pub interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve_statements(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.resolve_statement(statement)
        }
    }

    fn resolve_statement(&mut self, stmt: Stmt) {
        stmt.accept(self)
    }

    fn resolve_expression(&mut self, mut expr: Expr) {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme, false);
        self.scopes.push(scope);
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

    fn resolve_function(&mut self, function: stmt::Function) {
        self.begin_scope();
        for param in function.params {
            self.declare(param.clone());
            self.define(param.clone());
        }
        self.resolve_statements(function.body);
        self.end_scope();
    }
}

impl stmt::Visitor<()> for Resolver {
    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> () {
        self.begin_scope();
        self.resolve_statements(stmt.clone().statements);
        self.end_scope();
    }

    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> () {
        self.resolve_expression(stmt.clone().expression);
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> () {
        let cloned_stmt = stmt.clone();
        self.declare(cloned_stmt.name.clone());
        self.define(cloned_stmt.name.clone());

        self.resolve_function(cloned_stmt);
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> () {
        let cloned_stmt = stmt.clone();
        self.resolve_expression(cloned_stmt.condition);
        self.resolve_statement(*cloned_stmt.then_statement);
        if stmt.else_statement.is_none() {
            return;
        }
        self.resolve_statement(*stmt.clone().else_statement.unwrap());
    }

    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> () {
        self.resolve_expression(stmt.clone().expression);
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> () {
        self.resolve_expression(stmt.clone().value);
    }

    fn visit_var_stmt(&mut self, stmt: &stmt::Var) {
        let cloned_stmt = stmt.clone();
        self.declare(cloned_stmt.name.clone());
        self.resolve_expression(cloned_stmt.initializer);
        self.define(cloned_stmt.name);
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> () {
        let cloned_stmt = stmt.clone();
        self.resolve_expression(cloned_stmt.condition);
        self.resolve_statement(*cloned_stmt.body);
    }
}

impl expr::Visitor<()> for Resolver {
    fn visit_assign_expr(&mut self, expr: &expr::Assign) {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.value);
        self.resolve_local_variable(cloned_expr.name);
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> () {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.left);
        self.resolve_expression(*cloned_expr.right);
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> () {
        let cloned_expr = expr.clone();
        self.resolve_expression(*cloned_expr.callee);
        for arg in cloned_expr.arguments {
            self.resolve_expression(arg);
        }
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> () {
        self.resolve_expression(*expr.clone().expression);
    }

    fn visit_literal_expr(&mut self, _expr: &expr::Literal) -> () {}

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> () {
        self.resolve_expression(*expr.clone().left);
        self.resolve_expression(*expr.clone().right);
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> () {
        self.resolve_expression(*expr.clone().right);
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) {
        if !self.scopes.is_empty() {
            let variable = self.scopes.last().unwrap().get(&expr.name.lexeme);
            if variable.is_some() && !variable.unwrap() {
                lang_error::error(
                    expr.name.line,
                    "Can't read local variable in its own initializer.".to_string(),
                );
            }
        }

        self.resolve_local_variable(expr.clone().name);
    }
}
