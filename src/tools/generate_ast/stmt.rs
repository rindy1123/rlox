use crate::utils;
use std::fs::File;
use std::io::prelude::*;

pub const BASE_NAME: &str = "Stmt";

pub fn define_ast(output_dir: &str) {
    let types = vec![
        "Block; statements: Vec<Stmt>".to_string(),
        "Expression; expression: Expr".to_string(),
        "Function; name: Token, params: Vec<Token>, body: Vec<Stmt>".to_string(),
        "If; condition: Expr, then_statement: Box<Stmt>, else_statement: Option<Box<Stmt>>"
            .to_string(),
        "Print; expression: Expr".to_string(),
        "Return; keyword: Token, value: Expr".to_string(),
        "Var; name: Token, initializer: Expr".to_string(),
        "While; condition: Expr, body: Box<Stmt>".to_string(),
    ];
    let path = format!("{}/{}.rs", output_dir, BASE_NAME.to_lowercase());
    let mut file = File::create(&path).unwrap();
    let content = define_dependency()
        + &utils::define_visitor(types.clone(), BASE_NAME.to_lowercase())
        + &utils::define_accept(BASE_NAME.to_string())
        + &utils::define_enum(types.clone(), BASE_NAME.to_string())
        + &utils::define_accept_for_enum(types.clone(), BASE_NAME.to_string())
        + &utils::define_structs(types, BASE_NAME.to_string());
    file.write_all(content.as_bytes()).unwrap()
}

fn define_dependency() -> String {
    "use crate::expr::Expr;
     use crate::scanner::token::Token;

    "
    .to_string()
}
