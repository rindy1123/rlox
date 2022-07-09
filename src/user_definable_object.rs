use crate::{interpreter::Interpreter, scanner::literal_type::LiteralType};

/// Values and Callable Objects that a user can define
#[derive(Clone)]
pub enum UserDefinableObject {
    Callable(Box<dyn LoxCallable>),
    Value(LiteralType),
}

impl UserDefinableObject {
    pub fn fetch_value(self) -> LiteralType {
        match self {
            UserDefinableObject::Value(v) => v,
            _ => panic!("Supposed to be Value"),
        }
    }
}

pub trait LoxCallable: LoxCallableClone {
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralType>) -> LiteralType;
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
