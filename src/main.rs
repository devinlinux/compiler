mod lexer;

use std::io::{ Write, BufRead, BufReader };
use std::fs::File;
use clap::{ Command, Arg, ArgAction };

use crate::lexer::{ Lexer, Token };


fn main() {
    let matches = Command::new("compiler")
        .about("An experimental compiler")
        .version("0.4.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Michael Bobrowski")

        .subcommand(
            Command::new("repl")
            .short_flag('R')
            .long_flag("repl")
            .about("Read Evaluate Print Loop")
        )
        .subcommand(
            Command::new("file")
            .short_flag('F')
            .long_flag("file")
            .about("Compile a file")
            .arg(
                Arg::new("path")
                .help("the path to the file to compiler")
                .required(true)
                .action(ArgAction::Set)
                .num_args(1)
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("repl", _)) => {
            repl().expect("Failed to run repl");
        },
        Some(("file", file_matches)) => {
            let path: &String = file_matches.get_one("path").expect("is present");
            compile_file(path.to_string()).expect("Failed to copmile file");
        },
        _ => unreachable!(),
    }
}

fn repl() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    
    loop {
        print!(">> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        stdin_lock.read_line(&mut input)?;
        
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

    Ok(())
}

fn compile_file(file: String) -> std::io::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut lexer = Lexer::new(String::from(line));
                loop {
                    let token = lexer.next();
                    println!("{}", token);
                    if let Token::Eof = token {
                        break;
                    }
                }
            },
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    Ok(())
}
