#[cfg(test)]
use crate::ast::{Identifier, LetStatement};
#[cfg(test)]
use crate::ast::Node;
#[cfg(test)]
use crate::token::{Token, TokenType};

#[test]
fn test_ast_string() {
    let let_statement = LetStatement::new(
        Token::new(TokenType::Let, "let"),
        Identifier::new(Token::new(TokenType::Ident, "bob"), "bob"),
    );
    assert_eq!(let_statement.string(), "let bob = <placeholder>;")
}
