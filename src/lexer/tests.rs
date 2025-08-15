#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::token::TokenType;

#[test]
fn test_next_token_simple() {
    let input = "=+(){},;";

    let mut lexer = Lexer::new(input);

    let expected_token_types = [TokenType::Assign,
        TokenType::Plus,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Semicolon,
        TokenType::Eof];

    // Last literal is empty string because Eof token has an empty string literal
    let expected_token_literals = ["=", "+", "(", ")", "{", "}", ",", ";", ""];

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

    let expected_token_types = [TokenType::Assign,
        TokenType::Plus,
        TokenType::LParen,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::RBrace,
        TokenType::Comma,
        TokenType::Semicolon,
        TokenType::Eof];

    let expected_token_literals = ["=", "+", "(", ")", "{", "}", ",", ";", ""];

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
        let add = fun(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !true;
        5 + 12 / 10 * 2;
        5 < 105;
        10 == 10;
        10 != 9;
        5 <= 10;
        10 >= 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }
    ";

    let mut lexer = Lexer::new(input);

    let expected_token_types = [TokenType::Let,
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
        TokenType::Bang,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::Plus,
        TokenType::Int,
        TokenType::Slash,
        TokenType::Int,
        TokenType::Star,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::LArrow,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::Eq,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::NotEq,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::LessEq,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Int,
        TokenType::GreaterEq,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::If,
        TokenType::LParen,
        TokenType::Int,
        TokenType::LArrow,
        TokenType::Int,
        TokenType::RParen,
        TokenType::LBrace,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Else,
        TokenType::LBrace,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::RBrace,
        TokenType::Eof];

    let expected_token_literals = vec![
        "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fun", "(",
        "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=", "add", "(",
        "five", ",", "ten", ")", ";", "!", "true", ";", "5", "+", "12", "/", "10", "*", "2", ";",
        "5", "<", "105", ";", "10", "==", "10", ";", "10", "!=", "9", ";", "5", "<=", "10", ";",
        "10", ">=", "5", ";", "if", "(", "5", "<", "10", ")", "{", "return", "true", ";", "}",
        "else", "{", "return", "false", ";", "}", "", // Eof literal is an empty string
    ];

    for i in 0..expected_token_types.len() {
        let token = lexer.next_token();
        println!(
            "Token {}: {:?} (literal: '{}'), Expected: {:?} (literal: '{}')",
            i, token.token_type, token.literal, expected_token_types[i], expected_token_literals[i]
        );
        assert_eq!(token.token_type, expected_token_types[i]);
        assert_eq!(token.literal, expected_token_literals[i]);
    }
}
