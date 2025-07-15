mod environment;
mod tests;

use std::any::Any;

use crate::ast::{
    BlockStatement, BooleanLiteral, Expression, ExpressionStatement, FunctionLiteral, Identifier,
    IfExpression, InfixExpression, IntegerLiteral, LetStatement, Node, NullLiteral,
    PrefixExpression, Program, ReturnStatement,
};
use crate::evaluator::environment::Environment;
use crate::object::{Boolean, Integer, Null, Object, ReturnValue};

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

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn eval<T: Node + ?Sized>(&mut self, node: &T) -> Result<Box<dyn Object>, EvaluatorError> {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            let mut ret: Box<dyn Object> = Box::new(Integer::new(69));
            for statement in &program.statements {
                ret = self.eval(statement.as_ref())?;
                // If we encounter a ReturnValue, then return right away and don't evaluate the remaining statements.
                // I needed to convert ret to a Box<dyn Any> here in order to be able to downcast it to an owned value
                // of ReturnValue, which is needed for the return type to be a Box<dyn Object>. If there is a better
                // way to do this, please let me know :(
                if ret.as_any().is::<ReturnValue>() {
                    let ret: Box<dyn Any> = ret;
                    let return_value = ret
                        .downcast::<ReturnValue>()
                        .expect("Object should be ReturnValue");
                    let value = return_value.value;
                    return Ok(value);
                }
            }
            Ok(ret)
        } else if let Some(statement) = node.as_any().downcast_ref::<ExpressionStatement>() {
            self.eval(statement.expression.as_ref())
        } else if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            Ok(Box::new(Integer::new(integer_literal.value)))
        } else if let Some(boolean_literal) = node.as_any().downcast_ref::<BooleanLiteral>() {
            Ok(Box::new(Boolean::new(boolean_literal.value)))
        } else if node.as_any().is::<NullLiteral>() {
            Ok(Box::new(Null::new()))
        } else if let Some(identifier) = node.as_any().downcast_ref::<Identifier>() {
            match self.environment.get(&identifier.value) {
                Some(value) => panic!("this doesn't work yet"),
                None => Err(EvaluatorError::new(&format!(
                    "Unknown identifier found: {}",
                    identifier.value
                ))),
            }
        } else if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            self.eval_prefix_expression(prefix_expression)
        } else if let Some(infix_expression) = node.as_any().downcast_ref::<InfixExpression>() {
            self.eval_infix_expression(infix_expression)
        } else if let Some(if_expression) = node.as_any().downcast_ref::<IfExpression>() {
            self.eval_if_expression(if_expression)
        } else if let Some(block_statement) = node.as_any().downcast_ref::<BlockStatement>() {
            let mut ret: Box<dyn Object> = Box::new(Integer::new(69));
            for statement in &block_statement.statements {
                ret = self.eval(statement.as_ref())?;
                if ret.as_any().is::<ReturnValue>() {
                    break;
                }
            }
            Ok(ret)
        } else if let Some(return_statement) = node.as_any().downcast_ref::<ReturnStatement>() {
            self.eval_return_statement(return_statement)
        } else if let Some(let_statement) = node.as_any().downcast_ref::<LetStatement>() {
            self.eval_let_statement(let_statement)
        } else {
            Err(EvaluatorError::new(
                "Evaluator encountered unknown AST type",
            ))
        }
    }

    fn eval_prefix_expression(
        &mut self,
        prefix_expression: &PrefixExpression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        match prefix_expression.operator.as_ref() {
            "!" => self.eval_bang_expression(prefix_expression.right.as_ref()),
            "-" => self.eval_minus_expression(prefix_expression.right.as_ref()),
            _ => Err(EvaluatorError::new("Unknown operator in prefix expression")),
        }
    }

    fn eval_infix_expression(
        &mut self,
        infix_expression: &InfixExpression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        match infix_expression.operator.as_ref() {
            "+" | "-" | "*" | "/" | ">=" | "<=" | ">" | "<" => {
                self.eval_integer_infix_expression(infix_expression)
            }
            "==" | "!=" => self.eval_equality_infix_expression(infix_expression),
            _ => Err(EvaluatorError::new("Unknown operator in infix expression")),
        }
    }

    fn eval_integer_infix_expression(
        &mut self,
        infix_expression: &InfixExpression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(infix_expression.left.as_ref())?;
        let right = self.eval(infix_expression.right.as_ref())?;
        if let Some(left) = left.as_any().downcast_ref::<Integer>()
            && let Some(right) = right.as_any().downcast_ref::<Integer>()
        {
            match infix_expression.operator.as_ref() {
                "+" => Ok(Box::new(Integer::new(left.value + right.value))),
                "-" => Ok(Box::new(Integer::new(left.value - right.value))),
                "*" => Ok(Box::new(Integer::new(left.value * right.value))),
                "/" => {
                    if right.value == 0 {
                        Err(EvaluatorError::new("Division by zero"))
                    } else {
                        Ok(Box::new(Integer::new(left.value / right.value)))
                    }
                }
                ">" => Ok(Box::new(Boolean::new(left.value > right.value))),
                "<" => Ok(Box::new(Boolean::new(left.value < right.value))),
                ">=" => Ok(Box::new(Boolean::new(left.value >= right.value))),
                "<=" => Ok(Box::new(Boolean::new(left.value <= right.value))),
                _ => Err(EvaluatorError::new("Unknown integer infix operator")),
            }
        } else {
            Err(EvaluatorError::new(
                "Expected integer expressions in infix expression",
            ))
        }
    }

    // Note: It is valid in the Monkey language to compare two expressions of different types. Two expressions of different types are
    // always considered to be not equal.
    fn eval_equality_infix_expression(
        &mut self,
        infix_expression: &InfixExpression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(infix_expression.left.as_ref())?;
        let right = self.eval(infix_expression.right.as_ref())?;
        match infix_expression.operator.as_ref() {
            "==" => {
                if let (Some(left_int), Some(right_int)) = (
                    left.as_any().downcast_ref::<Integer>(),
                    right.as_any().downcast_ref::<Integer>(),
                ) {
                    Ok(Box::new(Boolean::new(left_int.value == right_int.value)))
                } else if let (Some(left_bool), Some(right_bool)) = (
                    left.as_any().downcast_ref::<Boolean>(),
                    right.as_any().downcast_ref::<Boolean>(),
                ) {
                    Ok(Box::new(Boolean::new(left_bool.value == right_bool.value)))
                } else if left.as_any().is::<Null>() && right.as_any().is::<Null>() {
                    Ok(Box::new(Boolean::new(true)))
                } else {
                    Ok(Box::new(Boolean::new(false)))
                }
            }
            "!=" => {
                if let (Some(left_int), Some(right_int)) = (
                    left.as_any().downcast_ref::<Integer>(),
                    right.as_any().downcast_ref::<Integer>(),
                ) {
                    Ok(Box::new(Boolean::new(left_int.value != right_int.value)))
                } else if let (Some(left_bool), Some(right_bool)) = (
                    left.as_any().downcast_ref::<Boolean>(),
                    right.as_any().downcast_ref::<Boolean>(),
                ) {
                    Ok(Box::new(Boolean::new(left_bool.value != right_bool.value)))
                } else if left.as_any().is::<Null>() && right.as_any().is::<Null>() {
                    Ok(Box::new(Boolean::new(false)))
                } else {
                    Ok(Box::new(Boolean::new(true)))
                }
            }
            _ => Err(EvaluatorError::new("Unknown boolean infix operator")),
        }
    }

    fn eval_return_statement(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let expression = self.eval(return_statement.return_value.as_ref())?;
        Ok(Box::new(ReturnValue::new(expression)))
    }

    fn eval_let_statement(
        &mut self,
        let_statement: &LetStatement,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let value = self.eval(let_statement.value.as_ref())?;
        let id = &let_statement.name.value;
        self.environment.insert(id, value);
        Ok(Box::new(Null::new()))
    }

    fn eval_if_expression(
        &mut self,
        if_expression: &IfExpression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let condition = self.eval(if_expression.condition.as_ref())?;
        if is_truthy(condition.as_ref()) {
            self.eval(&if_expression.consequence)
        } else if let Some(alternative) = if_expression.alternative.as_ref() {
            self.eval(alternative)
        } else {
            // If the if_expression has no else branch and the condition is falsey, then it evaluates to null
            Ok(Box::new(Null::new()))
        }
    }

    fn eval_bang_expression(
        &mut self,
        right: &dyn Expression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right)?;
        if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
            Ok(Box::new(Boolean::new(!boolean.value)))
        } else {
            Err(EvaluatorError::new(
                "Expected boolean expression after bang operator",
            ))
        }
    }

    fn eval_minus_expression(
        &mut self,
        right: &dyn Expression,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right)?;
        if let Some(integer) = right.as_any().downcast_ref::<Integer>() {
            Ok(Box::new(Integer::new(-integer.value)))
        } else {
            Err(EvaluatorError::new(
                "Expected integer expression after minus operator",
            ))
        }
    }
}

fn is_truthy(expression: &dyn Object) -> bool {
    if let Some(boolean) = expression.as_any().downcast_ref::<Boolean>() {
        boolean.value
    } else if let Some(integer) = expression.as_any().downcast_ref::<Integer>() {
        integer.value != 0
    } else {
        false
    }
}
