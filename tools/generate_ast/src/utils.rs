pub fn define_enum(content: &mut String, types: &Vec<&str>, base_name: &str) {
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

pub fn define_struct(content: &mut String, struct_name: &str, fields: &str) {
    content.push_str("#[derive(Clone)]\n");
    content.push_str(&format!("pub struct {} {{\n", struct_name));
    for field in split_fields(fields) {
        let field = field.trim();
        content.push_str(&format!("    pub {},\n", field));
    }
    content.push_str("}\n\n");
}

pub fn split_fields(fields: &str) -> Vec<&str> {
    fields.split(',').collect()
}
