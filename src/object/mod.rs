pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

pub trait Object {
    fn typ(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Integer {
    value: i64,
}

impl Object for Integer {
    fn typ(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self {
            value
        }
    }
}

pub struct Boolean {
    value: bool,
}

impl Object for Boolean {
    fn typ(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self {
            value
        }
    }
}

pub struct Null {}

impl Object for Null {
    fn typ(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

impl Null {
    pub fn new() -> Self {
        Self {}
    }
}
