use crate::ast::{BooleanLiteral, Expression, ExpressionStatement, InfixExpression, IntegerLiteral, Node, NullLiteral, PrefixExpression, Program};
use crate::object::{Boolean, Integer, Null, Object};

mod tests;

#[derive(Debug)]
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

    pub fn eval<T: Node + ?Sized>(
        &mut self,
        node: &Box<T>,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
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
        } else if let Some(boolean_literal) = node.as_any().downcast_ref::<BooleanLiteral>() {
            Ok(Box::new(Boolean::new(boolean_literal.value)))
        } else if node.as_any().is::<NullLiteral>() {
            Ok(Box::new(Null::new()))
        } else if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            self.eval_prefix_expression(prefix_expression)
        } else if let Some(infix_expression) = node.as_any().downcast_ref::<InfixExpression>() {
            self.eval_infix_expression(infix_expression)
        } else {
            Err(EvaluatorError::new(
                "Evaluator encountered unknown AST type",
            ))
        }
    }

    fn eval_prefix_expression(&mut self, prefix_expression: &PrefixExpression) -> Result<Box<dyn Object>, EvaluatorError> {
        match prefix_expression.operator.as_ref() {
            "!" => {
                self.eval_bang_expression(&prefix_expression.right)
            }
            "-" => {
                self.eval_minus_expression(&prefix_expression.right)
            }
            _ => {
                Err(EvaluatorError::new("Unknown operator in prefix expression"))
            }
        }
    }

    fn eval_infix_expression(&mut self, infix_expression: &InfixExpression) -> Result<Box<dyn Object>, EvaluatorError> {
        match infix_expression.operator.as_ref() {
            "+" | "-" | "*" | "/" | ">=" | "<=" | ">" | "<" => {
                self.eval_integer_infix_expression(infix_expression)
            }
            "==" | "!=" => {
                self.eval_equality_infix_expression(infix_expression)
            }
            _ => {
                Err(EvaluatorError::new("Unknown operator in infix expression"))
            }
        }
    }

    fn eval_integer_infix_expression(&mut self, infix_expression: &InfixExpression) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(&infix_expression.left)?;
        let right = self.eval(&infix_expression.right)?;
        if let Some(left) = left.as_any().downcast_ref::<Integer>() && let Some(right) = right.as_any().downcast_ref::<Integer>() {
            match infix_expression.operator.as_ref() {
                "+" => {
                    Ok(Box::new(Integer::new(left.value + right.value)))
                }
                "-" => {
                    Ok(Box::new(Integer::new(left.value - right.value)))
                }
                "*" => {
                    Ok(Box::new(Integer::new(left.value * right.value)))
                }
                "/" => {
                    if right.value == 0 {
                        Err(EvaluatorError::new("Division by zero"))
                    } else {
                        Ok(Box::new(Integer::new(left.value / right.value)))
                    }
                }
                ">" => {
                    Ok(Box::new(Boolean::new(left.value > right.value)))
                }
                "<" => {
                    Ok(Box::new(Boolean::new(left.value < right.value)))
                }
                ">=" => {
                    Ok(Box::new(Boolean::new(left.value >= right.value)))
                }
                "<=" => {
                    Ok(Box::new(Boolean::new(left.value <= right.value)))
                }
                _ => {
                    Err(EvaluatorError::new("Unknown integer infix operator"))
                }
            }
        } else {
            Err(EvaluatorError::new("Expected integer expressions in infix expression"))
        }
    }

    // Note: It is valid in the Monkey language to compare two expressions of different types
    fn eval_equality_infix_expression(&mut self, infix_expression: &InfixExpression) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(&infix_expression.left)?;
        let right = self.eval(&infix_expression.right)?;
        match infix_expression.operator.as_ref() {
            "==" => {
                // Handle equality comparison for different types
                if let (Some(left_int), Some(right_int)) = (left.as_any().downcast_ref::<Integer>(), right.as_any().downcast_ref::<Integer>()) {
                    Ok(Box::new(Boolean::new(left_int.value == right_int.value)))
                } else if let (Some(left_bool), Some(right_bool)) = (left.as_any().downcast_ref::<Boolean>(), right.as_any().downcast_ref::<Boolean>()) {
                    Ok(Box::new(Boolean::new(left_bool.value == right_bool.value)))
                } else if left.as_any().is::<Null>() && right.as_any().is::<Null>() {
                    Ok(Box::new(Boolean::new(true)))
                } else {
                    Ok(Box::new(Boolean::new(false)))
                }
            }
            "!=" => {
                // Handle inequality comparison for different types
                if let (Some(left_int), Some(right_int)) = (left.as_any().downcast_ref::<Integer>(), right.as_any().downcast_ref::<Integer>()) {
                    Ok(Box::new(Boolean::new(left_int.value != right_int.value)))
                } else if let (Some(left_bool), Some(right_bool)) = (left.as_any().downcast_ref::<Boolean>(), right.as_any().downcast_ref::<Boolean>()) {
                    Ok(Box::new(Boolean::new(left_bool.value != right_bool.value)))
                } else if left.as_any().is::<Null>() && right.as_any().is::<Null>() {
                    Ok(Box::new(Boolean::new(false)))
                } else {
                    Ok(Box::new(Boolean::new(true)))
                }
            }
            _ => {
                Err(EvaluatorError::new("Unknown boolean infix operator"))
            }
        }
    }

    fn eval_bang_expression(&mut self, right: &Box<dyn Expression>) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right)?;
        if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
            Ok(Box::new(Boolean::new(!boolean.value)))
        } else {
            Err(EvaluatorError::new("Expected boolean expression after bang operator"))
        }
    }

    fn eval_minus_expression(&mut self, right: &Box<dyn Expression>) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right)?;
        if let Some(integer) = right.as_any().downcast_ref::<Integer>() {
            Ok(Box::new(Integer::new(-integer.value)))
        } else {
            Err(EvaluatorError::new("Expected integer expression after minus operator"))
        }
    }
}
