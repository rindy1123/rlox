use std::collections::HashMap;

use crate::{lang_error::LangError, object::Object, scanner::token::Token};

use super::callable::lox_class::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Object {
        Object::Instance(LoxInstance {
            class,
            fields: HashMap::new(),
        })
    }

    pub fn get(&self, name: Token) -> Result<Object, LangError> {
        let value = self.fields.get(&name.lexeme);
        match value {
            Some(v) => Ok(v.clone()),
            None => {
                let error_message = format!("Undefined property '{}'.", name.lexeme);
                Err(LangError::RuntimeError(error_message, name))
            }
        }
    }

    pub fn set(&mut self, name: Token, value: Object) {
        self.fields.insert(name.lexeme, value);
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }
}
