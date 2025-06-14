use super::TokenType;
use crate::lexer::Lexer;

#[test]
fn test_next_token_simple() {
    let input = "=+(){},;";

    let mut lexer = Lexer::new(input);

    let expected_token_types = vec![
        TokenType::Assign,
        TokenType::Plus,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    // Last literal is empty string because Eof token has an empty string literal
    let expected_token_literals = vec!["=", "+", "(", ")", "{", "}", ",", ";", ""];

    for i in 0..expected_token_types.len() {
        let token = lexer.next_token();
        assert_eq!(token.token_type, expected_token_types[i]);
        assert_eq!(token.literal, expected_token_literals[i]);
    }
}

#[test]
fn test_next_token_skip_whitespace() {
    let input = " \t\n=\r\n +\t(\n )\r{  }\t,\n;\t ";

    let mut lexer = Lexer::new(input);

    let expected_token_types = vec![
        TokenType::Assign,
        TokenType::Plus,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    let expected_token_literals = vec!["=", "+", "(", ")", "{", "}", ",", ";", ""];

    for i in 0..expected_token_types.len() {
        let token = lexer.next_token();
        assert_eq!(token.token_type, expected_token_types[i]);
        assert_eq!(token.literal, expected_token_literals[i]);
    }
}

#[test]
fn test_next_token_complex() {
    let input = "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
    ";

    let mut lexer = Lexer::new(input);

    let expected_token_types = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Function,
        TokenType::LParen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::Ident,
        TokenType::Plus,
        TokenType::Ident,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Ident,
        TokenType::LParen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::RParen,
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    let expected_token_literals = vec![
        "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn", "(",
        "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=", "add", "(",
        "five", ",", "ten", ")", ";", "", // Eof literal is an empty string
    ];

    for i in 0..expected_token_types.len() {
        let token = lexer.next_token();
        assert_eq!(token.token_type, expected_token_types[i]);
        assert_eq!(token.literal, expected_token_literals[i]);
    }
}
