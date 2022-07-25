use crate::utils;
use std::fs::File;
use std::io::prelude::*;

pub const BASE_NAME: &str = "Expr";

pub fn define_ast(output_dir: &str) {
    let types = vec![
        "Assign; name: Token, value: Box<Expr>".to_string(),
        "Binary; left: Box<Expr>, operator: Token, right: Box<Expr>".to_string(),
        "Call; callee: Box<Expr>, paren: Token, arguments: Vec<Expr>".to_string(),
        "Get; object: Box<Expr>, name: Token".to_string(),
        "Grouping; expression: Box<Expr>".to_string(),
        "Literal; value: LiteralType".to_string(),
        "Logical; left: Box<Expr>, operator: Token, right: Box<Expr>".to_string(),
        "Set; object: Box<Expr>, name: Token, value: Box<Expr>".to_string(),
        "Unary; operator: Token, right: Box<Expr>".to_string(),
        "Variable; name: Token".to_string(),
    ];
    let path = format!("{}/{}.rs", output_dir, BASE_NAME.to_lowercase());
    let mut file = File::create(&path).unwrap();
    let content = define_dependency()
        + &utils::define_visitor(types.clone(), BASE_NAME.to_lowercase())
        + &utils::define_accept(BASE_NAME.to_string())
        + &utils::define_enum(types.clone(), BASE_NAME.to_string())
        + &utils::define_accept_for_enum(types.clone(), BASE_NAME.to_string())
        + &utils::define_structs(types, BASE_NAME.to_string());
    file.write_all(content.as_bytes()).unwrap();
}

fn define_dependency() -> String {
    "use crate::object::literal_type::LiteralType;
     use crate::scanner::token::Token;

    "
    .to_string()
}
