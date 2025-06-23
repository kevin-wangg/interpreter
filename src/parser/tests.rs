#[cfg(test)]
use crate::ast::LetStatement;
#[cfg(test)]
use crate::ast::ReturnStatement;
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::parser::Parser;
#[cfg(test)]
use crate::token::TokenType;

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 82388;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected_identifier_literals = vec!["x", "y", "foobar"];
    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 3);
        for i in 0..program.statements.len() {
            let statement = &program.statements[i];
            let let_statement = statement
                .as_any()
                .downcast_ref::<LetStatement>()
                .expect("Expected let statement");

            assert!(check_let_statement(
                let_statement,
                expected_identifier_literals[i]
            ))
        }
    } else {
        assert!(false, "Failed to parse program")
    }
}

#[test]
fn test_return_statements() {
    let input = "
        return 10;
        return foo;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 2);
        for i in 0..program.statements.len() {
            let statement = &program.statements[i];
            let return_statement = statement
                .as_any()
                .downcast_ref::<ReturnStatement>()
                .expect("Expected return statement");

            assert!(check_return_statement(return_statement))
        }
    } else {
        assert!(false, "Failed to parse program")
    }
}

#[cfg(test)]
fn check_parser_errors(parser: &Parser) {
    let errors = parser.get_errors();
    if !errors.is_empty() {
        for error in errors {
            eprintln!("Parser error: {}", error);
        }
        assert!(false, "Parser has {} error(s)", errors.len());
    }
}

// TODO: Add check for expression literal when expression parsing is supported
#[cfg(test)]
fn check_let_statement(
    let_statement: &LetStatement,
    expected_identifier_literal: &str,
    /*expected_expression_literal: &str,*/
) -> bool {
    let_statement.token.token_type == TokenType::Let
        && let_statement.name.value == expected_identifier_literal
}

// TODO: Add check for expression literal when expression parsing is supported
#[cfg(test)]
fn check_return_statement(
    return_statement: &ReturnStatement,
    /*expected_expression_literal: &str*/
) -> bool {
    return_statement.token.token_type == TokenType::Return
}
