const BOX_REQUIRED: [&str; 7] = [
    "Assign", "Binary", "Grouping", "Literal", "Unary", "Variable", "If",
];

pub fn define_enum(content: &mut String, types: &Vec<&str>, base_name: &str) {
    content.push_str("#[derive(Clone)]\n");
    content.push_str(&format!("pub enum {} {{\n", base_name));
    for type_string in types {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        if BOX_REQUIRED.contains(&struct_name) {
            content.push_str(&format!("    {}(Box<{}>),\n", struct_name, struct_name))
        } else {
            content.push_str(&format!("    {}({}),\n", struct_name, struct_name))
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

pub fn define_new_function(content: &mut String, struct_name: &str, fields: &str) {
    let function = if BOX_REQUIRED.contains(&struct_name) {
        new_boxed_function(struct_name, fields)
    } else {
        new_function(struct_name, fields)
    };
    content.push_str(&function)
}

fn new_boxed_function(struct_name: &str, fields: &str) -> String {
    let mut content = String::new();
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
    content
}

fn new_function(struct_name: &str, fields: &str) -> String {
    let mut content = String::new();
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
    content
}

fn split_fields(fields: &str) -> Vec<&str> {
    fields.split(',').collect()
}
