use std::fs::File;
use std::io::Read;

mod cli;
mod lexer;
mod types;

use cli::Cli;
use lexer::Lexer;

fn main() {
    let args = Cli::new();

    let file = File::open(&args.path).expect("Could not open file");

    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Could not read file");

    let mut lexer = Lexer::new(&contents);

    println!("{}", contents);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        match token.token {
            lexer::types::Tokens::Eof => break,
            _ => (),
        }
    }
}
