use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{lang_error::LangError, object::Object, scanner::token::Token};

use super::callable::lox_class::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: RefCell<HashMap<String, Object>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(self: Rc<Self>, name: Token) -> Result<Object, LangError> {
        if let Some(v) = self.fields.borrow().get(&name.lexeme) {
            return Ok(v.clone());
        }
        if let Some(method) = self.class.find_method(name.lexeme.clone()) {
            let lox_function = Box::new(method.clone().bind(self));
            let object = Object::Function(lox_function);
            return Ok(object);
        }
        let message = format!("Undefined property '{}'.", name.lexeme);
        Err(LangError::RuntimeError {
            message,
            line: name.line,
        })
    }

    pub fn set(&self, name: Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme, value);
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }
}
