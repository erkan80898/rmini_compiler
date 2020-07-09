use super::lexer::*;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum SExpr {
    Atom(Token),
    Cons(Token, Vec<SExpr>),
}

impl Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(match self {
            SExpr::Atom(token) => {
                write!(f, "{:?}", token)?;
            }
            SExpr::Cons(token, cons) => {
                write!(f, "({:?} ", token)?;
                for s in cons {
                    write!(f, " {}", s)?;
                }
                write!(f, ")")?;
            }
        })
    }
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(mut self) -> SExpr {
        self.parse_rec(0)
    }

    fn parse_rec(&mut self, min_bp: u8) -> SExpr {
        let mut lhs = match self.lexer.next() {
            token => {
                if matches!(token, Token::INT(_)) || matches!(token, Token::FLOAT(_)) {
                    SExpr::Atom(token)
                } else if matches!(token, Token::ADD)
                    || matches!(token, Token::SUB)
                    || matches!(token, Token::EXP)
                {
                    let right_bp = Parser::prefix_binding_power(&token);
                    let rhs = self.parse_rec(right_bp);
                    SExpr::Cons(token, vec![rhs])
                } else {
                    panic!("BAD TOKEN: {:?}", token);
                }
            }
            _ => {
                panic!("NO TOKEN");
            }
        };

        loop {
            let op = match self.lexer.peek() {
                Token::ADD => Token::ADD,
                Token::SUB => Token::SUB,
                Token::MUL => Token::MUL,
                Token::DIV => Token::DIV,
                Token::EXP => Token::EXP,
                Token::EOF => break,
                _ => panic!("GIVEN A WRONG TOKEN FOR INFIX: {:?}", self.lexer.pos()),
            };

            let (left_bp, right_bp) = Parser::infix_binding_power(&op);

            if left_bp < min_bp {
                break;
            }

            self.lexer.next();
            let rhs = self.parse_rec(right_bp);

            lhs = SExpr::Cons(op, vec![lhs, rhs]);
        }

        lhs
    }

    fn infix_binding_power(token: &Token) -> (u8, u8) {
        match token {
            Token::ADD => (1, 2),
            Token::SUB => (1, 2),
            Token::MUL => (3, 4),
            Token::DIV => (3, 4),
            Token::EXP => (5, 6),
            _ => panic!("GIVEN A WRONG TOKEN FOR INFIX: {:?}", token),
        }
    }

    fn prefix_binding_power(token: &Token) -> u8 {
        match token {
            Token::ADD => 7,
            Token::SUB => 7,
            Token::EXP => 7,
            _ => panic!("GIVEN A WRONG TOKEN FOR INFIX: {:?}", token),
        }
    }
}
