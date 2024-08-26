use bhasha::{Token, Parser, Interpreter};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    if !filename.ends_with(".bhasha") {
        eprintln!("Error: Invalid file format. Only .bhasha files are supported.");
        return;
    }

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let source = format!("{} samapt", source);
    
    // let lexer = Token::tokenize(&source);
    
    // for token in lexer {
    //     println!("{:?}", token);
    // }
    
    let mut lexer = Token::tokenize(&source);

    let mut parser = Parser::new(&mut lexer);

    let program = parser.parse();

    println!("{:?}", program);

    let mut interpreter = Interpreter::new();
    
    interpreter.run(program);
}