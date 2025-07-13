mod ast;
mod lexer;
mod parser;
mod token;

use std::io;

use lexer::Lexer;
use parser::{Parser, check_parser_errors};

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
                if let Some(program) = parser.parse_program() {
                    check_parser_errors(&parser);
                    for statement in program.statements {
                        println!("{}", statement.string());
                    }
                } else {
                    assert!(false, "Failed to parse program");
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}
