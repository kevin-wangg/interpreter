use std::collections::HashMap;

use crate::{ast::Identifier, object::Object};

#[derive(Clone)]
pub struct Environment {
    mapping: HashMap<String, Box<dyn Object>>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_wrapped(outer: &Environment) -> Self {
        Self {
            mapping: HashMap::new(),
            outer: Some(Box::new(outer.clone())),
        }
    }

    pub fn insert(&mut self, id: &Identifier, value: Box<dyn Object>) {
        self.mapping.insert(id.value.clone(), value);
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn Object>> {
        self.mapping.get(id).or_else(|| {
            self.outer
                .as_ref()
                .and_then(|environment| environment.get(id))
        })
    }
}
