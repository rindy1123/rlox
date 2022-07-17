pub mod callable;
pub mod literal_type;

use std::fmt::Debug;

use self::{callable::CallableType, literal_type::LiteralType};

/// Values and Callable Objects that a user can define
#[derive(Debug, Clone)]
pub enum Object {
    Callable(CallableType),
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
