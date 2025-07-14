#[cfg(test)]
use crate::evaluator::Evaluator;
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::object::{Boolean, Integer, Null, Object};
#[cfg(test)]
use crate::parser::Parser;

#[test]
fn integer_literal_evaluation() {
    let tests = vec![
        ("5;", 5),
        ("10;", 10),
        ("-5;", -5),
        ("-10;", -10),
        ("0;", 0),
        ("42;", 42),
        ("-42;", -42),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn boolean_literal_evaluation() {
    let tests = vec![
        ("true;", true),
        ("false;", false),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn null_literal_evaluation() {
    let input = "null;";
    let evaluated = test_eval(input);
    test_null_object(&evaluated);
}

#[test]
fn bang_operator_evaluation() {
    let tests = vec![
        ("!true;", false),
        ("!false;", true),
        ("!!true;", true),
        ("!!false;", false),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn minus_operator_evaluation() {
    let tests = vec![
        ("-5;", -5),
        ("-10;", -10),
        ("--5;", 5),
        ("--10;", 10),
        ("-0;", 0),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn integer_infix_expressions() {
    let tests = vec![
        ("5 + 5;", 10),
        ("5 - 5;", 0),
        ("5 * 5;", 25),
        ("5 / 5;", 1),
        ("10 / 2;", 5),
        ("2 * 3 + 4;", 10),
        ("5 + 2 * 3;", 11),
        ("(5 + 2) * 3;", 21),
        ("5 - 2 * 3;", -1),
        ("20 / 4 + 2;", 7),
        ("2 * (5 + 5);", 20),
        ("3 * 3 * 3 + 10;", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10;", 50),
        ("-5 + 10;", 5),
        ("10 - 5;", 5),
        ("-10 + 5;", -5),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn comparison_infix_expressions() {
    let tests = vec![
        ("1 < 2;", true),
        ("1 > 2;", false),
        ("1 < 1;", false),
        ("1 > 1;", false),
        ("1 <= 2;", true),
        ("1 >= 2;", false),
        ("1 <= 1;", true),
        ("1 >= 1;", true),
        ("2 <= 1;", false),
        ("2 >= 1;", true),
        ("5 > 3;", true),
        ("3 < 5;", true),
        ("10 >= 10;", true),
        ("10 <= 10;", true),
        ("5 + 5 > 8;", true),
        ("3 * 3 < 10;", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn equality_infix_expressions() {
    let tests = vec![
        // Integer equality
        ("1 == 1;", true),
        ("1 != 1;", false),
        ("1 == 2;", false),
        ("1 != 2;", true),

        // Boolean equality
        ("true == true;", true),
        ("false == false;", true),
        ("true == false;", false),
        ("true != false;", true),
        ("false != true;", true),
        ("false != false;", false),

        // Null equality
        ("null == null;", true),
        ("null != null;", false),

        // Cross-type equality
        ("1 == true;", false),
        ("1 != true;", true),
        ("0 == false;", false),
        ("0 != false;", true),
        ("null == 0;", false),
        ("null != 0;", true),
        ("null == false;", false),
        ("null != false;", true),

        // Complex expressions
        ("(1 < 2) == true;", true),
        ("(1 < 2) != false;", true),
        ("(1 > 2) == false;", true),
        ("(1 > 2) != true;", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn division_by_zero_error() {
    let input = "5 / 0;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval(&Box::new(program));
    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.error_message.contains("Division by zero"));
    }
}

#[test]
fn unknown_operator_errors() {
    let tests = vec![
        "true + false;", // Invalid operation for booleans
        "null * 5;", // Invalid operation with null
    ];

    for input in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();

        let result = evaluator.eval(&Box::new(program));
        assert!(result.is_err(), "Expected error for input: {}", input);
    }
}

#[test]
fn complex_expressions() {
    let tests = vec![
        ("!true == false;", true),
        ("!false == true;", true),
        ("!(5 > 3);", false),
        ("!(3 > 5);", true),
        ("!(true == false);", true),
        ("!(false != true);", false),
        ("5 * 2 + 3 == 13;", true),
        ("10 - 5 * 2 == 0;", true),
        ("(5 + 5) / 2 == 5;", true),
        ("2 * (3 + 4) > 10;", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

// Helper functions

#[cfg(test)]
fn test_eval(input: &str) -> Box<dyn Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new();

    evaluator.eval(&Box::new(program)).expect("Evaluation failed")
}

#[cfg(test)]
fn test_integer_object(obj: &Box<dyn Object>, expected: i64) {
    if let Some(integer) = obj.as_any().downcast_ref::<Integer>() {
        assert_eq!(integer.value, expected, "Integer value mismatch");
    } else {
        panic!("Expected Integer object, got different type");
    }
}

#[cfg(test)]
fn test_boolean_object(obj: &Box<dyn Object>, expected: bool) {
    if let Some(boolean) = obj.as_any().downcast_ref::<Boolean>() {
        assert_eq!(boolean.value, expected, "Boolean value mismatch");
    } else {
        panic!("Expected Boolean object, got different type");
    }
}

#[cfg(test)]
fn test_null_object(obj: &Box<dyn Object>) {
    if !obj.as_any().is::<Null>() {
        panic!("Expected Null object, got different type");
    }
}
