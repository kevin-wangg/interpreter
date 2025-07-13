mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

use std::io;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::{Parser, has_parser_errors};

fn main() {
    println!("Welcome to the Monkey programming language!");
    println!("Press Ctrl+D to exit");
    loop {
        let mut input_string = String::new();
        match io::stdin().read_line(&mut input_string) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!("Exiting... Bye Bye!");
                break;
            }
            Ok(_) => {
                let lexer = Lexer::new(&input_string);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !has_parser_errors(&parser) {
                    println!("Printing AST...");
                    for statement in &program.statements {
                        println!("{}", statement.string());
                    }
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval(&Box::new(program)) {
                        Ok(value) => {
                            println!("Evaluated value:");
                            println!("{}", value.inspect());
                        }
                        Err(e) => {
                            println!("Error evaluating program: {}", e.error_message)
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}
