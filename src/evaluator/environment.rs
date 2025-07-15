use std::collections::HashMap;

use crate::object::Object;

pub struct Environment {
    mapping: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: &str, value: Box<dyn Object>) {
        self.mapping.insert(id.to_string(), value);
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn Object>> {
        self.mapping.get(id)
    }
}
