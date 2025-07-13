use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn integer_literals() {
    let input = "5;";

}

fn check_integer_literal(input: &str, expected: i64) -> bool {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    true
}
