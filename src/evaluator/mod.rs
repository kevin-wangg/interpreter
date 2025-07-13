use crate::ast::{ExpressionStatement, IntegerLiteral, Node, Program};
use crate::object::{Integer, Object};

mod tests;

pub struct EvaluatorError {
    pub error_message: String,
}

impl EvaluatorError {
    fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval<T: Node + ?Sized>(&mut self, node: &Box<T>) -> Result<Box<dyn Object>, EvaluatorError> {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            let mut ret: Box<dyn Object> = Box::new(Integer::new(69));
            for statement in &program.statements {
                ret = self.eval(statement)?;
            }
            Ok(ret)
        } else if let Some(statement) = node.as_any().downcast_ref::<ExpressionStatement>() {
            self.eval(&statement.expression)
        } else if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            Ok(Box::new(Integer::new(integer_literal.value)))
        } else {
            Err(EvaluatorError::new(
                "Evaluator encountered unknown AST type",
            ))
        }
    }
}
