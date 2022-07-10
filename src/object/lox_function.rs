use crate::{
    environment::Environment, interpreter::Interpreter, lang_error::LangError, stmt::Function,
};

use super::{literal_type::LiteralType, LoxCallable, Object};

#[derive(Clone)]
pub struct LoxFunction {
    declaration: Function,
}

impl LoxFunction {
    pub fn new(declaration: Function) -> LoxFunction {
        LoxFunction { declaration }
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
        let environment = Some(Box::new(interpreter.environment.clone()));
        let mut new_environment = Environment::new(environment);
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
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
