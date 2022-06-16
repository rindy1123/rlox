use crate::scanner::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum LangError {
    ParseError,
    RuntimeError(String, Token),
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
