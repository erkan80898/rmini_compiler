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
                if matches!(token, Token::ADD)
                    || matches!(token, Token::SUB)
                    || matches!(token, Token::EXP)
                {
                    let right_bp = Parser::prefix_binding_power(&token);
                    let rhs = self.parse_rec(right_bp);
                    SExpr::Cons(token, vec![rhs])
                } else if matches!(token, Token::LPAREN) {
                    let sub = self.parse_rec(0);
                    assert!(matches!(self.lexer.next(), Token::RPAREN));
                    sub
                } else {
                    SExpr::Atom(token)
                }
            }
        };

        loop {
            let op = self.lexer.peek().clone();

            if let Some(left_bp) = Parser::postfix_binding_power(&op) {
                let rhs = self.parse_rec(left_bp);
                SExpr::Cons(op, vec![rhs]);
                continue;
            }

            if let Some((left_bp, right_bp)) = Parser::infix_binding_power(&op) {
                if left_bp < min_bp {
                    break;
                }

                self.lexer.next();
                let rhs = self.parse_rec(right_bp);

                lhs = SExpr::Cons(op, vec![lhs, rhs]);
                continue;
            }

            // breaks if the operator isn't defined for any positional operations
            // ie for ), }, EOF
            break;
        }

        lhs
    }

    fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
        match token {
            Token::ADD => Some((1, 2)),
            Token::SUB => Some((1, 2)),
            Token::MUL => Some((3, 4)),
            Token::DIV => Some((3, 4)),
            Token::EXP => Some((5, 6)),
            Token::SEMI => Some((0, 0)),
            _ => None,
        }
    }

    fn prefix_binding_power(token: &Token) -> u8 {
        match token {
            Token::ADD => 7,
            Token::SUB => 7,
            Token::EXP => 7,
            _ => panic!("GIVEN A WRONG TOKEN FOR PREFIX: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<u8> {
        match token {
            _ => None,
        }
    }
}
