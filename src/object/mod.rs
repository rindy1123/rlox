pub mod callable;
pub mod literal_type;
pub mod lox_instance;

use std::{fmt::Debug, rc::Rc};

use crate::{interpreter::Interpreter, lang_error::LangError};

use self::{callable::lox_class::LoxClass, literal_type::LiteralType, lox_instance::LoxInstance};

/// Values and Callable Objects that a user can define
#[derive(Debug, Clone)]
pub enum Object {
    Function(Box<dyn LoxCallable>),
    Class(LoxClass),
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

pub trait LoxCallable: LoxCallableClone {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError>;
    fn to_string(&self) -> String;
}

pub trait LoxCallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> LoxCallableClone for T
where
    T: 'static + LoxCallable + Clone,
{
    fn clone_box(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Box<dyn LoxCallable> {
        self.clone_box()
    }
}

impl Debug for Box<dyn LoxCallable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoxCallable")
    }
}

#[cfg(test)]
mod tests {
    use super::LoxCallable;
    use crate::object::callable::global_function::Clock;

    #[test]
    fn test_to_string() {
        let clock_function = Clock::new();
        assert_eq!(clock_function.to_string(), "native fn <Clock>")
    }
}
