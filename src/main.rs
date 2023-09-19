mod lexer;

use std::io::{Write, BufRead};

use crate::lexer::{ Lexer, Token };

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin_lock.read_line(&mut input).unwrap();
        
        if input.trim() == "EXIT()" {
            break;
        }

        let mut lexer = Lexer::new(input);

        loop {
            let token = lexer.next();
            println!("{}", token);
            if let Token::Eof = token {
                break;
            }
        }
    }
}
