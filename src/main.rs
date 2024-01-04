use std::fs::File;
use std::io::Read;

mod cli;
mod lexer;
use lexer::Lexer;
use cli::Cli;

fn main() {
    let args = Cli::new();

    let file = File::open(&args.path)
        .expect("Could not open file");

    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)
        .expect("Could not read file");

    println!("{}", contents);

    let mut lexer = Lexer::new(&contents);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
    }

}
