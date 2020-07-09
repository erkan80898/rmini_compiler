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
            Some(token) => {
                if matches!(token, Token::INT(_))
                    || matches!(token, Token::FLOAT(_))
                    || matches!(token, Token::IDENT(_))
                    || matches!(token, Token::STR(_))
                    || matches!(token, Token::VOID)
                    || matches!(token, Token::LBRACE)
                    || matches!(token, Token::LCBRACE)
                {
                    SExpr::Atom(token)
                } else if matches!(token, Token::ADD)
                    || matches!(token, Token::SUB)
                    || matches!(token, Token::EXP)
                    || matches!(token, Token::NOT)
                    || matches!(token, Token::FN)
                    || matches!(token, Token::IF)
                {
                    let right_bp = Parser::prefix_binding_power(&token);
                    let rhs;

                    if matches!(token, Token::FN) {
                        let id = self.lexer.next();
                        assert!(matches!(id, Some(Token::IDENT(_))));
                        assert!(matches!(self.lexer.peek(), Some(Token::LPAREN)));
                        rhs = self.parse_rec(right_bp);
                        return SExpr::Cons(token, vec![SExpr::Atom(id.unwrap()), rhs]);
                    } else {
                        rhs = self.parse_rec(right_bp);
                        return SExpr::Cons(token, vec![rhs]);
                    }
                } else if matches!(token, Token::LPAREN) {
                    let sub = self.parse_rec(0);
                    assert!(matches!(self.lexer.next(), Some(Token::RPAREN)));
                    sub
                } else if matches!(token, Token::LCBRACE) {
                    let sub = self.parse_rec(0);
                    assert!(matches!(self.lexer.next(), Some(Token::RCBRACE)));
                    sub
                } else {
                    println!("{:?}", token);
                    panic!("BAD TOKEN");
                }
            }
            None => panic!("BAD TOKEN"),
        };

        loop {
            let op = self.lexer.peek();

            if op.is_none() {
                break;
            }
            let op = op.unwrap().clone();

            if let Some(left_bp) = Parser::postfix_binding_power(&op) {
                self.lexer.next();
                let rhs = self.parse_rec(left_bp);
                if matches!(op, Token::LBRACE) {
                    assert!(matches!(self.lexer.next(), Some(Token::RBRACE)));
                } else if matches!(op, Token::LCBRACE) {
                    println!("{:?}", rhs);
                    assert!(matches!(self.lexer.next(), Some(Token::RCBRACE)));
                }
                lhs = SExpr::Cons(op, vec![lhs, rhs]);
                continue;
            }

            if let Some((left_bp, right_bp)) = Parser::infix_binding_power(&op) {
                if left_bp < min_bp {
                    break;
                }

                self.lexer.next();
                let rhs = self.parse_rec(right_bp);

                if matches!(op, Token::ASSIGN) {
                    assert!(
                        matches!(lhs, SExpr::Atom(Token::IDENT(_))),
                        "ERROR: Can't assign to non-identifiers"
                    );

                    assert!(
                        !matches!(rhs, SExpr::Atom(Token::LBRACE)),
                        "ERROR: Bad identifier assignment"
                    );
                }

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
            Token::ADD => Some((2, 3)),
            Token::SUB => Some((2, 3)),
            Token::MUL => Some((4, 5)),
            Token::DIV => Some((4, 5)),
            Token::EXP => Some((6, 7)),
            Token::ASSIGN => Some((1, 1)),
            Token::EQ => Some((1, 2)),
            Token::LEQ => Some((1, 2)),
            Token::GEQ => Some((1, 2)),
            Token::NEQ => Some((1, 2)),
            Token::LESS => Some((1, 2)),
            Token::GREATER => Some((1, 2)),
            Token::SEMI => Some((0, 0)),
            _ => None,
        }
    }

    fn prefix_binding_power(token: &Token) -> u8 {
        match token {
            Token::IF => 0,
            Token::ADD => 9,
            Token::SUB => 9,
            Token::EXP => 9,
            Token::NOT => 8,
            Token::FN => 10,
            _ => panic!("GIVEN A WRONG TOKEN FOR PREFIX: {:?}", token),
        }
    }

    fn postfix_binding_power(token: &Token) -> Option<u8> {
        match token {
            Token::LBRACE => Some(0),
            Token::LCBRACE => Some(0),
            _ => None,
        }
    }
}
