use std::rc::Rc;

use crate::{
    environment::Environment, interpreter::Interpreter, lang_error::LangError,
    object::literal_type::LiteralType, stmt::Function,
};

use super::{CallableType, LoxCallable, Object};

#[derive(Clone)]
pub struct LoxFunction {
    declaration: Function,
    closure: Rc<Environment>,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: Rc<Environment>) -> CallableType {
        CallableType::Function(Box::new(LoxFunction {
            declaration,
            closure,
        }))
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError> {
        let new_environment = Environment::new(Some(self.closure.clone()));
        for n in 0..self.declaration.params.len() {
            new_environment.define(
                self.declaration.params[n].clone().lexeme,
                Object::Value(arguments[n].clone()),
            );
        }

        if let Err(lang_error) = interpreter.execute_block(&self.declaration.body, new_environment)
        {
            if let LangError::Return(ret) = lang_error {
                return Ok(ret);
            } else {
                return Err(lang_error);
            };
        }

        Ok(Object::Value(LiteralType::Nil))
    }

    fn to_string(&self) -> String {
        format!("fn <{:?}>", self.declaration.name.lexeme)
    }
}
