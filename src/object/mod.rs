pub mod literal_type;
pub mod lox_function;

use crate::{interpreter::Interpreter, lang_error::LangError};

use self::literal_type::LiteralType;

/// Values and Callable Objects that a user can define
#[derive(Clone)]
pub enum Object {
    Callable(Box<dyn LoxCallable>),
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
    fn to_string(&self) -> String;
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
