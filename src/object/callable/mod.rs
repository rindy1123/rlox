pub mod global_function;
pub mod lox_class;
pub mod lox_function;

use std::fmt::Debug;

use crate::{interpreter::Interpreter, lang_error::LangError};

use super::{literal_type::LiteralType, Object};

#[derive(Debug, Clone)]
pub enum CallableType {
    Function(Box<dyn LoxCallable>),
    Class(Box<dyn LoxCallable>),
}

impl CallableType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Function(function) => function.to_string(),
            Self::Class(class) => class.to_string(),
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
    use crate::object::callable::global_function::Clock;

    #[test]
    fn test_to_string() {
        let callable = Clock::new();
        assert_eq!(callable.to_string(), "fn <LoxCallable>")
    }
}
