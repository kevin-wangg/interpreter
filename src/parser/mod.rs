mod tests;

use crate::{
    ast::Program,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        // Initialize the tokens to random numbers, since they can't be null
        // and the actual value of the token doesn't matter
        let mut parser = Self {
            lexer,
            cur_token: Token::new(TokenType::Int, "6"),
            peek_token: Token::new(TokenType::Int, "9"),
        };
        // Advance the parser by two tokens so
        // both cur_token and peek_token are populated
        parser.next_token();
        parser.next_token();
        parser
    }

    fn parse_program(&mut self) -> Option<Program> {
        todo!()
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}
