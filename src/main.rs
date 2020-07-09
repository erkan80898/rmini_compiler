mod lexer;
mod parser;

fn main() {
    let tokens = lexer::Lexer::scan(String::from("input"));
    let parser = match tokens {
        Ok(tokens) => {
            println!("{:?}", tokens);
            parser::Parser::new(tokens)
        }
        Err(err) => panic!("Issue lexing: {}", err),
    };

    let s_exp = parser.parse();

    println!("{}", s_exp);
}
