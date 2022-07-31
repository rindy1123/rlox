use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::Interpreter,
    lang_error::LangError,
    object::{literal_type::LiteralType, lox_instance::LoxInstance, LoxCallable, Object},
};

use super::lox_function::LoxFunction;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Box<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Box<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Object {
        let lox_class = LoxClass {
            name,
            superclass,
            methods,
        };
        Object::Class(lox_class)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError> {
        let instance = Rc::new(LoxInstance::new(self.clone()));
        let initializer = self.methods.get("init");
        if let Some(init_method) = initializer {
            init_method
                .clone()
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }
        Ok(Object::Instance(instance))
    }

    fn arity(&self) -> usize {
        let initializer = self.methods.get("init");
        if let Some(init_method) = initializer {
            return init_method.arity();
        }
        0
    }

    fn to_string(&self) -> String {
        self.name.clone()
    }
}
