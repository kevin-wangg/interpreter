mod ast;
mod code;
mod compiler;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use evaluator::{Evaluator, environment::Environment};
use lexer::Lexer;
use parser::{Parser, has_parser_errors};

fn execute_file(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error reading file '{filename}': {error}");
            process::exit(1);
        }
    };

    let lexer = Lexer::new(&contents);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if has_parser_errors(&parser) {
        eprintln!("Parser errors found in file '{filename}'");
        process::exit(1);
    }

    let mut env = Environment::new();
    let mut evaluator = Evaluator::new();
    match evaluator.eval(&program, &mut env) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Runtime error: {}", e.error_message);
            process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Welcome to the Monkey programming language!");
    println!("Press Ctrl+D to exit");
    let mut env = Environment::new();
    loop {
        let mut input_string = String::new();
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush output");
        match io::stdin().read_line(&mut input_string) {
            Ok(0) => {
                println!("Exiting... Bye Bye!");
                break;
            }
            Ok(_) => {
                let lexer = Lexer::new(&input_string);
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !has_parser_errors(&parser) {
                    let mut evaluator = Evaluator::new();
                    match evaluator.eval(&program, &mut env) {
                        Ok(value) => {
                            println!("{}", value.inspect());
                        }
                        Err(e) => {
                            println!("Error evaluating program: {}", e.error_message)
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {error}");
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        execute_file(&args[1]);
    } else {
        run_repl();
    }
}
