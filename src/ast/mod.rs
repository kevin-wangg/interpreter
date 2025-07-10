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
    pub value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> Self {
        LetStatement { token, name, value }
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
        format!("let {} = {};", self.name.string(), self.value.string())
    }
}

impl Statement for LetStatement {}

// ========== Let statement End ==========

// ========== Return statement Start ==========

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<dyn Expression>) -> Self {
        Self {
            token,
            return_value,
        }
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
        format!("return {};", self.return_value.string())
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

// ========== Integer literal Start ==========

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

// ========== Integer literal End ==========

// ========== Boolean literal Start ==========

pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }
}

impl Node for BooleanLiteral {
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

impl Expression for BooleanLiteral {}

// ========== Boolean literal End ==========

// ========== Prefix expression Start ==========

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: &str, right: Box<dyn Expression>) -> Self {
        Self {
            token,
            operator: operator.to_string(),
            right,
        }
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {}

// ========== Prefix expression End ==========

// ========== Infix expression Start ==========

pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(
        token: Token,
        operator: &str,
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            operator: operator.to_string(),
            left,
            right,
        }
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

impl Expression for InfixExpression {}

// ========== Infix expression End ==========

// ========== IfExpression Start ==========

pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        match self.alternative.as_ref() {
            Some(alternative) => {
                format!(
                    "if {} {} else {}",
                    self.condition.string(),
                    self.consequence.string(),
                    alternative.string()
                )
            }
            None => {
                format!(
                    "if {} {}",
                    self.condition.string(),
                    self.consequence.string(),
                )
            }
        }
    }
}

impl Expression for IfExpression {}

// ========== IfExpression End ==========

// ========== BlockStatement Start ==========

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> Self {
        Self { token, statements }
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let statements = self
            .statements
            .iter()
            .map(|statement| statement.string())
            .collect::<Vec<String>>()
            .join(" ");
        format!("{{ {} }}", statements)
    }
}

impl Statement for BlockStatement {}

// ========== BlockStatement End ==========

// ========== Function literal Start ==========

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let parameter_string = self
            .parameters
            .iter()
            .map(|identifier| identifier.string())
            .collect::<Vec<_>>()
            .join(",");
        format!("fun({}) {}", parameter_string, self.body.string())
    }
}

impl Expression for FunctionLiteral {}

// ========== Function literal End ==========
