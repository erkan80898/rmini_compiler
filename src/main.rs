mod lexer;
mod parser;

fn main() {
    let tokens = lexer::Lexer::scan(String::from("input"));
    match tokens {
        Ok(x) => println!("{:#?}", x),
        Err(err) => println!("{}", err),
    }
}
