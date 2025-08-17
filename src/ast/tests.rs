#[cfg(test)]
use crate::ast::{BlockStatement, FunctionLiteral, Identifier, IntegerLiteral, LetStatement, Node};
#[cfg(test)]
use crate::token::{Token, TokenType};

#[test]
fn test_let_statement() {
    let let_statement = LetStatement::new(
        Token::new(TokenType::Let, "let"),
        Identifier::new(Token::new(TokenType::Ident, "bob"), "bob"),
        Box::new(IntegerLiteral::new(Token::new(TokenType::Int, "10"), 10)),
        false,
    );
    assert_eq!(let_statement.string(), "let bob = 10;")
}

#[test]
fn test_let_rec_statement() {
    let let_statement = LetStatement::new(
        Token::new(TokenType::Let, "let"),
        Identifier::new(Token::new(TokenType::Ident, "bob"), "bob"),
        Box::new(FunctionLiteral::new(
            Token::new(TokenType::Function, "fun"),
            Vec::new(),
            BlockStatement::new(Token::new(TokenType::LBrace, "{"), Vec::new()),
        )),
        true,
    );
    assert_eq!(let_statement.string(), "let rec bob = fun() {  };")
}
