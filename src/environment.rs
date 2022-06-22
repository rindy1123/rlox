use std::collections::HashMap;

use crate::{
    lang_error::LangError,
    scanner::{literal_type::LiteralType, token::Token},
};

#[derive(Default, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralType>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            enclosing,
            ..Default::default()
        }
    }

    pub fn define(&mut self, name: String, value: LiteralType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralType, LangError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(environment) = &self.enclosing {
            return environment.get(name);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }

    pub fn assign(&mut self, name: Token, value: LiteralType) -> Result<(), LangError> {
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
}
