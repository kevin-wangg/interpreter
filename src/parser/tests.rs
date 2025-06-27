use crate::ast::InfixExpression;
#[cfg(test)]
use crate::ast::{BooleanLiteral, IntegerLiteral, PrefixExpression};
#[cfg(test)]
use crate::ast::{ExpressionStatement, Identifier, LetStatement, Node, ReturnStatement};
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

#[test]
fn test_identifier_expression() {
    let input = "foobar;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 1);

        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");

        let identifier = expression_statement
            .expression
            .as_any()
            .downcast_ref::<Identifier>()
            .expect("Expected identifier expression");

        assert_eq!(identifier.value, "foobar");
        assert_eq!(identifier.token_literal(), "foobar");
    } else {
        assert!(false, "Failed to parse program");
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = "10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 1);
        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");
        let integer_literal = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("Expected integer literal expression");
        assert_eq!(integer_literal.value, 10);
        assert_eq!(integer_literal.token_literal(), "10");
    } else {
        assert!(false, "Failed to parse program");
    }
}

#[test]
fn test_boolean_literal_expression() {
    let input = "false;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 1);
        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");
        let integer_literal = expression_statement
            .expression
            .as_any()
            .downcast_ref::<BooleanLiteral>()
            .expect("Expected boolean literal expression");
        assert_eq!(integer_literal.value, false);
        assert_eq!(integer_literal.token_literal(), "false");
    } else {
        assert!(false, "Failed to parse program");
    }
}

#[test]
fn test_bang_expression() {
    let input = "!true;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 1);

        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");

        let prefix_expression = expression_statement
            .expression
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .expect("Expected prefix expression");

        assert_eq!(prefix_expression.operator, "!");
        assert_eq!(prefix_expression.token_literal(), "!");

        let right_operand = prefix_expression
            .right
            .as_any()
            .downcast_ref::<BooleanLiteral>()
            .expect("Expected boolean literal as right operand");

        assert_eq!(right_operand.value, true);
        assert_eq!(right_operand.token_literal(), "true");
    } else {
        assert!(false, "Failed to parse program");
    }
}

#[test]
fn test_minus_expression() {
    let input = "-42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    if let Some(program) = parser.parse_program() {
        check_parser_errors(&parser);
        assert!(program.statements.len() == 1);

        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");

        let prefix_expression = expression_statement
            .expression
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .expect("Expected prefix expression");

        assert_eq!(prefix_expression.operator, "-");
        assert_eq!(prefix_expression.token_literal(), "-");

        let right_operand = prefix_expression
            .right
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("Expected integer literal as right operand");

        assert_eq!(right_operand.value, 42);
        assert_eq!(right_operand.token_literal(), "42");
    } else {
        assert!(false, "Failed to parse program");
    }
}

#[test]
fn test_infix_expressions() {
    let tests = vec![vec!["5+5;", "5", "+", "5"]];

    for test in tests {
        let lexer = Lexer::new(test[0]);
        let mut parser = Parser::new(lexer);
        if let Some(program) = parser.parse_program() {
            check_parser_errors(&parser);
            assert!(program.statements.len() == 1);
            let statement = &program.statements[0];
            let expression_statement = statement
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .expect("Expected expression statement");
            let infix_expression = expression_statement
                .expression
                .as_any()
                .downcast_ref::<InfixExpression>()
                .expect("Expected infix expression");
            assert_eq!(infix_expression.left.string(), test[1]);
            assert_eq!(infix_expression.operator, test[2]);
            assert_eq!(infix_expression.right.string(), test[3]);
        } else {
            assert!(false, "Failed to parse program");
        }
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
