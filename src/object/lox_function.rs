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
        let global_environment = Some(Box::new(interpreter.globals.clone()));
        let mut environment = Environment::new(global_environment);
        for n in 0..self.declaration.params.len() {
            environment.define(
                self.declaration.params[n].clone().lexeme,
                Object::Value(arguments[n].clone()),
            );
        }

        interpreter.execute_block(&self.declaration.body, environment)?;
        Ok(Object::Value(LiteralType::Nil))
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
