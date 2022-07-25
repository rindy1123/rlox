use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{lang_error::LangError, object::Object, scanner::token::Token};

use super::callable::lox_class::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: RefCell<HashMap<String, Object>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Object {
        let lox_instance = LoxInstance {
            class,
            fields: RefCell::new(HashMap::new()),
        };
        Object::Instance(Rc::new(lox_instance))
    }

    pub fn get(&self, name: Token) -> Result<Object, LangError> {
        if let Some(v) = self.fields.borrow().get(&name.lexeme) {
            return Ok(v.clone());
        }
        if let Some(method) = self.class.methods.get(&name.lexeme) {
            return Ok(method.clone());
        }
        let error_message = format!("Undefined property '{}'.", name.lexeme);
        Err(LangError::RuntimeError(error_message, name))
    }

    pub fn set(&self, name: Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme, value);
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }
}
