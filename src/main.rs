mod lexer;
mod token;

use std::io;

use lexer::Lexer;
use token::TokenType;

fn main() {
    println!("Welcome to the Monkey programming language!");
    println!("Press Ctrl+D to exit");
    loop {
        let mut input_string = String::new();

        match io::stdin().read_line(&mut input_string) {
            Ok(0) => break, // EOF (Ctrl+D)
            Ok(_) => {
                let mut lexer = Lexer::new(&input_string);

                loop {
                    let token = lexer.next_token();
                    dbg!("{:?}", &token);
                    if token.token_type == TokenType::Illegal {
                        println!("Illegal token found: {}", token.literal);
                        break;
                    } else if token.token_type == TokenType::Eof {
                        break;
                    } else {
                        println!("{}", token.literal);
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
