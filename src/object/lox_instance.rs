use crate::object::Object;

use super::callable::lox_class::LoxClass;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Object {
        Object::Instance(LoxInstance { class })
    }

    pub fn to_string(&self) -> String {
        format!("{} instance", self.class.name)
    }
}
