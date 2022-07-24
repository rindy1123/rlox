use super::Object;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Object {
        Object::Class(LoxClass { name })
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}
