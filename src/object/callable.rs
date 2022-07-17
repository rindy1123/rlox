use std::fmt::Debug;

use crate::{interpreter::Interpreter, lang_error::LangError};

use super::{literal_type::LiteralType, Object};

#[derive(Debug, Clone)]
pub enum CallableType {
    Function(Box<dyn LoxCallable>),
}

impl CallableType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Function(function) => format!("fn <{:?}>", function).to_string(),
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
