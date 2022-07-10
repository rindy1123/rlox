use std::collections::HashMap;

use crate::{lang_error::LangError, object::Object, scanner::token::Token};

#[derive(Default, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            enclosing,
            ..Default::default()
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LangError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(environment) = &self.enclosing {
            return environment.get(name);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<(), LangError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        if let Some(environment) = &mut self.enclosing {
            return environment.assign(name, value);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }

    pub fn get_parent_environment(&self) -> Option<Self> {
        match self.clone().enclosing {
            Some(env) => Some(*env),
            _ => None,
        }
    }
}
