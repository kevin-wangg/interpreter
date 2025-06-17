mod tests;

use crate::{
    ast::{Identifier, LetStatement, Node, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        // Initialize the tokens to random numbers, since they can't be null
        // and the actual value of the token doesn't matter
        let mut parser = Self {
            lexer,
            cur_token: Token::new(TokenType::Int, "6"),
            peek_token: Token::new(TokenType::Int, "9"),
            errors: Vec::new(),
        };
        // Advance the parser by two tokens so
        // both cur_token and peek_token are populated
        parser.next_token();
        parser.next_token();
        parser
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new(Vec::new());

        while self.cur_token.token_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement)
            } else {
                // If we failed to parse the statement, then just skip to the end to avoid the bad tokens.
                self.skip_to_statement_end();
            }

            // Add a parser error if the statement does not end with a semicolon, but still continue parsing
            // following statements.
            if self.cur_token.token_type != TokenType::Semicolon {
                self.expect_error(TokenType::Semicolon);
            } else {
                self.next_token();
            }
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        // This check is technically not needed since if we enter this function,
        // the current token should have TokenType::Let.
        let token = if self.cur_token.token_type == TokenType::Let {
            self.cur_token.clone()
        } else {
            return None;
        };
        // If the next token is TokenType::Ident, then we advance the token pointers.
        // Then `cur_token` points to the Identifier token.
        let name = if self.expect_peek(TokenType::Ident) {
            Identifier::new(self.cur_token.clone(), &self.cur_token.literal)
        } else {
            return None;
        };
        // We expect an Assign token after the Identifier token. If present,
        // then consume it and advance the token pointers. Otherwise, return early.
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        // Skip to end of statement for now since parsing expressions is not yet supported
        self.skip_to_statement_end();
        Some(Box::new(LetStatement::new(token, name)))
    }

    /// If `peek_token` is equal to the expected token type, then
    /// advance the token pointers and return true. Otherwise
    /// returns false and the token pointers do not change.
    fn expect_peek(&mut self, expected_token_type: TokenType) -> bool {
        if self.peek_token.token_type == expected_token_type {
            self.next_token();
            true
        } else {
            self.expect_error(expected_token_type);
            false
        }
    }

    fn expect_error(&mut self, expected_token_type: TokenType) {
        self.errors.push(format!(
            "Expected  {:?}, found {:?} instead",
            expected_token_type, self.peek_token.token_type
        ))
    }

    /// This method should only be used during development. It is used to
    /// skip to the end of a statement to skip over AST nodes that the parser
    /// doesn't support yet.
    fn skip_to_statement_end(&mut self) {
        while self.cur_token.token_type != TokenType::Semicolon
            && self.cur_token.token_type != TokenType::Eof
        {
            self.next_token();
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }
}
