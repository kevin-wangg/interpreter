mod tests;

use std::collections::HashMap;

use crate::ast::{
    ArrayExpression, BlockStatement, BooleanLiteral, CallExpression, DefStatement, Expression, ExpressionStatement, FunctionLiteral, IfExpression, IndexExpression, InfixExpression, IntegerLiteral, NullLiteral, PrefixExpression, ReturnStatement
};

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

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
    prefix_parse_functions: HashMap<TokenType, PrefixParseFn>,
    infix_parse_functions: HashMap<TokenType, InfixParseFn>,
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

        // Register the prefix functions
        parser.register_prefix_function(TokenType::Ident, |parser| parser.parse_identifier());
        parser.register_prefix_function(TokenType::Int, |parser| parser.parse_integer_literal());
        parser.register_prefix_function(TokenType::Null, |parser| parser.parse_null());
        parser.register_prefix_function(TokenType::True, |parser| parser.parse_boolean_literal());
        parser.register_prefix_function(TokenType::False, |parser| parser.parse_boolean_literal());
        parser
            .register_prefix_function(TokenType::Minus, |parser| parser.parse_prefix_expression());
        parser.register_prefix_function(TokenType::Bang, |parser| parser.parse_prefix_expression());
        parser.register_prefix_function(TokenType::LParen, |parser| {
            parser.parse_grouped_expression()
        });
        parser
            .register_prefix_function(TokenType::LSquare, |parser| parser.parse_array_expression());
        parser.register_prefix_function(TokenType::If, |parser| parser.parse_if_expression());
        parser.register_prefix_function(TokenType::Function, |parser| {
            parser.parse_function_literal()
        });

        // Register the infix functions
        parser.register_infix_function(TokenType::Eq, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::NotEq, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::LArrow, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::RArrow, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::LessEq, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::GreaterEq, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::Plus, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::Minus, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::Star, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::Slash, |parser, left| {
            parser.parse_infix_expression(left)
        });
        parser.register_infix_function(TokenType::LParen, |parser, left| {
            parser.parse_call_expression(left)
        });
        parser.register_infix_function(TokenType::LSquare, |parser, left| {
            parser.parse_index_expression(left)
        });
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new(Vec::new());
        while self.cur_token.token_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            } else {
                // If we failed to parse the statement, then just skip to the end to avoid the bad tokens.
                self.skip_to_statement_end();
            }
        }
        program
    }

    // This function should leave the parser in a state where cur_token points to the first token
    // of the next statement. This allows us to selectively control which statements require an
    // ending semicolon.
    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Def => self.parse_def_statement(),
            // Default case is assume we are parsing an expression statement
            _ => self.parse_expression_statement(),
        }
    }

    // When this function is called, cur_token should be pointing to the LBrace
    // When this function returns, cur_token should be pointing to the RBrace
    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let token = if self.cur_token.token_type == TokenType::LBrace {
            self.cur_token.clone()
        } else {
            return None;
        };
        self.next_token();
        let mut statements = Vec::new();
        while self.peek_token.token_type != TokenType::Eof {
            let statement = self.parse_statement()?;
            statements.push(statement);
            if self.cur_token.token_type == TokenType::RBrace {
                break;
            }
        }
        Some(BlockStatement::new(token, statements))
    }

    // When this function is called, cur_token should be pointing to the Def
    fn parse_def_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = if self.cur_token.token_type == TokenType::Def {
            self.cur_token.clone()
        } else {
            return None;
        };
        // If the next token is TokenType::Ident, then we advance the token pointers.
        // Then `cur_token` points to the Identifier token.
        let name = if self.expect_peek(TokenType::Ident) {
            Identifier::new(self.cur_token.clone(), &self.cur_token.literal)
        } else {
            self.expect_error(TokenType::Ident);
            return None;
        };
        if !self.expect_peek(TokenType::LParen) {
            self.expect_error(TokenType::LParen);
            return None;
        }
        // cur_token now points to the LParen
        let parameters = self.parse_parameter_list()?;
        if !self.expect_peek(TokenType::LBrace) {
            self.expect_error(TokenType::LBrace);
            return None;
        }
        // cur_token now points to the LBrace
        let body = self.parse_block_statement()?;
        self.next_token();
        Some(Box::new(DefStatement::new(token, name, parameters, body)))
    }

    // When this function is called, cur_token should be pointing to the Let
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
            self.expect_error(TokenType::Ident);
            return None;
        };
        // We expect an Assign token after the Identifier token. If present,
        // then consume it and advance the token pointers. Otherwise, return early.
        if !self.expect_peek(TokenType::Assign) {
            self.expect_error(TokenType::Assign);
            return None;
        }
        // Advance token to start of expression
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest as i32)?;
        // Advance token to the semicolon
        self.next_token();
        if self.cur_token.token_type != TokenType::Semicolon {
            self.expect_error(TokenType::Semicolon);
        } else {
            self.next_token();
        }
        Some(Box::new(LetStatement::new(token, name, value)))
    }

    // When this function is called, self.cur_token should be pointing to a token with
    // type TokenType::Return
    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        // This check is technically not needed since if we enter this function,
        // the current token should have TokenType::Return.
        let token = if self.cur_token.token_type == TokenType::Return {
            self.cur_token.clone()
        } else {
            return None;
        };
        // Advance token to start of expression
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest as i32)?;
        // Advance token token to the semicolon
        self.next_token();
        if self.cur_token.token_type != TokenType::Semicolon {
            self.expect_error(TokenType::Semicolon);
        } else {
            self.next_token();
        }
        Some(Box::new(ReturnStatement::new(token, return_value)))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest as i32)?;
        // let requires_semi = self.expr_requires_semi_to_be_stmt(&*expression);
        // Advance token to potentially a semicolon
        self.next_token();
        // If the current token is not a semicolon and semicolon is required, then add a
        // parser error.
        // If the current token is not a semicolon but semicolon is not required, then do nothing.
        // If the current token is a semicolon, regardless of whether semicolon is required or not,
        // advance to the next token.
        //
        // Uncomment this code block and comment the block below to enforce semicolons on all
        // statements
        // if self.cur_token.token_type != TokenType::Semicolon {
        //     if requires_semi {
        //         self.expect_error(TokenType::Semicolon);
        //         return None;
        //     }
        // } else {
        //     self.next_token();
        // }
        if self.cur_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        Some(Box::new(ExpressionStatement::new(token, expression)))
    }

    /// Parses an expression and returns an AST node representing that expression.
    /// This function consumes tokens up to and including the last token in the expression.
    /// Namely, it does NOT consume the semicolon (or comma) following an expression, so cur_token
    /// is pointing to the last token that is part of the expression when the function returns.
    fn parse_expression(&mut self, precedence: i32) -> Option<Box<dyn Expression>> {
        let prefix_function =
            if let Some(f) = self.prefix_parse_functions.get(&self.cur_token.token_type) {
                f
            } else {
                self.no_prefix_function_error(self.cur_token.token_type);
                return None;
            };
        let mut left = prefix_function(self)?;
        loop {
            if self.peek_token.token_type == TokenType::Semicolon
                || self.peek_token.token_type == TokenType::Comma
            {
                break;
            } else {
                let next_precedence =
                    Parser::token_to_precedence(self.peek_token.token_type) as i32;
                if next_precedence > precedence {
                    let infix_function = if let Some(f) =
                        self.infix_parse_functions.get(&self.peek_token.token_type)
                    {
                        *f
                    } else {
                        break;
                    };
                    self.next_token();
                    left = infix_function(self, left)?;
                } else {
                    break;
                }
            }
        }
        Some(left)
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier::new(
            self.cur_token.clone(),
            &self.cur_token.literal,
        )))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        match token.literal.parse::<i64>() {
            Ok(value) => Some(Box::new(IntegerLiteral::new(token, value))),
            Err(_) => {
                self.errors
                    .push(format!("Could not parse {} as integer", token.literal));
                None
            }
        }
    }

    fn parse_boolean_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        match token.literal.parse::<bool>() {
            Ok(value) => Some(Box::new(BooleanLiteral::new(token, value))),
            Err(_) => {
                self.errors
                    .push(format!("Could not parse {} as a bool", token.literal));
                None
            }
        }
    }

    fn parse_null(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        Some(Box::new(NullLiteral::new(token)))
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix as i32)?;
        Some(Box::new(PrefixExpression::new(token, &operator, right)))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = token.literal.clone();
        let precendence = Parser::token_to_precedence(self.cur_token.token_type);
        self.next_token();
        let right = self.parse_expression(precendence as i32)?;
        Some(Box::new(InfixExpression::new(
            token, &operator, left, right,
        )))
    }

    fn parse_call_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        if !(left.as_any().is::<Identifier>() || left.as_any().is::<FunctionLiteral>()) {
            self.errors
                .push("Expected function literal or identifier in call position".to_string());
            return None;
        }
        let token = if self.cur_token.token_type == TokenType::LParen {
            self.cur_token.clone()
        } else {
            return None;
        };
        let arguments = self.parse_argument_list()?;
        Some(Box::new(CallExpression::new(token, left, arguments)))
    }

    fn parse_index_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let token = if self.cur_token.token_type == TokenType::LSquare {
            self.cur_token.clone()
        } else {
            return None;
        };
        // Advance cur_token so it points to the first token of the index value
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest as i32)?;
        if !self.expect_peek(TokenType::RSquare) {
            self.expect_error(TokenType::RSquare);
            return None;
        }
        Some(Box::new(IndexExpression::new(token, left, index)))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest as i32)?;
        if self.peek_token.token_type == TokenType::RParen {
            self.next_token();
            Some(expression)
        } else {
            self.expect_error(TokenType::RParen);
            None
        }
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        // This check is technically not needed since if we enter this function,
        // the current token should have TokenType::If.
        let token = if self.cur_token.token_type == TokenType::If {
            self.cur_token.clone()
        } else {
            return None;
        };
        if !self.expect_peek(TokenType::LParen) {
            self.expect_error(TokenType::LParen);
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest as i32)?;
        if !self.expect_peek(TokenType::RParen) {
            self.expect_error(TokenType::RParen);
            return None;
        }
        if !self.expect_peek(TokenType::LBrace) {
            self.expect_error(TokenType::LBrace);
            return None;
        }
        let consequence = self.parse_block_statement()?;

        if self.peek_token.token_type == TokenType::Else {
            self.next_token();
            if !self.expect_peek(TokenType::LBrace) {
                self.expect_error(TokenType::LBrace);
                return None;
            }
            let alternative = self.parse_block_statement()?;
            Some(Box::new(IfExpression::new(
                token,
                condition,
                consequence,
                Some(alternative),
            )))
        } else {
            Some(Box::new(IfExpression::new(
                token,
                condition,
                consequence,
                None,
            )))
        }
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = if self.cur_token.token_type == TokenType::Function {
            self.cur_token.clone()
        } else {
            return None;
        };
        if !self.expect_peek(TokenType::LParen) {
            self.expect_error(TokenType::LParen);
            return None;
        }
        // cur_token now points to the LParen
        let parameters = self.parse_parameter_list()?;

        if !self.expect_peek(TokenType::LBrace) {
            self.expect_error(TokenType::LBrace);
            return None;
        }
        // cur_token now points to the LBrace
        let body = self.parse_block_statement()?;

        Some(Box::new(FunctionLiteral::new(token, parameters, body)))
    }

    fn parse_argument_list(&mut self) -> Option<Vec<Box<dyn Expression>>> {
        // cur_token points to the LParen here
        let mut ret = Vec::new();
        if !self.expect_peek(TokenType::RParen) {
            self.next_token();
            loop {
                let argument = self.parse_expression(Precedence::Lowest as i32)?;
                ret.push(argument);
                // If the next token is RParen, then break out of the loop
                if self.expect_peek(TokenType::RParen) {
                    break;
                }
                // Otherwise we expect a comma after the identifier. If there is isn't, then add a Parser error
                // and break out of the loop
                if !self.expect_peek(TokenType::Comma) {
                    self.expect_error(TokenType::Comma);
                    return None;
                }
                self.next_token();
            }
        }
        // cur_token points to the RParen here
        Some(ret)
    }

    fn parse_parameter_list(&mut self) -> Option<Vec<Identifier>> {
        // cur_token points to the LParen here
        let mut ret = Vec::new();
        if !self.expect_peek(TokenType::RParen) {
            self.next_token();
            loop {
                let identifier = Identifier::new(self.cur_token.clone(), &self.cur_token.literal);
                ret.push(identifier);
                // If the next token is RParen, then break out of the loop
                if self.expect_peek(TokenType::RParen) {
                    break;
                }
                // Otherwise we expect a comma after the identifier. If there isn't a comma, then add a parser
                // error and break out of the loop
                if !self.expect_peek(TokenType::Comma) {
                    self.expect_error(TokenType::Comma);
                    return None;
                }
                self.next_token();
            }
        }
        // cur_token points to the RParen here
        Some(ret)
    }

    // When this function is called, cur_token should point to LSquare.
    // When it returns, cur_token should point to RSquare
    fn parse_array_expression(&mut self) -> Option<Box<dyn Expression>> {
        // cur_token points to the LSquare here
        let token = if self.cur_token.token_type == TokenType::LSquare {
            self.cur_token.clone()
        } else {
            return None;
        };
        let mut items = Vec::new();
        if !self.expect_peek(TokenType::RSquare) {
            self.next_token();
            loop {
                // Parse the item, which should be an expression
                let item = self.parse_expression(Precedence::Lowest as i32)?;
                items.push(item);
                // If the next token is RSquare, then break out of the loop
                if self.expect_peek(TokenType::RSquare) {
                    break;
                }
                // Otherwise, we expect a comma after the item. If there isn't, then add a Parser
                // error and break out of the loop
                if !self.expect_peek(TokenType::Comma) {
                    self.expect_error(TokenType::Comma);
                    return None;
                }
                self.next_token();
            }
        }
        // cur_token points to the RSqaure here
        Some(Box::new(ArrayExpression::new(token, items)))
    }

    fn no_prefix_function_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "No prefix parse function found for {:?} found",
            token_type
        ))
    }

    fn token_to_precedence(token_type: TokenType) -> Precedence {
        match token_type {
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::LArrow => Precedence::LessGreater,
            TokenType::RArrow => Precedence::LessGreater,
            TokenType::LessEq => Precedence::LessGreater,
            TokenType::GreaterEq => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Star => Precedence::Product,
            TokenType::Slash => Precedence::Product,
            TokenType::LSquare => Precedence::Call,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    /// If `peek_token` is equal to the expected token type, then
    /// advance the token pointers and return true. Otherwise
    /// returns false and the token pointers do not change.
    fn expect_peek(&mut self, expected_token_type: TokenType) -> bool {
        if self.peek_token.token_type == expected_token_type {
            self.next_token();
            true
        } else {
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

    fn skip_to_statement_end(&mut self) {
        while self.cur_token.token_type != TokenType::Semicolon
            && self.cur_token.token_type != TokenType::Eof
        {
            self.next_token();
        }
        self.next_token();
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn register_prefix_function(&mut self, token_type: TokenType, prefix_function: PrefixParseFn) {
        self.prefix_parse_functions
            .insert(token_type, prefix_function);
    }

    fn register_infix_function(&mut self, token_type: TokenType, infix_function: InfixParseFn) {
        self.infix_parse_functions
            .insert(token_type, infix_function);
    }

    #[allow(dead_code)]
    fn expr_requires_semi_to_be_stmt(&self, statement: &dyn Expression) -> bool {
        if statement.as_any().is::<IfExpression>() {
            false
        } else {
            true
        }
    }
}

pub fn has_parser_errors(parser: &Parser) -> bool {
    let errors = parser.get_errors();
    if !errors.is_empty() {
        eprintln!("Parser has {} errors(s)", errors.len());
        for error in errors {
            eprintln!("Parser error: {}", error);
        }
        true
    } else {
        false
    }
}

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}
