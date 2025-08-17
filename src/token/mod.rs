#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LSquare,
    RSquare,
    Function,
    Let,
    // Used in let rec statements
    Rec,
    Bang,
    Minus,
    Slash,
    Star,
    LArrow,
    RArrow,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
    GreaterEq,
    LessEq,
    Null,
    Def,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }
}
