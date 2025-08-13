#[cfg(test)]
use crate::evaluator::Evaluator;
#[cfg(test)]
use crate::evaluator::environment::Environment;
#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::object::{Array, Boolean, Integer, Null, Object};
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
    let tests = vec![("true;", true), ("false;", false)];

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
    let mut env = Environment::new();

    let result = evaluator.eval(&program, &mut env);
    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.error_message.contains("Division by zero"));
    }
}

#[test]
fn unknown_operator_errors() {
    let tests = vec![
        "true + false;", // Invalid operation for booleans
        "null * 5;",     // Invalid operation with null
    ];

    for input in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();
        let mut env = Environment::new();

        let result = evaluator.eval(&program, &mut env);
        assert!(result.is_err(), "Expected error for input: {}", input);
    }
}

#[test]
fn if_expressions() {
    let tests = vec![
        // Basic if with truthy condition - note: blocks with single expressions work differently
        ("if (true) { 10; };", Some(10)),
        ("if (1) { 20; };", Some(20)),
        ("if (5 > 3) { 30; };", Some(30)),
        // Basic if with falsey condition (should return null)
        ("if (false) { 10; };", None),
        ("if (0) { 20; };", None),
        ("if (3 > 5) { 30; };", None),
        // If-else with truthy condition
        ("if (true) { 10; } else { 20; };", Some(10)),
        ("if (1) { 15; } else { 25; };", Some(15)),
        ("if (5 > 3) { 100; } else { 200; };", Some(100)),
        // If-else with falsey condition
        ("if (false) { 10; } else { 20; };", Some(20)),
        ("if (0) { 15; } else { 25; };", Some(25)),
        ("if (3 > 5) { 100; } else { 200; };", Some(200)),
        // Complex conditions
        ("if (1 + 1 == 2) { 42; };", Some(42)),
        ("if (5 * 2 > 8) { 99; } else { 11; };", Some(99)),
        ("if (10 / 2 == 4) { 1; } else { 2; };", Some(2)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        match expected {
            Some(value) => test_integer_object(&evaluated, value),
            None => test_null_object(&evaluated),
        }
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

#[test]
fn return_statements() {
    let tests = vec![
        // Simple return statements
        ("return 10;", 10),
        ("return 42;", 42),
        ("return -5;", -5),
        // Return with expression
        ("return 2 * 5;", 10),
        ("return 10 + 5;", 15),
        ("return 20 / 4;", 5),
        // Return with complex expression
        ("return 5 + 5 * 2;", 15),
        ("return (10 + 2) / 3;", 4),
        // Return stops execution
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("return 1; return 2;", 1),
        // Return with other statements
        ("9; return 10;", 10),
        ("9; return 2 * 5; 9;", 10),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn return_statements_with_conditionals() {
    let tests = vec![
        // Return in if block
        ("if (true) { return 10; };", 10),
        ("if (false) { return 10; } else { return 20; };", 20),
        ("if (1 > 0) { return 42; };", 42),
        // Return stops execution even in nested blocks
        ("if (true) { return 10; 9; };", 10),
        ("if (true) { 5; return 20; 9; };", 20),
        // Return with expressions in conditionals
        ("if (5 > 3) { return 2 * 5; };", 10),
        ("if (3 > 5) { return 1; } else { return 2; };", 2),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, expected);
    }
}

#[test]
fn return_statements_with_boolean_values() {
    let tests = vec![
        ("return true;", true),
        ("return false;", false),
        ("return 1 > 2;", false),
        ("return 2 > 1;", true),
        ("return true == false;", false),
        ("return true != false;", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(&evaluated, expected);
    }
}

#[test]
fn return_null_value() {
    let input = "return null;";
    let evaluated = test_eval(input);
    test_null_object(&evaluated);
}

// Helper functions

#[cfg(test)]
fn test_eval(input: &str) -> Box<dyn Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new();
    let mut env = Environment::new();

    evaluator
        .eval(&program, &mut env)
        .expect("Evaluation failed")
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

#[cfg(test)]
fn test_array_object(obj: &Box<dyn Object>, expected: &str) {
    if let Some(array) = obj.as_any().downcast_ref::<Array>() {
        assert_eq!(array.inspect(), expected, "Array value mismatch");
    } else {
        panic!("Expected Array object, got different type");
    }
}

#[test]
fn factorial_function() {
    let input = r#"
        def factorial(n) {
            if (n <= 1) {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
        factorial(5);
    "#;

    let evaluated = test_eval(input);
    test_integer_object(&evaluated, 120);
}

#[test]
fn fibonacci_function() {
    let input = r#"
        def fibonacci(n) {
            if (n <= 1) {
                return n;
            } else {
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
        }
        fibonacci(8);
    "#;

    let evaluated = test_eval(input);
    test_integer_object(&evaluated, 21);
}

#[test]
fn array_literal_evaluation() {
    let tests = vec![
        // Empty array
        ("[];", "[]"),
        // Single element arrays
        ("[1];", "[1]"),
        ("[true];", "[true]"),
        ("[null];", "[null]"),
        // Multiple element arrays
        ("[1, 2, 3];", "[1, 2, 3]"),
        ("[1, true, null];", "[1, true, null]"),
        // Arrays with expressions
        ("[1 + 1, 2 * 3, 4 / 2];", "[2, 6, 2]"),
        ("[-1, -2, -3];", "[-1, -2, -3]"),
        ("[1 > 2, 2 > 1];", "[false, true]"),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_array_object(&evaluated, expected);
    }
}
