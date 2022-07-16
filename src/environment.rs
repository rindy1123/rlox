use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{lang_error::LangError, object::Object, scanner::token::Token};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<Environment>>,
    pub values: RefCell<HashMap<String, Object>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<Environment>>) -> Rc<Environment> {
        Rc::new(Environment {
            enclosing,
            ..Default::default()
        })
    }

    pub fn define(&self, name: String, value: Object) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LangError> {
        if let Some(value) = self.values.borrow().get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(environment) = &self.enclosing {
            return environment.get(name);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }

    pub fn assign(&self, name: Token, value: Object) -> Result<(), LangError> {
        let mut values = self.values.borrow_mut();
        if values.contains_key(&name.lexeme) {
            values.insert(name.lexeme, value);
            return Ok(());
        }

        if let Some(environment) = self.enclosing.clone() {
            return environment.assign(name, value);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }
}
