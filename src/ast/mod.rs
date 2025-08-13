mod tests;

use std::any::Any;

use dyn_clone::DynClone;

use crate::token::Token;

/// Represents a node in the AST. Each node implements the `token_literal` function, which
/// is mainly used for debugging purposes. It returns the literal of the token associated
/// with this node.
pub trait Node: DynClone {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn string(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

dyn_clone::clone_trait_object!(Node);
dyn_clone::clone_trait_object!(Statement);
dyn_clone::clone_trait_object!(Expression);

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        "Program".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let mut ret = Vec::new();
        for statements in &self.statements {
            ret.push(statements.string());
        }
        ret.join("\n")
    }
}

// ========== Identifier Start ==========

#[derive(Clone)]
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

#[derive(Clone)]
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

// ========== Def statement Start ==========

#[derive(Clone)]
pub struct DefStatement {
    pub token: Token,
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl DefStatement {
    pub fn new(
        token: Token,
        name: Identifier,
        parameters: Vec<Identifier>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            name,
            parameters,
            body,
        }
    }
}

impl Node for DefStatement {
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
        format!(
            "def {}({}) {}",
            self.name.value,
            parameter_string,
            self.body.string()
        )
    }
}

impl Statement for DefStatement {}

// ========== Def statement End ==========

// ========== Return statement Start ==========

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

// ========== Null literal Start ==========

#[derive(Clone)]
pub struct NullLiteral {
    pub token: Token,
}

impl NullLiteral {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl Node for NullLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        "null".to_string()
    }
}

impl Expression for NullLiteral {}

// ========== Null literal End ==========

// ========== Prefix expression Start ==========

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
                    "if ({}) {} else {}",
                    self.condition.string(),
                    self.consequence.string(),
                    alternative.string()
                )
            }
            None => {
                format!(
                    "if ({}) {}",
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Self {
            token,
            parameters,
            body,
        }
    }
}

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

// ========== Call expression Start ==========

#[derive(Clone)]
pub struct CallExpression {
    pub token: Token,                  // The ( token
    pub function: Box<dyn Expression>, // Even though the type allows for any Expression here, in practice this should only be an identifier or a function literal
    pub arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let function = self.function.string();
        let arguments = self
            .arguments
            .iter()
            .map(|arg| arg.string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{function}({arguments})")
    }
}

impl Expression for CallExpression {}

// ========== Call expression End ==========

// ========== Index expression Start ==========

#[derive(Clone)]
pub struct IndexExpression {
    pub token: Token,
    // Even though the type allows any Expression here, this should only be an array expression or
    // a map expression (if I ever add map expressions)
    pub collection: Box<dyn Expression>,
    // Even though the type allows any Expression here, this should only be an integer (for array
    // indexing) or a string (for map indexing)
    pub index: Box<dyn Expression>,
}

impl IndexExpression {
    pub fn new(token: Token, collection: Box<dyn Expression>, index: Box<dyn Expression>) -> Self {
        Self {
            token,
            collection,
            index,
        }
    }
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let array_string = self.collection.string();
        let index_string = self.index.string();
        format!("{}[{}]", array_string, index_string)
    }
}

impl Expression for IndexExpression {}

// ========== Index expression End ==========

// ========== Array expression Start ==========

#[derive(Clone)]
pub struct ArrayExpression {
    pub token: Token,
    pub items: Vec<Box<dyn Expression>>,
}

impl ArrayExpression {
    pub fn new(token: Token, items: Vec<Box<dyn Expression>>) -> Self {
        Self { token, items }
    }
}

impl Node for ArrayExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn string(&self) -> String {
        let items = self
            .items
            .iter()
            .map(|item| item.string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{items}]")
    }
}

impl Expression for ArrayExpression {}

// ========== Array expression End ==========
