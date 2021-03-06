use crate::{
    object::Object,
    scanner::token::{Token, TokenType},
};

#[derive(Clone)]
pub enum LangError {
    ParseError,
    ResolveError,
    RuntimeError { message: String, line: u32 },
    Return(Object),
}

pub fn error(line_num: u32, message: String) {
    report(line_num, String::from(""), message);
}

fn report(line_num: u32, location: String, message: String) {
    println!("[line: {}] Error{}: {}", line_num, location, message)
}

pub fn parser_error(token: &Token, message: String) {
    if token.token_type == TokenType::EOF {
        report(token.line, " at end".to_string(), message);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), message);
    }
}
