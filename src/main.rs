mod lexer;
mod token;

use std::io;

use lexer::Lexer;
use token::TokenType;

fn main() {
    let mut input_string = String::new();

    println!("Enter some text:");

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let mut lexer = Lexer::new(&input_string);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.token_type == TokenType::Illegal {
            panic!("Illegal token found: {}", token.literal);
        } else if token.token_type == TokenType::Eof {
            break;
        } else {
            println!("{}", token.literal);
        }
    }
}
