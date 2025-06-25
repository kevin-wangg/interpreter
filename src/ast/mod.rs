mod tests;

use std::any::Any;

use crate::token::Token;

/// Represents a node in the AST. Each node implements the `token_literal` function, which
/// is mainly used for debugging purposes. It returns the literal of the token associated
/// with this node.
pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
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

// ========== Identifier Start ==========

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

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {}

// ========== Identifier End ==========

// ========== Let statement Start ==========

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

    fn string(&self) -> String {
        format!("let {} = <placeholder>;", self.name.string())
    }
}

impl Statement for LetStatement {}

// ========== Let statement End ==========

// ========== Return statement Start ==========

pub struct ReturnStatement {
    pub token: Token,
    // TODO: Uncomment this when parsing Expressions is supported
    // pub return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token /*return_value: Box<dyn Expression>*/) -> Self {
        Self { token }
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!("return <placeholder>")
    }
}

impl Statement for ReturnStatement {}

// ========== Return statement End ==========

// ========== Expression statement Start ==========

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!("{};", self.expression.string())
    }
}

impl Statement for ExpressionStatement {}

// ========== Expression statement End ==========

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        Self { token, value }
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for IntegerLiteral {}
