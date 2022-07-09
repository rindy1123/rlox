use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    interpreter::Interpreter, scanner::literal_type::LiteralType,
    user_definable_object::LoxCallable,
};

#[derive(Clone)]
pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {
        Clock {}
    }
}

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<LiteralType>) -> LiteralType {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        LiteralType::Num(current_time.as_secs_f64())
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }
}
