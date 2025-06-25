mod tests;

use std::collections::HashMap;

use crate::ast::{Expression, ExpressionStatement, IntegerLiteral, ReturnStatement};

use crate::{
    ast::{Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_functions: HashMap<TokenType, fn(&mut Parser) -> Option<Box<dyn Expression>>>,
    infix_parse_functions: HashMap<TokenType, fn(Box<dyn Expression>) -> Box<dyn Expression>>,
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
            prefix_parse_functions: HashMap::new(),
            infix_parse_functions: HashMap::new(),
        };
        // Advance the parser by two tokens so
        // both cur_token and peek_token are populated
        parser.next_token();
        parser.next_token();

        parser.register_prefix_function(TokenType::Ident, |parser| parser.parse_identifier());
        parser.register_prefix_function(TokenType::Int, |parser| parser.parse_integer_literal());
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
            TokenType::Return => self.parse_return_statement(),
            // Default case is assume we are parsing an expression statement
            _ => self.parse_expression_statement()
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();
        let expression = if let Some(expression) = self.parse_expression(Precendence::Lowest as i32) {
            expression
        } else {
            return None;
        };

        if !self.expect_peek(TokenType::Semicolon) {
            self.expect_error(TokenType::Semicolon);
            return None;
        }

        Some(Box::new(ExpressionStatement::new(token, expression)))
    }

    fn parse_expression(&mut self, precendence: i32) -> Option<Box<dyn Expression>> {
        let prefix_function = if let Some(f) = self.prefix_parse_functions.get(&self.cur_token.token_type) {
            f
        } else {
            return None;
        };

        prefix_function(self)
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

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        // This check is technically not needed since if we enter this function,
        // the current token should have TokenType::Return.
        let token = if self.cur_token.token_type == TokenType::Return {
            self.cur_token.clone()
        } else {
            return None;
        };
        // Skip to the end of the statement for now since parsing expressions is not yet supported
        self.skip_to_statement_end();
        Some(Box::new(ReturnStatement::new(token)))
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

    /// Adds a parser error indicating the expected token type was not found
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

    pub fn register_prefix_function(
        &mut self,
        token_type: TokenType,
        prefix_function: fn(&mut Parser) -> Option<Box<dyn Expression>>,
    ) {
        self.prefix_parse_functions
            .insert(token_type, prefix_function);
    }

    pub fn register_infix_function(
        &mut self,
        token_type: TokenType,
        infix_function: fn(Box<dyn Expression>) -> Box<dyn Expression>,
    ) {
        self.infix_parse_functions
            .insert(token_type, infix_function);
    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier::new(
            self.cur_token.clone(),
            &self.cur_token.literal,
        )))
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        match token.literal.parse::<i64>() {
            Ok(value) => Some(Box::new(IntegerLiteral::new(token, value))),
            Err(_) => {
                self.errors.push(format!("Could not parse {} as integer", token.literal));
                None
            }
        }
    }
}

enum Precendence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call
}
