use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    interpreter::Interpreter,
    lang_error::LangError,
    object::{literal_type::LiteralType, LoxCallable, Object},
};

#[derive(Clone, Debug)]
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

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let ret = Object::Value(LiteralType::Num(current_time.as_secs_f64()));
        Ok(ret)
    }

    fn to_string(&self) -> String {
        "native fn <Clock>".to_string()
    }
}
