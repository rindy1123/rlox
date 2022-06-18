use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: generate_ast <output directory>");
        exit(64)
    }
    let output_dir = if args.len() == 1 {
        "../../src"
    } else {
        &args[1]
    };
    let types_for_expr = vec![
        "Binary; left: Box<Expr>, operator: Token, right: Box<Expr>",
        "Grouping; expression: Box<Expr>",
        "Literal; value: LiteralType",
        "Unary; operator: Token, right: Box<Expr>",
    ];
    let types_for_stmt = vec!["Expression; expression: Expr", "Print; expression: Expr"];
    define_ast(output_dir, "Expr", types_for_expr);
    define_ast(output_dir, "Stmt", types_for_stmt);
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) {
    fs::create_dir_all(output_dir).unwrap();
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(&path).unwrap();
    let mut content = String::new();
    define_dependency(&mut content, base_name);
    define_visitor(&mut content, &types, &base_name.to_lowercase());
    define_accept(&mut content);
    define_enum(&mut content, &types, base_name);
    define_accept_for_enum(&mut content, &types, base_name);
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let fields = struct_name_and_fields[1].trim();
        define_struct(&mut content, &struct_name, fields);
        define_impl_for_each_expr(&mut content, &struct_name, fields, base_name);
    }
    file.write_all(content.as_bytes());
}

fn define_dependency(content: &mut String, base_name: &str) {
    match base_name {
        "Expr" => {
            content.push_str("use crate::scanner::literal_type::LiteralType;\n");
            content.push_str("use crate::scanner::token::Token;\n");
            content.push_str("\n");
        }
        "Stmt" => {
            content.push_str("use crate::expr::Expr;\n");
            content.push_str("\n");
        }
        _ => panic!("base_name does not exist."),
    }
}

fn define_visitor(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str("pub trait Visitor<T> {\n");
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let fields = struct_name_and_fields[1].trim();
        content.push_str(&format!(
            "    fn visit_{}_{}(&self, {}: &{}) -> T;\n",
            struct_name.to_lowercase(),
            base_name,
            base_name,
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

fn define_enum(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str("#[derive(Clone)]\n");
    content.push_str(&format!("pub enum {} {{\n", base_name));
    for type_string in types {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        match base_name {
            "Expr" => content.push_str(&format!("    {}(Box<{}>),\n", struct_name, struct_name)),
            _ => content.push_str(&format!("    {}({}),\n", struct_name, struct_name)),
        }
    }
    content.push_str("}\n\n");
}

fn define_accept_for_enum(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str(&format!("impl<T> Accept<T> for {} {{\n", base_name));
    content.push_str("    fn accept(&self, visitor: &impl Visitor<T>) -> T {\n");
    content.push_str("        match self {\n");
    for type_string in types {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        content.push_str(&format!(
            "            {}::{}(e) => e.accept(visitor),\n",
            base_name, struct_name
        ));
    }
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");
}

fn define_struct(content: &mut String, struct_name: &str, fields: &str) {
    content.push_str("#[derive(Clone)]\n");
    content.push_str(&format!("pub struct {} {{\n", struct_name));
    for field in split_fields(fields) {
        let field = field.trim();
        content.push_str(&format!("    pub {},\n", field));
    }
    content.push_str("}\n\n");
}

fn define_impl_for_each_expr(
    content: &mut String,
    struct_name: &str,
    fields: &str,
    base_name: &str,
) {
    content.push_str(&format!("impl {} {{\n", struct_name));
    // start of new function
    define_new_function(content, struct_name, fields, base_name);
    content.push_str("}\n");
    content.push_str("\n");
    // end of new function
    content.push_str(&format!("impl<T> Accept<T> for {} {{\n", struct_name));
    // start of accept function
    content.push_str("    fn accept(&self, visitor: &impl Visitor<T>) -> T {\n");
    content.push_str(&format!(
        "        visitor.visit_{}_{}(self)\n",
        struct_name.to_lowercase(),
        base_name.to_lowercase()
    ));
    content.push_str("    }\n");
    // end of accept function
    content.push_str("}\n\n");
}

fn define_new_function(content: &mut String, struct_name: &str, fields: &str, base_name: &str) {
    match base_name {
        "Expr" => {
            content.push_str(&format!(
                "    pub fn new({}) -> Box<{}> {{\n",
                fields, struct_name
            ));
            content.push_str("        Box::new(\n");
            content.push_str(&format!("            {} {{\n", struct_name));
            for field in split_fields(fields) {
                let argument = field.split(':').collect::<Vec<&str>>()[0].trim();
                content.push_str(&format!("                {},\n", argument));
            }
            content.push_str("            }\n");
            content.push_str("        )\n");
            content.push_str("    }\n");
        }
        _ => {
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
        }
    }
}

fn split_fields(fields: &str) -> Vec<&str> {
    fields.split(',').collect()
}
