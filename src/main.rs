use std::{env, path::Path};

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(src_file) = args.get(1) {
        let path = Path::new(src_file);
        
        if path.exists() {
            let _lexer = lexer::Lexer::read(src_file.to_owned());
            let parse = parser::Parser::parse(_lexer.into_token_stream());
            println!("{}", parse);
            println!("Parser Finished")
        } else {
            println!("File {} does not exist", src_file);
        }
    } else {
        println!("Usage: {} <src>", args.get(0).unwrap_or(&String::from("./minic")));
    }
}