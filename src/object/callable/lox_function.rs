use std::rc::Rc;

use crate::{
    environment::Environment,
    interpreter::Interpreter,
    lang_error::LangError,
    object::{literal_type::LiteralType, lox_instance::LoxInstance, LoxCallable, Object},
    scanner::token::{Token, TokenType},
    stmt::Function,
};

#[derive(Clone, Debug)]
pub struct LoxFunction {
    declaration: Function,
    closure: Rc<Environment>,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: Function,
        closure: Rc<Environment>,
        is_initializer: bool,
    ) -> LoxFunction {
        LoxFunction {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn bind(self, instance: Rc<LoxInstance>) -> Self {
        let environment = Environment::new(Some(self.closure));
        let instance = Object::Instance(instance);
        environment.define("this".to_string(), instance);
        Self::new(self.declaration, environment, self.is_initializer)
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralType>,
    ) -> Result<Object, LangError> {
        let new_environment = Environment::new(Some(self.closure.clone()));
        for n in 0..self.declaration.params.len() {
            new_environment.define(
                self.declaration.params[n].clone().lexeme,
                Object::Value(arguments[n].clone()),
            );
        }

        if let Err(lang_error) = interpreter.execute_block(&self.declaration.body, new_environment)
        {
            match lang_error {
                LangError::Return(ret) => {
                    if self.is_initializer {
                        let this_token = generate_this_token(self.declaration.clone());
                        return self.closure.get_at(0, this_token);
                    }
                    return Ok(ret);
                }
                _ => {
                    return Err(lang_error);
                }
            }
        }

        if self.is_initializer {
            let this_token = generate_this_token(self.declaration.clone());
            return self.closure.get_at(0, this_token);
        }
        Ok(Object::Value(LiteralType::Nil))
    }

    fn to_string(&self) -> String {
        format!("fn <{:?}>", self.declaration.name.lexeme)
    }
}

fn generate_this_token(declaration: Function) -> Token {
    let line = declaration.name.line;
    let id = declaration.name.id;
    Token::new(TokenType::This, "this".to_string(), None, line, id)
}
