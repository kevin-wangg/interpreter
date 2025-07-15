use std::any::Any;

pub trait Object: Any {
    fn as_any(&self) -> &dyn Any;
    fn inspect(&self) -> String;
}

// ========== Integer Start ==========

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

// ========== Integer End ==========

// ========== Boolean Start ==========

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

// ========== Boolean End ==========

// ========== Null Start ==========

pub struct Null {}

impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
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

// ========== Null End ==========

pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

impl ReturnValue {
    pub fn new(value: Box<dyn Object>) -> Self {
        Self { value }
    }
}
