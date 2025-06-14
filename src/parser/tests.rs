#[cfg(test)]
use crate::ast::LetStatement;
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::parser::Parser;

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

#[cfg(test)]
fn check_let_statement(let_statement: &LetStatement, expected_identifier_literal: &str) -> bool {
    let_statement.name.value == expected_identifier_literal
}
