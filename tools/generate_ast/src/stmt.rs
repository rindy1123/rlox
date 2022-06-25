use crate::utils;
use std::fs::File;
use std::io::prelude::*;

pub fn define_ast(output_dir: &str) {
    let base_name = "Stmt";
    let types = vec![
        "Block; statements: Vec<Stmt>",
        "Expression; expression: Expr",
        "If; condition: Expr, then_statement: Box<Stmt>, else_statement: Option<Box<Stmt>>",
        "Print; expression: Expr",
        "Var; name: Token, initializer: Expr",
    ];
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(&path).unwrap();
    let mut content = String::new();
    define_dependency(&mut content);
    define_visitor(&mut content, &types, &base_name.to_lowercase());
    define_accept(&mut content);
    utils::define_enum(&mut content, &types, base_name);
    define_accept_for_enum(&mut content, &types, base_name);
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let fields = struct_name_and_fields[1].trim();
        utils::define_struct(&mut content, &struct_name, fields);
        define_impl_for_each_stmt(&mut content, &struct_name, fields, base_name);
    }
    file.write_all(content.as_bytes());
}

fn define_dependency(content: &mut String) {
    content.push_str(
        "use crate::expr::Expr;
         use crate::scanner::token::Token;

        ",
    );
}

fn define_visitor(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str("pub trait Visitor<T> {\n");
    for type_string in types {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        content.push_str(&format!(
            "    fn visit_{}_{}(&mut self, {}: &{}) -> T;\n",
            struct_name.to_lowercase(),
            base_name,
            base_name,
            struct_name
        ));
    }
    content.push_str("}\n\n");
}

fn define_accept(content: &mut String) {
    content.push_str(
        "pub trait Accept<T> {
            fn accept(&self, visitor: &mut impl Visitor<T>) -> T;
         }

        ",
    );
}

fn define_accept_for_enum(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str(&format!("impl<T> Accept<T> for {} {{\n", base_name));
    content.push_str("    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {\n");
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

fn define_impl_for_each_stmt(
    content: &mut String,
    struct_name: &str,
    fields: &str,
    base_name: &str,
) {
    content.push_str(&format!("impl {} {{\n", struct_name));
    // start of new function
    utils::define_new_function(content, struct_name, fields);
    content.push_str("}\n");
    content.push_str("\n");
    // end of new function
    content.push_str(&format!("impl<T> Accept<T> for {} {{\n", struct_name));
    // start of accept function
    content.push_str("    fn accept(&self, visitor: &mut impl Visitor<T>) -> T {\n");
    content.push_str(&format!(
        "        visitor.visit_{}_{}(self)\n",
        struct_name.to_lowercase(),
        base_name.to_lowercase()
    ));
    content.push_str("    }\n");
    // end of accept function
    content.push_str("}\n\n");
}
