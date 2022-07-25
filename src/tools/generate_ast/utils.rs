use crate::expr;

const BOX_REQUIRED: [&str; 10] = [
    "Assign", "Binary", "Call", "Get", "Grouping", "Logical", "Set", "Unary", "If", "While",
];

pub fn define_enum(types: Vec<String>, base_name: String) -> String {
    let fields: String = types.iter().fold(String::new(), |acc, type_string| {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        let field = if BOX_REQUIRED.contains(&struct_name) {
            format!("{struct_name}(Box<{struct_name}>),\n")
        } else {
            format!("{struct_name}({struct_name}),\n")
        };
        acc + &field
    });
    format!(
        "
    #[derive(Clone)]
    pub enum {} {{
        {}
    }}

    ",
        base_name, fields
    )
}

pub fn define_visitor(types: Vec<String>, base_name: String) -> String {
    let signatures: String = types.iter().fold(String::new(), |acc, type_string| {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim();
        let lowered_struct_name = struct_name.to_lowercase();
        acc + &format!(
            "fn visit_{lowered_struct_name}_{base_name}(&mut self, {base_name}: &{struct_name}) -> T;\n"
        )
    });
    format!(
        "pub trait Visitor<T> {{
            {signatures}
        }}

        "
    )
}

pub fn define_accept(base_name: String) -> String {
    let first_param = get_accept_first_param(base_name);
    format!(
        "pub trait Accept<T> {{
             fn accept({first_param}, visitor: &mut impl Visitor<T>) -> T;
         }}

    "
    )
}

pub fn define_struct(struct_name: String, fields: String) -> String {
    let fields: String = fields.split(',').fold(String::new(), |acc, field| {
        acc + &format!("pub {},\n", field.trim())
    });
    format!(
        "
    #[derive(Clone)]
    pub struct {struct_name} {{
        {fields}
    }}

    "
    )
}

pub fn define_new_function(struct_name: String, fields: String) -> String {
    if BOX_REQUIRED.contains(&struct_name.as_str()) {
        new_boxed_function(struct_name, fields)
    } else {
        new_function(struct_name, fields)
    }
}

fn new_boxed_function(struct_name: String, fields: String) -> String {
    let arguments: String = fields.split(',').fold(String::new(), |acc, field| {
        let argument = field.split(':').collect::<Vec<&str>>()[0].trim();
        acc + &format!("{},\n", argument)
    });
    format!(
        "
        pub fn new({fields}) -> Box<{struct_name}> {{
            Box::new(
                {struct_name} {{\n
                    {arguments}
                }}
            )
        }}

        "
    )
}

fn new_function(struct_name: String, fields: String) -> String {
    let arguments: String = fields.split(',').fold(String::new(), |acc, field| {
        let argument = field.split(':').collect::<Vec<&str>>()[0].trim();
        acc + &format!("{},\n", argument)
    });
    format!(
        "
        pub fn new({fields}) -> {struct_name} {{
            {struct_name} {{\n
                {arguments}
            }}
        }}

        "
    )
}

pub fn define_structs(types: Vec<String>, base_name: String) -> String {
    types.iter().fold(String::new(), |acc, type_string| {
        let struct_name_and_fields: Vec<&str> = type_string.split(';').collect();
        let struct_name = struct_name_and_fields[0].trim().to_string();
        let fields = struct_name_and_fields[1].trim().to_string();
        let struct_in_string = define_struct(struct_name.clone(), fields.clone());
        let new_function = define_new_function(struct_name.clone(), fields);
        let lowered_struct_name = struct_name.to_lowercase();
        let lowered_base_name = base_name.to_lowercase();
        let first_param = get_accept_first_param(base_name.clone());
        acc + &format!(
            "
            {struct_in_string}

            impl {struct_name} {{
                {new_function}
            }}

            impl<T> Accept<T> for {struct_name} {{
                fn accept({first_param}, visitor: &mut impl Visitor<T>) -> T {{
                    visitor.visit_{lowered_struct_name}_{lowered_base_name}(self)
                }}
            }}

            "
        )
    })
}

pub fn define_accept_for_enum(types: Vec<String>, base_name: String) -> String {
    let accept: String = types.iter().fold(String::new(), |acc, type_string| {
        let struct_name = type_string.split(';').collect::<Vec<&str>>()[0].trim();
        acc + &format!("{base_name}::{struct_name}(e) => e.accept(visitor),\n")
    });
    let first_param = get_accept_first_param(base_name.clone());
    format!(
        "
        impl<T> Accept<T> for {base_name} {{
            fn accept({first_param}, visitor: &mut impl Visitor<T>) -> T {{
                match self {{
                    {accept}
                }}
            }}
        }}

        "
    )
}

fn get_accept_first_param(base_name: String) -> String {
    match base_name.as_str() {
        expr::BASE_NAME => "&mut self",
        _ => "&self",
    }
    .to_string()
}
