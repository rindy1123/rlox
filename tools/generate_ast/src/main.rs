use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: generate_ast <output directory>");
        exit(64)
    }
    let output_dir = &args[1];
    let types = vec![
        "Binary; left: Box<Expr>, operator: Token, right: Box<Expr>",
        "Grouping; expression: Box<Expr>",
        "Literal; value: LiteralType",
        "Unary; operator: Token, right: Box<Expr>",
    ];
    define_ast(output_dir, "expr", types);
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) {
    fs::create_dir_all(output_dir).unwrap();
    let path = format!("{}/{}.rs", output_dir, base_name);
    let mut file = File::create(&path).unwrap();
    let mut content = String::new();
    content.push_str("use crate::scanner::token::{LiteralType, Token};\n");
    define_visitor(&mut content, &types);
    define_accept(&mut content);
    define_expr(&mut content, &types);
    define_accept_for_expr(&mut content, &types);
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let fields = struct_name_and_fields[1].trim();
        define_struct(&mut content, &struct_name, fields);
        define_impl_for_each_expr(&mut content, &struct_name, fields);
    }
    file.write_all(content.as_bytes());
}

fn define_struct(content: &mut String, struct_name: &str, fields: &str) {
    content.push_str(&format!("pub struct {} {{\n", struct_name));
    for field in split_fields(fields) {
        let field = field.trim();
        content.push_str(&format!("    pub {},\n", field));
    }
    content.push_str("}\n\n");
}

fn define_impl_for_each_expr(content: &mut String, struct_name: &str, fields: &str) {
    content.push_str(&format!("impl {} {{\n", struct_name));
    // start of new function
    content.push_str(&format!(
        "    pub fn new({}) -> {} {{\n",
        fields, struct_name
    ));
    content.push_str(&format!("        {} {{\n", struct_name));
    for field in split_fields(fields) {
        let argument = field.split(':').collect::<Vec<&str>>()[0].trim();
        content.push_str(&format!("            {},\n", argument));
    }
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");
    // end of new function
    content.push_str(&format!("impl<T> Accept<T> for {} {{\n", struct_name));
    // start of accept function
    content.push_str("    fn accept(&self, visitor: &impl Visitor<T>) -> T {\n");
    content.push_str(&format!(
        "        visitor.visit_{}_expr(self)\n",
        struct_name.to_lowercase()
    ));
    content.push_str("    }\n");
    // end of accept function
    content.push_str("}\n\n");
}

fn define_visitor(content: &mut String, types: &Vec<&str>) {
    content.push_str("pub trait Visitor<T> {\n");
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let fields = struct_name_and_fields[1].trim();
        content.push_str(&format!(
            "    fn visit_{}_expr(&self, expr: &{}) -> T;\n",
            struct_name.to_lowercase(),
            struct_name
        ));
    }
    content.push_str("}\n\n");
}

fn define_accept(content: &mut String) {
    content.push_str("pub trait Accept<T> {\n");
    content.push_str("    fn accept(&self, visitor: &impl Visitor<T>) -> T;\n");
    content.push_str("}\n\n");
}

fn define_expr(content: &mut String, types: &Vec<&str>) {
    content.push_str("pub enum Expr {\n");
    for type_string in types {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        content.push_str(&format!("    {}(Box<{}>),\n", struct_name, struct_name));
    }
    content.push_str("}\n\n");
}

fn define_accept_for_expr(content: &mut String, types: &Vec<&str>) {
    content.push_str("impl<T> Accept<T> for Expr {\n");
    content.push_str("    fn accept(&self, visitor: &impl Visitor<T>) -> T {\n");
    content.push_str("        match self {\n");
    for type_string in types {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        content.push_str(&format!(
            "            Expr::{}(e) => e.accept(visitor),\n",
            struct_name
        ));
    }
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");
}

fn split_fields(fields: &str) -> Vec<&str> {
    fields.split(',').collect()
}
