pub mod environment;

mod tests;

use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{
    ArrayExpression, BlockStatement, BooleanLiteral, CallExpression, Expression,
    ExpressionStatement, FunctionLiteral, Identifier, IfExpression, IndexExpression,
    InfixExpression, IntegerLiteral, LetStatement, Node, NullLiteral, PrefixExpression, Program,
    ReturnStatement, Statement, StringExpression,
};
use crate::evaluator::environment::Environment;
use crate::object::{
    Array, Boolean, BuiltinFn, Function, Integer, Null, Object, ReturnValue, SelfRef, StringObject,
};

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
    builtin_fns: HashMap<String, Box<dyn Object>>,
    self_fn: Option<Box<Function>>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut builtin_fns: HashMap<String, Box<dyn Object>> = HashMap::new();

        // Define builtin functions here.
        // Monkey Lang supports the following builtin functions:
        // - len: Returns the length of an input array or string
        // - print: Prints the input argument to stdout
        // - println: Prints the input argument to stdout, then prints newline character
        // - push: Expects an array and an object. Returns a new array with the object appended to
        // the end
        builtin_fns.insert(
            "len".to_string(),
            Box::new(BuiltinFn::new(Rc::new(|args| {
                if args.len() != 1 {
                    Err(EvaluatorError::new("len expects exactly one argument"))
                } else if let Some(array_expression) = args[0].as_any().downcast_ref::<Array>() {
                    Ok(Box::new(Integer::new(array_expression.items.len() as i64)))
                } else {
                    Err(EvaluatorError::new(
                        "Builtin function len expects array argument",
                    ))
                }
            }))),
        );
        builtin_fns.insert(
            "print".to_string(),
            Box::new(BuiltinFn::new(Rc::new(|args| {
                if args.len() != 1 {
                    Err(EvaluatorError::new("print expects exactly one argument"))
                } else {
                    print!("{}", args[0].inspect());
                    Ok(Box::new(Null::new()))
                }
            }))),
        );
        builtin_fns.insert(
            "println".to_string(),
            Box::new(BuiltinFn::new(Rc::new(|args| {
                if args.len() != 1 {
                    Err(EvaluatorError::new("print expects exactly one argument"))
                } else {
                    println!("{}", args[0].inspect());
                    Ok(Box::new(Null::new()))
                }
            }))),
        );
        builtin_fns.insert(
            "push".to_string(),
            Box::new(BuiltinFn::new(Rc::new(|args| {
                if args.len() != 2 {
                    Err(EvaluatorError::new("push expects exactly two arguments"))
                } else if let Some(array_expression) = args[0].as_any().downcast_ref::<Array>() {
                    let mut new_array_items = array_expression.items.clone();
                    new_array_items.push(args[1].clone());
                    Ok(Box::new(Array::new(new_array_items)))
                } else {
                    Err(EvaluatorError::new(
                        "push expects the first argument to be an array",
                    ))
                }
            }))),
        );
        builtin_fns.insert(
            "tail".to_string(),
            Box::new(BuiltinFn::new(Rc::new(|args| {
                if args.len() != 1 {
                    Err(EvaluatorError::new("tail expects exactly one arguments"))
                } else if let Some(array_expression) = args[0].as_any().downcast_ref::<Array>() {
                    // Very inefficient since removing first element from Vec is O(n)
                    let mut new_array_items = array_expression.items.clone();
                    if new_array_items.is_empty() {
                        Err(EvaluatorError::new("tail expects on a non-empty list"))
                    } else {
                        new_array_items.remove(0);
                        Ok(Box::new(Array::new(new_array_items)))
                    }
                } else {
                    Err(EvaluatorError::new(
                        "tail expects the argument to be an array",
                    ))
                }
            }))),
        );
        Self {
            builtin_fns,
            self_fn: None,
        }
    }

    pub fn eval<T: Node + ?Sized>(
        &mut self,
        node: &T,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            self.eval_block_statement(&program.statements, env, true)
        } else if let Some(statement) = node.as_any().downcast_ref::<ExpressionStatement>() {
            self.eval(statement.expression.as_ref(), env)
        } else if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
            Ok(Box::new(Integer::new(integer_literal.value)))
        } else if let Some(boolean_literal) = node.as_any().downcast_ref::<BooleanLiteral>() {
            Ok(Box::new(Boolean::new(boolean_literal.value)))
        } else if node.as_any().is::<NullLiteral>() {
            Ok(Box::new(Null::new()))
        } else if let Some(function_literal) = node.as_any().downcast_ref::<FunctionLiteral>() {
            let function_env = Environment::new_wrapped(env);
            Ok(Box::new(Function::new(
                &function_literal.parameters,
                function_literal.body.clone(),
                function_env,
            )))
        } else if let Some(identifier) = node.as_any().downcast_ref::<Identifier>() {
            match env.get(&identifier.value) {
                Some(value) => Ok(value.clone()),
                None => match self.builtin_fns.get(&identifier.value) {
                    Some(value) => Ok(value.clone()),
                    None => Err(EvaluatorError::new(&format!(
                        "Unknown identifier found: {}",
                        identifier.value
                    ))),
                },
            }
        } else if let Some(index_expression) = node.as_any().downcast_ref::<IndexExpression>() {
            self.eval_index_expression(index_expression, env)
        } else if let Some(call_expression) = node.as_any().downcast_ref::<CallExpression>() {
            self.eval_call_expression(call_expression, env)
        } else if let Some(prefix_expression) = node.as_any().downcast_ref::<PrefixExpression>() {
            self.eval_prefix_expression(prefix_expression, env)
        } else if let Some(infix_expression) = node.as_any().downcast_ref::<InfixExpression>() {
            self.eval_infix_expression(infix_expression, env)
        } else if let Some(if_expression) = node.as_any().downcast_ref::<IfExpression>() {
            self.eval_if_expression(if_expression, env)
        } else if let Some(array_expression) = node.as_any().downcast_ref::<ArrayExpression>() {
            self.eval_array_expression(array_expression, env)
        } else if let Some(string_expression) = node.as_any().downcast_ref::<StringExpression>() {
            Ok(Box::new(StringObject::new(string_expression.value.clone())))
        } else if let Some(block_statement) = node.as_any().downcast_ref::<BlockStatement>() {
            let mut wrapped_env = Environment::new_wrapped(env);
            self.eval_block_statement(&block_statement.statements, &mut wrapped_env, false)
        } else if let Some(return_statement) = node.as_any().downcast_ref::<ReturnStatement>() {
            self.eval_return_statement(return_statement, env)
        } else if let Some(let_statement) = node.as_any().downcast_ref::<LetStatement>() {
            self.eval_let_statement(let_statement, env)
        } else {
            Err(EvaluatorError::new(
                "Evaluator encountered unknown AST type",
            ))
        }
    }

    fn eval_block_statement(
        &mut self,
        statements: &[Box<dyn Statement>],
        env: &mut Environment,
        unwrap_return_value: bool,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let mut ret: Box<dyn Object> = Box::new(Integer::new(69));
        for statement in statements {
            ret = self.eval(statement.as_ref(), env)?;
            if ret.as_any().is::<ReturnValue>() {
                if unwrap_return_value {
                    let ret: Box<dyn Any> = ret;
                    let return_value = ret
                        .downcast::<ReturnValue>()
                        .expect("Object should be ReturnValue");
                    let value = return_value.value;
                    return Ok(value);
                } else {
                    return Ok(ret);
                }
            }
        }
        Ok(ret)
    }

    fn eval_index_expression(
        &mut self,
        index_expression: &IndexExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let index = self.eval(index_expression.index.as_ref(), env)?;
        if let Some(index) = index.as_any().downcast_ref::<Integer>() {
            let collection = self.eval(index_expression.collection.as_ref(), env)?;
            if let Some(collection) = collection.as_any().downcast_ref::<Array>() {
                let index = index.value as usize;
                if index >= collection.items.len() {
                    Err(EvaluatorError::new(&format!(
                        "Out of bounds array access. Index is {} but array length is {}",
                        index,
                        collection.items.len()
                    )))
                } else {
                    Ok(collection.items[index].clone())
                }
            } else {
                Err(EvaluatorError::new(
                    "Expected collection to be an array when the index is an integer literal",
                ))
            }
        } else {
            Err(EvaluatorError::new(
                "Expected index to be an integer literal",
            ))
        }
    }

    fn eval_call_expression(
        &mut self,
        call_expression: &CallExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let mut arguments = Vec::new();
        // Evaluate the call expression arguments. If any of them return an error, immediately
        // return from this function with that error.
        for arg in &call_expression.arguments {
            match self.eval(arg.as_ref(), env) {
                Ok(arg) => arguments.push(arg),
                Err(e) => return Err(e),
            }
        }
        if let Some(function_literal) = call_expression
            .function
            .as_any()
            .downcast_ref::<FunctionLiteral>()
        {
            let function: Box<dyn Any> = self.eval(function_literal, env)?;
            if let Ok(function) = function.downcast::<Function>() {
                self.apply_function(function, arguments)
            } else {
                Err(EvaluatorError::new(
                    "Expected function literal to evaluate to function",
                ))
            }
        } else if let Some(identifier) = call_expression
            .function
            .as_any()
            .downcast_ref::<Identifier>()
        {
            if let Some(value) = env.get(&identifier.value) {
                let value: Box<dyn Any> = value.clone();
                if value.is::<SelfRef>() {
                    if let Some(self_fn) = self.self_fn.clone() {
                        self.apply_function(self_fn, arguments)
                    } else {
                        Err(EvaluatorError::new(
                            "Expected self_fn to be Some when evaluating recursive function",
                        ))
                    }
                } else if let Ok(function) = value.downcast::<Function>() {
                    let temp = self.self_fn.clone();
                    self.self_fn = Some(function.clone());
                    let ret = self.apply_function(function, arguments);
                    self.self_fn = temp;
                    ret
                } else {
                    Err(EvaluatorError::new(&format!(
                        "Expected function literal in call expression. {} is not a function literal",
                        identifier.value
                    )))
                }
            } else if let Some(value) = self.builtin_fns.get(&identifier.value) {
                // Check for builtin functions here
                let value: Box<dyn Any> = value.clone();
                if let Ok(builtin_fn) = value.downcast::<BuiltinFn>() {
                    let builtin_fn = builtin_fn.builtin_fn;
                    builtin_fn(arguments)
                } else {
                    Err(EvaluatorError::new(
                        "Unable to downcast to builtin function",
                    ))
                }
            } else {
                Err(EvaluatorError::new(&format!(
                    "Unknown identifier: {}",
                    identifier.value
                )))
            }
        } else {
            Err(EvaluatorError::new(
                "Expected function literal or identifier in call expression",
            ))
        }
    }

    fn apply_function(
        &mut self,
        mut function: Box<Function>,
        arguments: Vec<Box<dyn Object>>,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        if function.parameters.len() != arguments.len() {
            return Err(EvaluatorError::new(&format!(
                "Mismatched number of parameters in call expression: {} != {}",
                function.parameters.len(),
                arguments.len()
            )));
        }
        function
            .parameters
            .iter()
            .zip(arguments)
            .for_each(|(param, arg)| {
                function.env.insert(param, arg);
            });
        self.eval_block_statement(&function.body.statements, &mut function.env, true)
    }

    fn eval_prefix_expression(
        &mut self,
        prefix_expression: &PrefixExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        match prefix_expression.operator.as_ref() {
            "!" => self.eval_bang_expression(prefix_expression.right.as_ref(), env),
            "-" => self.eval_minus_expression(prefix_expression.right.as_ref(), env),
            _ => Err(EvaluatorError::new("Unknown operator in prefix expression")),
        }
    }

    fn eval_infix_expression(
        &mut self,
        infix_expression: &InfixExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        match infix_expression.operator.as_ref() {
            "+" | "-" | "*" | "/" | ">=" | "<=" | ">" | "<" => {
                self.eval_integer_infix_expression(infix_expression, env)
            }
            "==" | "!=" => self.eval_equality_infix_expression(infix_expression, env),
            _ => Err(EvaluatorError::new("Unknown operator in infix expression")),
        }
    }

    fn eval_integer_infix_expression(
        &mut self,
        infix_expression: &InfixExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(infix_expression.left.as_ref(), env)?;
        let right = self.eval(infix_expression.right.as_ref(), env)?;
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
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let left = self.eval(infix_expression.left.as_ref(), env)?;
        let right = self.eval(infix_expression.right.as_ref(), env)?;
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
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let expression = self.eval(return_statement.return_value.as_ref(), env)?;
        Ok(Box::new(ReturnValue::new(expression)))
    }

    fn eval_let_statement(
        &mut self,
        let_statement: &LetStatement,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        if let_statement.rec {
            env.insert(&let_statement.name, Box::new(SelfRef::new()));
            let value = self.eval(let_statement.value.as_ref(), env)?;
            env.insert(&let_statement.name, value);
            Ok(Box::new(Null::new()))
        } else {
            let value = self.eval(let_statement.value.as_ref(), env)?;
            env.insert(&let_statement.name, value);
            Ok(Box::new(Null::new()))
        }
    }

    fn eval_if_expression(
        &mut self,
        if_expression: &IfExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        for cons in &if_expression.consequences {
            let condition = self.eval(cons.0.as_ref(), env)?;
            if is_truthy(condition.as_ref()) {
                return self.eval(&cons.1, env);
            }
        }
        if let Some(alternative) = if_expression.alternative.as_ref() {
            self.eval(alternative, env)
        } else {
            // If the if_expression has no else branch and all the conditions are falsey, then it evaluates to null
            Ok(Box::new(Null::new()))
        }
    }

    fn eval_array_expression(
        &mut self,
        array_expression: &ArrayExpression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let mut items = Vec::new();
        for item in &array_expression.items {
            let item_object = self.eval(item.as_ref(), env)?;
            items.push(item_object);
        }
        Ok(Box::new(Array::new(items)))
    }

    fn eval_bang_expression(
        &mut self,
        right: &dyn Expression,
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right, env)?;
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
        env: &mut Environment,
    ) -> Result<Box<dyn Object>, EvaluatorError> {
        let right = self.eval(right, env)?;
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
