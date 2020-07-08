use super::lexer::*;

#[derive(Debug)]
enum S_Expr {
    Atom(Token),
    Cons(Token, Vec<S_Expr>),
}
#[derive(Debug)]
struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    fn parse(&mut self) -> S_Expr {
        let lhs = match self.lexer.next(){
            token => match token {
                Token::INT(_) => S_Expr::Atom(token),
                Token::FLOAT(_) => S_Expr::Atom(token),
                _ => {
                    panic!("BAD TOKEN: {:?}", token);
                }
            },
        };

        loop {
            let op = match self.lexer.peek(){
                To
            }
        }
    }

}
