#[cfg(test)]
use crate::ast::{
    ArrayExpression, BooleanLiteral, CallExpression, ExpressionStatement, FunctionLiteral,
    Identifier, IfExpression, IndexExpression, InfixExpression, IntegerLiteral, LetStatement, Node, NullLiteral,
    PrefixExpression, ReturnStatement,
};
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::parser::{Parser, has_parser_errors};
#[cfg(test)]
use crate::token::TokenType;

#[test]
fn let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 82388;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let expected_identifier_literals = vec!["x", "y", "foobar"];
    let expected_values = vec!["5", "10", "82388"];
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 3);
    for i in 0..program.statements.len() {
        let statement = &program.statements[i];
        let let_statement = statement
            .as_any()
            .downcast_ref::<LetStatement>()
            .expect("Expected let statement");

        assert!(check_let_statement(
            let_statement,
            expected_identifier_literals[i],
            expected_values[i]
        ))
    }
}

#[test]
fn return_statements() {
    let input = "
        return 10;
        return foo;
    ";
    let expected_values = vec!["10", "foo"];
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 2);
    for i in 0..program.statements.len() {
        let statement = &program.statements[i];
        let return_statement = statement
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .expect("Expected return statement");
        assert!(check_return_statement(return_statement, expected_values[i]))
    }
}

// Not really a complete test since it doesn't call `parse_program`, but still included
// to make sure block statement parsing works.
#[test]
fn block_statement() {
    let input = "{ let a = 10; a + b; return 10; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let block_statement = parser
        .parse_block_statement()
        .expect("Failed to parser block statement");
    assert_eq!(
        block_statement.string(),
        "{ let a = 10; (a + b); return 10; }"
    );
}

#[test]
fn if_expression() {
    let input = "if (x < y) { return x; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    if true {
        println!();
    }

    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 1);

    let statement = &program.statements[0];
    let expression_statement = statement
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected expression statement");

    let if_expression = expression_statement
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect("Expected if expression");

    assert_eq!(if_expression.condition.string(), "(x < y)");
    assert_eq!(if_expression.consequence.string(), "{ return x; }");
    assert!(if_expression.alternative.is_none());
}

#[test]
fn if_else_expression() {
    let input = "if (x < y) { return x; } else { return y; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 1);

    let statement = &program.statements[0];
    let expression_statement = statement
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected expression statement");

    let if_expression = expression_statement
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect("Expected if expression");

    assert_eq!(if_expression.condition.string(), "(x < y)");
    assert_eq!(if_expression.consequence.string(), "{ return x; }");
    assert!(if_expression.alternative.is_some());
    assert_eq!(
        if_expression.alternative.as_ref().unwrap().string(),
        "{ return y; }"
    );
}

#[test]
fn function_literal_expression() {
    let input = "fun(a, b) { return a + b; };";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 1);
    let statement = &program.statements[0];
    let expression_statement = statement
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected expression statement");
    let function_literal = expression_statement
        .expression
        .as_any()
        .downcast_ref::<FunctionLiteral>()
        .expect("Expected function literal");
    check_params_list(&function_literal.parameters, vec!["a", "b"]);
}

#[test]
fn identifier_expression() {
    let input = "foobar;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
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
}

#[test]
fn integer_literal_expression() {
    let input = "10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
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
}

#[test]
fn boolean_literal_expression() {
    let input = "false;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
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
}

#[test]
fn null_literal_expression() {
    let input = "null;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 1);
    let statement = &program.statements[0];
    let expression_statement = statement
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected expression statement");
    let null_literal = expression_statement
        .expression
        .as_any()
        .downcast_ref::<NullLiteral>()
        .expect("Expected null literal expression");
    assert_eq!(null_literal.token_literal(), "null");
}

#[test]
fn bang_expression() {
    let input = "!true;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
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
}

#[test]
fn minus_expression() {
    let input = "-42;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
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
}

#[test]
fn infix_expressions() {
    let tests = vec![
        vec!["5 + 5;", "5", "+", "5"],
        vec!["5 - 5;", "5", "-", "5"],
        vec!["5 * 5;", "5", "*", "5"],
        vec!["5 / 5;", "5", "/", "5"],
        vec!["5 > 5;", "5", ">", "5"],
        vec!["5 < 5;", "5", "<", "5"],
        vec!["5 == 5;", "5", "==", "5"],
        vec!["5 != 5;", "5", "!=", "5"],
        vec!["5 >= 5;", "5", ">=", "5"],
        vec!["5 <= 5;", "5", "<=", "5"],
        vec!["true == true;", "true", "==", "true"],
        vec!["true != false;", "true", "!=", "false"],
        vec!["false == false;", "false", "==", "false"],
    ];

    for test in tests {
        let lexer = Lexer::new(test[0]);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(!has_parser_errors(&parser));
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
    }
}

#[test]
fn operator_precedence() {
    let tests = vec![
        ("5 + 5 * 5;", "(5 + (5 * 5))"),
        ("5 * 5 + 5;", "((5 * 5) + 5)"),
        ("2 + 3 * 4 + 5;", "((2 + (3 * 4)) + 5)"),
        ("2 * 3 + 4 * 5;", "((2 * 3) + (4 * 5))"),
        ("5 + 5 / 5;", "(5 + (5 / 5))"),
        ("5 / 5 + 5;", "((5 / 5) + 5)"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5;",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("3 > 5 == false;", "((3 > 5) == false)"),
        ("3 < 5 == true;", "((3 < 5) == true)"),
        ("(1 + 2) * 3;", "((1 + 2) * 3)"),
        ("2 * (3 + 4);", "(2 * (3 + 4))"),
        ("(5 + 5) * 2 * (5 + 5);", "(((5 + 5) * 2) * (5 + 5))"),
        ("-(5 + 5);", "(-(5 + 5))"),
        ("!(true == true);", "(!(true == true))"),
        ("(1 + 2) * 3 + 4;", "(((1 + 2) * 3) + 4)"),
        ("1 + (2 * 3) + 4;", "((1 + (2 * 3)) + 4)"),
        ("(1 + 2) * (3 + 4);", "((1 + 2) * (3 + 4))"),
        ("(5 + 5) / (2 + 3);", "((5 + 5) / (2 + 3))"),
        ("add(2, 3) + 4;", "(add(2, 3) + 4)"),
        ("4 + add(2, 3);", "(4 + add(2, 3))"),
        ("add(2, 3) * 4;", "(add(2, 3) * 4)"),
        ("4 * add(2, 3);", "(4 * add(2, 3))"),
        ("add(1, 2) + add(3, 4);", "(add(1, 2) + add(3, 4))"),
        ("add(1, 2) * add(3, 4);", "(add(1, 2) * add(3, 4))"),
        ("add(2 + 3, 4);", "add((2 + 3), 4)"),
        ("add(2 * 3, 4 + 5);", "add((2 * 3), (4 + 5))"),
        ("-add(2, 3);", "(-add(2, 3))"),
        ("!add(true, false);", "(!add(true, false))"),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(!has_parser_errors(&parser));
        assert!(program.statements.len() == 1);
        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");
        let actual = expression_statement.expression.string();
        assert_eq!(actual, expected, "Input: {}", input);
    }
}

#[test]
fn call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert!(!has_parser_errors(&parser));
    assert!(program.statements.len() == 1);

    let statement = &program.statements[0];
    let expression_statement = statement
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected expression statement");

    let call_expression = expression_statement
        .expression
        .as_any()
        .downcast_ref::<CallExpression>()
        .expect("Expected call expression");

    assert_eq!(call_expression.function.string(), "add");
    assert_eq!(call_expression.arguments.len(), 3);
    assert_eq!(call_expression.arguments[0].string(), "1");
    assert_eq!(call_expression.arguments[1].string(), "(2 * 3)");
    assert_eq!(call_expression.arguments[2].string(), "(4 + 5)");
}

#[test]
fn test_array_expressions() {
    let tests = vec![
        ("[1, 2, 3];", vec!["1", "2", "3"]),
        ("[true, false];", vec!["true", "false"]),
        ("[1, true, null];", vec!["1", "true", "null"]),
        ("[];", vec![]),
        (
            "[1 + 2, 3 * 4, 5 - 6];",
            vec!["(1 + 2)", "(3 * 4)", "(5 - 6)"],
        ),
        ("[x, y, z];", vec!["x", "y", "z"]),
        ("[fun(x) { x }, 42];", vec!["fun(x) { x; }", "42"]),
    ];

    for (input, expected_items) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(!has_parser_errors(&parser));
        assert_eq!(program.statements.len(), 1);
        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");

        let array_expression = expression_statement
            .expression
            .as_any()
            .downcast_ref::<ArrayExpression>()
            .expect("Expected array expression");

        assert_eq!(array_expression.items.len(), expected_items.len());

        for (i, expected_item) in expected_items.iter().enumerate() {
            assert_eq!(array_expression.items[i].string(), *expected_item);
        }
    }
}

#[test]
fn index_expressions() {
    let tests = vec![
        // Basic array indexing with integer literals
        ("arr[0];", "arr", "0"),
        ("myArray[1];", "myArray", "1"),
        ("numbers[5];", "numbers", "5"),
        ("data[10];", "data", "10"),
        // Variable as index
        ("arr[i];", "arr", "i"),
        ("data[index];", "data", "index"),
        // Expression as index
        ("arr[1 + 2];", "arr", "(1 + 2)"),
        ("items[i * 2];", "items", "(i * 2)"),
        ("list[len - 1];", "list", "(len - 1)"),
        ("matrix[x + y];", "matrix", "(x + y)"),
        // Array expression with various indices
        ("[1, 2, 3][0];", "[1, 2, 3]", "0"),
        ("[true, false][1];", "[true, false]", "1"),
        ("[a, b, c][i];", "[a, b, c]", "i"),
        ("[1, 2, 3][x + 1];", "[1, 2, 3]", "(x + 1)"),
        // Complex expressions as collection
        ("getArray()[0];", "getArray()", "0"),
        ("getArray()[i];", "getArray()", "i"),
        ("(a + b)[x];", "(a + b)", "x"),
        ("(func())[index];", "func()", "index"),
    ];

    for (input, expected_collection, expected_index) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(!has_parser_errors(&parser));
        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        let expression_statement = statement
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("Expected expression statement");

        let index_expression = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IndexExpression>()
            .expect("Expected index expression");

        assert_eq!(index_expression.collection.string(), expected_collection);
        assert_eq!(index_expression.index.string(), expected_index);
        assert_eq!(index_expression.token_literal(), "[");
    }
}

#[cfg(test)]
fn check_let_statement(
    let_statement: &LetStatement,
    expected_identifier_literal: &str,
    expected_expression_literal: &str,
) -> bool {
    let_statement.token.token_type == TokenType::Let
        && let_statement.name.value == expected_identifier_literal
        && let_statement.value.string() == expected_expression_literal
}

#[cfg(test)]
fn check_params_list(parameters: &Vec<Identifier>, expected: Vec<&str>) {
    assert_eq!(parameters.len(), expected.len());
    for (i, param) in parameters.iter().enumerate() {
        assert_eq!(param.value, expected[i]);
    }
}

#[cfg(test)]
fn check_return_statement(
    return_statement: &ReturnStatement,
    expected_expression_literal: &str,
) -> bool {
    return_statement.token.token_type == TokenType::Return
        && return_statement.return_value.string() == expected_expression_literal
}
