use std::any::Any;

use crate::token::Token;

/// Represents a node in the AST. Each node implements the `token_literal` function, which
/// is mainly used for debugging purposes. It returns the literal of the token associated
/// with this node.
pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: &str) -> Self {
        Identifier {
            token,
            value: value.to_string(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    // TODO: Uncomment this when parsing Expressions is supported
    // pub value: Box<dyn Expression>,
}

impl LetStatement {
    // TODO: Add a value parameter when parsing Expressions is supported
    pub fn new(token: Token, name: Identifier) -> Self {
        LetStatement { token, name }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for LetStatement {}
