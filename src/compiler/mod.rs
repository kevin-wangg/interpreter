use std::rc::Rc;

use crate::ast::{ExpressionStatement, InfixExpression, IntegerLiteral, Node, Program};
use crate::code::{OpCode, make_instruction};
use crate::object::{Integer, Object};

struct Compiler {
    instructions: Vec<u8>,
    constants: Vec<Rc<dyn Object>>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn compile<T: Node + ?Sized>(&mut self, node: &T) -> Result<(), CompilerError> {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            for statement in &program.statements {
                self.compile(statement.as_ref())?;
            }
        } else if let Some(expression_statement) =
            node.as_any().downcast_ref::<ExpressionStatement>()
        {
            self.compile(expression_statement.expression.as_ref())?;
        } else if let Some(infix_expression) = node.as_any().downcast_ref::<InfixExpression>() {
            self.compile(infix_expression.left.as_ref())?;
            self.compile(infix_expression.right.as_ref())?;
        } else if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            let value = integer_literal.value;
            let index = self.add_constant(Rc::new(Integer::new(value)));
            let instruction = make_instruction(OpCode::OpConstant, vec![index]);
            self.instructions.extend_from_slice(&instruction);
        }

        Ok(())
    }

    fn add_constant(&mut self, object: Rc<dyn Object>) -> u32 {
        self.constants.push(Rc::clone(&object));
        (self.constants.len() - 1) as u32
    }
}

#[derive(Debug)]
struct CompilerError {
    error_message: String,
}

impl CompilerError {
    fn new(error_message: String) -> Self {
        Self { error_message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Program;
    use crate::code::{OpCode, make_instruction};
    use crate::lexer::Lexer;
    use crate::object::Integer;
    use crate::parser::Parser;

    struct CompilerTestCase {
        input: String,
        expected_consts: Vec<Rc<dyn Object>>,
        expected_instructions: Vec<Vec<u8>>,
    }

    #[test]
    fn integer_arithmetric() {
        let tests = vec![CompilerTestCase {
            input: "1 + 2;".to_string(),
            expected_consts: vec![Rc::new(Integer::new(1)), Rc::new(Integer::new(2))],
            expected_instructions: vec![
                make_instruction(OpCode::OpConstant, vec![0]),
                make_instruction(OpCode::OpConstant, vec![1]),
            ],
        }];
        run_compiler_tests(tests);
    }

    fn run_compiler_tests(tests: Vec<CompilerTestCase>) {
        for t in tests {
            let program = parse(t.input);
            let mut compiler = Compiler::new();
            compiler.compile(&program).expect("Compilation failed");

            test_instructions(t.expected_instructions, compiler.instructions);
            test_constants(t.expected_consts, compiler.constants);
        }
    }

    fn test_instructions(expected_instructions: Vec<Vec<u8>>, instructions: Vec<u8>) {
        let expected_instructions: Vec<u8> = expected_instructions.into_iter().flatten().collect();
        assert_eq!(
            expected_instructions.len(),
            instructions.len(),
            "Expected instructions and instructions have differing lengths: {} != {}",
            expected_instructions.len(),
            instructions.len()
        );
        for (i, ins) in expected_instructions.into_iter().enumerate() {
            assert_eq!(
                instructions[i], ins,
                "Wrong instruction found at position {}. Wanted {}, got {}",
                i, ins, instructions[i]
            );
        }
    }

    fn test_constants(expected_constants: Vec<Rc<dyn Object>>, constants: Vec<Rc<dyn Object>>) {
        assert_eq!(
            expected_constants.len(),
            constants.len(),
            "Expected constants and constants: {} != {}",
            expected_constants.len(),
            constants.len()
        );
        for (i, c) in expected_constants.into_iter().enumerate() {
            if let Some(integer) = c.as_any().downcast_ref::<Integer>() {
                test_integer_object(integer.value, constants[i].clone());
            }
        }
    }

    fn test_integer_object(expected: i64, actual: Rc<dyn Object>) {
        let actual = actual
            .as_any()
            .downcast_ref::<Integer>()
            .expect("Expected an integer object");
        assert_eq!(
            expected, actual.value,
            "object has the wrong value. Wanted {}, got {}",
            expected, actual.value
        );
    }

    fn parse(input: String) -> Program {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }
}
