use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
#[derive(Debug, Clone)]
pub enum Token {
    IF,
    ELSE,
    WHILE,
    SEMI,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LCBRACE,
    RCBRACE,
    RETURNSIG,
    RETURN,
    ADD,
    SUB,
    MUL,
    DIV,
    EXP,
    ASSIGN,
    NOT,
    NEQ,
    EQ,
    LESS,
    LEQ,
    GREATER,
    GEQ,
    LET,
    IDENT(String),
    STR(String),
    INT(i32),
    FLOAT(f32),
}

#[derive(Debug)]
pub struct Lexer {
    token_list: VecDeque<Token>,
}

impl Lexer {
    pub fn scan(file_name: String) -> Result<Lexer, Error> {
        let buf = std::fs::read(file_name)?;
        let mut token_list = VecDeque::new();
        let mut ln_num = 0;

        let s: Vec<char> = String::from_utf8(buf)
            .expect("from_utf8 failed")
            .chars()
            .collect();

        let mut peek;
        let mut current;
        let mut i = 0;
        let mut j = 0;
        ln_num += 1;
        let file_bound = s.len() - 1;
        while i < file_bound {
            current = &s[i];
            j = i + 1;
            peek = s[j];
            if current == &' ' || current == &'\n' {
                i += 1;
                continue;
            }

            match current {
                '+' => {
                    token_list.push_back(Token::ADD);
                    i += 1;
                }

                '-' => {
                    if peek == '>' {
                        token_list.push_back(Token::RETURNSIG);
                        i += 2;
                    } else {
                        token_list.push_back(Token::SUB);
                        i += 1;
                    }
                }

                '*' => {
                    if peek == '*' {
                        token_list.push_back(Token::EXP);
                        i += 2;
                    } else {
                        token_list.push_back(Token::MUL);
                        i += 1;
                    }
                }
                '/' => {
                    token_list.push_back(Token::DIV);
                    i += 1;
                }

                '=' => {
                    if peek == '=' {
                        token_list.push_back(Token::EQ);
                        i += 2;
                    } else {
                        token_list.push_back(Token::ASSIGN);
                        i += 1;
                    }
                }

                '!' => {
                    if peek == '=' {
                        token_list.push_back(Token::NEQ);
                        i += 2;
                    } else {
                        token_list.push_back(Token::NOT);
                        i += 1;
                    }
                }

                '<' => {
                    if peek == '=' {
                        token_list.push_back(Token::LEQ);
                        i += 2;
                    } else {
                        token_list.push_back(Token::LESS);
                        i += 1;
                    }
                }

                '>' => {
                    if peek == '=' {
                        token_list.push_back(Token::GEQ);
                        i += 2;
                    } else {
                        token_list.push_back(Token::GREATER);
                        i += 1;
                    }
                }

                '"' => {
                    while peek != '"' {
                        j += 1;
                        peek = s[j];
                    }
                    if j - i == 1 {
                        token_list.push_back(Token::STR(String::new()));
                    } else {
                        let word = s[i + 1..j].iter().collect::<String>();
                        token_list.push_back(Token::STR(word));
                    }
                    i = j + 1;
                }

                '(' => {
                    token_list.push_back(Token::LPAREN);
                    i += 1;
                }

                ')' => {
                    token_list.push_back(Token::RPAREN);
                    i += 1;
                }

                '[' => {
                    token_list.push_back(Token::LBRACE);
                    i += 1;
                }

                ']' => {
                    token_list.push_back(Token::RBRACE);
                    i += 1;
                }

                '{' => {
                    token_list.push_back(Token::LCBRACE);
                    i += 1;
                }

                '}' => {
                    token_list.push_back(Token::RCBRACE);
                    i += 1;
                }

                ';' => {
                    token_list.push_back(Token::SEMI);
                    i += 1;
                }

                _ => {
                    if current.is_digit(10) {
                        let mut float = false;
                        while peek.is_digit(10) || (peek == '.' && float == false) {
                            if peek == '.' {
                                float = true;
                            }
                            j += 1;
                            peek = s[j];
                        }
                        if peek != ' '
                            && peek != ';'
                            && peek != ')'
                            && peek != ']'
                            && j != file_bound
                        {
                            return Err(std::io::Error::new(
                                ErrorKind::InvalidInput,
                                format!("INVALID TOKEN AT: Ln {}, Col {}", ln_num, j),
                            ));
                        }

                        let num = s[i..j].iter().collect::<String>();

                        if float {
                            token_list.push_back(Token::FLOAT(num.parse::<f32>().unwrap()));
                        } else {
                            token_list.push_back(Token::INT(num.parse::<i32>().unwrap()));
                        }

                        i = j;
                    } else if current.is_alphabetic() || current == &'_' {
                        while (peek.is_alphanumeric() || peek == '_') && j != file_bound {
                            j += 1;
                            peek = s[j];
                        }

                        let word = &s[i..j];

                        if word == ['w', 'h', 'i', 'l', 'e'] {
                            token_list.push_back(Token::WHILE);
                        } else if word == ['i', 'f'] {
                            token_list.push_back(Token::IF);
                        } else if word == ['e', 'l', 's', 'e'] {
                            token_list.push_back(Token::ELSE)
                        } else if word == ['r', 'e', 't', 'u', 'r', 'n'] {
                            token_list.push_back(Token::RETURN)
                        } else if word == ['l', 'e', 't'] {
                            token_list.push_back(Token::LET);
                        } else {
                            token_list.push_back(Token::IDENT(word.iter().collect()));
                        }
                        i = j;
                    } else {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("INVALID TOKEN AT: Ln {}, Col {}", ln_num, j),
                        ));
                    }
                }
            }
        }
        Ok(Self { token_list })
    }

    pub fn next(&mut self) -> Option<Token> {
        self.token_list.pop_front()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.token_list.front()
    }
}
