use std::{cell::RefCell, collections::HashMap};

use crate::{
    expr::{self, Accept as AcceptExpr, Expr},
    interpreter::Interpreter,
    lang_error::{self, LangError},
    scanner::token::Token,
    stmt::{self, Accept as AcceptStmt, Stmt},
};

type Scopes = Vec<RefCell<HashMap<String, bool>>>;

impl ScopesOps<String, bool> for Scopes {
    fn insert_to_last(&mut self, key: String, value: bool) -> Option<bool> {
        self.last().unwrap().borrow_mut().insert(key, value)
    }
}

trait ScopesOps<K, V> {
    /// insert a key-value pair into the last scope
    /// # Examples
    /// ```
    /// scopes.insert_to_last("key", 1);
    /// ```
    fn insert_to_last(&mut self, key: K, value: V) -> Option<V>;
}

pub struct Resolver {
    pub interpreter: Interpreter,
    scopes: Scopes,
    current_function: FunctionType,
    current_class: ClassType,
}

#[derive(Clone, Debug)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, Debug)]
enum ClassType {
    None,
    Class,
    SubClass,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
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
        let scope = RefCell::new(HashMap::new());
        self.scopes.push(scope);
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) -> Result<(), LangError> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        let scope = self.scopes.pop().unwrap();
        let result = if scope.borrow().contains_key(&name.lexeme) {
            report_error(
                name.line,
                "Already a variable with this name in this scope.".to_string(),
            )
        } else {
            Ok(())
        };
        scope.borrow_mut().insert(name.lexeme, false);
        self.scopes.push(scope);
        result
    }

    fn define(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.pop().unwrap();
        scope.borrow_mut().insert(name.lexeme, true);
        self.scopes.push(scope);
    }

    fn resolve_local_variable(&mut self, name: Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.borrow().contains_key(&name.lexeme) {
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
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;
        self.declare(stmt.name.clone())?;
        self.define(stmt.name.clone());

        if let Some(superclass) = stmt.superclass.clone() {
            if stmt.name.lexeme == superclass.name.lexeme {
                report_error(
                    superclass.name.line,
                    "A class can't inherit from itself.".to_string(),
                )?;
            }
            self.current_class = ClassType::SubClass;
            self.resolve_expression(Expr::Variable(superclass))?;
            self.begin_scope();
            self.scopes.insert_to_last("super".to_string(), true);
        }
        self.begin_scope();
        self.scopes.insert_to_last("this".to_string(), true);
        for method in stmt.methods.iter() {
            let declaration = if method.name.lexeme == "init".to_string() {
                FunctionType::Initializer
            } else {
                FunctionType::Method
            };
            self.resolve_function(method.clone(), declaration)?;
        }
        self.end_scope();
        if let Some(_) = stmt.superclass.clone() {
            self.end_scope();
        }

        self.current_class = enclosing_class;
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
        let result = match self.current_function {
            FunctionType::None => report_error(
                stmt.keyword.line,
                "Can't return from top-level code.".to_string(),
            ),
            FunctionType::Initializer => report_error(
                stmt.keyword.line,
                "Can't return a value from an initializer.".to_string(),
            ),
            _ => Ok(()),
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

    fn visit_get_expr(&mut self, expr: &expr::Get) -> Result<(), LangError> {
        self.resolve_expression(*expr.object.clone())
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

    fn visit_set_expr(&mut self, expr: &expr::Set) -> Result<(), LangError> {
        self.resolve_expression(*expr.clone().value)?;
        self.resolve_expression(*expr.clone().object)
    }

    fn visit_super_expr(&mut self, expr: &expr::Super) -> Result<(), LangError> {
        if let ClassType::None = self.current_class {
            return report_error(
                expr.keyword.line,
                "Can't use 'super' outside of a class.".to_string(),
            );
        }
        if let ClassType::Class = self.current_class {
            return report_error(
                expr.keyword.line,
                "Can't use 'super' in a class with no superclass.".to_string(),
            );
        }
        self.resolve_local_variable(expr.clone().keyword);
        Ok(())
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> Result<(), LangError> {
        if let ClassType::None = self.current_class {
            return report_error(
                expr.keyword.line,
                "Can't use 'this' outside of a class.".to_string(),
            );
        }
        self.resolve_local_variable(expr.keyword.clone());
        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Result<(), LangError> {
        self.resolve_expression(*expr.clone().right)
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<(), LangError> {
        let result = if !self.scopes.is_empty() {
            let scope = self.scopes.last().unwrap().borrow();
            let variable = scope.get(&expr.name.lexeme);
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
