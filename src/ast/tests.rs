#[cfg(test)]
use crate::ast::{Identifier, IntegerLiteral, LetStatement, Node};
#[cfg(test)]
use crate::token::{Token, TokenType};

#[test]
fn test_ast_string() {
    let let_statement = LetStatement::new(
        Token::new(TokenType::Let, "let"),
        Identifier::new(Token::new(TokenType::Ident, "bob"), "bob"),
        Box::new(IntegerLiteral::new(Token::new(TokenType::Int, "10"), 10)),
    );
    assert_eq!(let_statement.string(), "let bob = 10;")
}
