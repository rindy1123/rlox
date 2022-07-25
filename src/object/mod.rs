pub mod callable;
pub mod literal_type;
pub mod lox_instance;

use std::{fmt::Debug, rc::Rc};

use self::{callable::CallableType, literal_type::LiteralType, lox_instance::LoxInstance};

/// Values and Callable Objects that a user can define
#[derive(Debug, Clone)]
pub enum Object {
    Callable(CallableType),
    Instance(Rc<LoxInstance>),
    Value(LiteralType),
}

impl Object {
    pub fn fetch_value(self) -> LiteralType {
        match self {
            Object::Value(v) => v,
            _ => panic!("Supposed to be Value"),
        }
    }
}
