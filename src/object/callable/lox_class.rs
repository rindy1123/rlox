use std::collections::HashMap;

use crate::{
    interpreter::Interpreter,
    lang_error::LangError,
    object::{literal_type::LiteralType, lox_instance::LoxInstance, Object},
};

use super::{CallableType, LoxCallable};

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, Object>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Object>) -> Object {
        let lox_class = Box::new(LoxClass { name, methods });
        Object::Callable(CallableType::Class(lox_class))
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError> {
        Ok(LoxInstance::new(self.clone()))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        self.name.clone()
    }
}
