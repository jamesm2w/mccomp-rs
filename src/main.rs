mod lexer;
mod parser;

fn main() {
    let _lexer = lexer::Lexer::read(String::from(r#"C:\Users\james\rust-proj\minic\data\test.c"#));
    let parse = parser::Parser::parse(_lexer.into_token_stream());
    println!("{:?}", parse);
    println!("Parser Finished")
}