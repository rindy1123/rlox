use std::collections::HashMap;

use crate::{
    lang_error::LangError,
    scanner::{literal_type::LiteralType, token::Token},
};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, LiteralType>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralType, LangError> {
        let value = self.values.get(&name.lexeme);
        match value {
            Some(value) => Ok(value.clone()),
            None => {
                let message = format!("Undefined variable '{}'.", name.lexeme);
                Err(LangError::RuntimeError(message, name.clone()))
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: LiteralType) -> Result<(), LangError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        Err(LangError::RuntimeError(message, name.clone()))
    }
}
