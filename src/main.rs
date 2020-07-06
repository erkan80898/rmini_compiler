use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Token {
    IF,
    THEN,
    ELSE,
    WHILE,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMI,
    RETURN,
    ADD,
    SUB,
    MUL,
    DIV,
    ASSIGN,
    NOT,
    NEQ,
    EQ,
    LESS,
    LEQ,
    GREATER,
    GEQ,
    IDENT(String),
    STR(String),
    INT(i32),
    FLOAT(f32),
}

fn scanner(file_name: String) -> Result<Vec<Token>, String> {
    let f = File::open(file_name);
    if f.is_err() {
        return Err(String::from("ERROR: File cannot be opened"));
    }

    let f = f.unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::<u8>::new();
    let mut token_list = Vec::new();
    let mut ln_num = 0;
    while reader
        .read_until(b'\n', &mut buf)
        .expect("read_until failed")
        != 0
    {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s: Vec<char> = String::from_utf8(buf)
            .expect("from_utf8 failed")
            .chars()
            .collect();

        let mut peek;
        let mut current;
        let mut i = 0;
        let mut j = 0;
        ln_num += 1;
        while i < s.len() - 1 {
            current = s[i];
            j = i + 1;
            peek = s[j];
            if current == ' ' {
                i += 1;
                continue;
            }

            //match symbols
            match current {
                '+' => {
                    token_list.push(Token::ADD);
                    i += 1;
                }

                '-' => {
                    if peek == '>' {
                        token_list.push(Token::RETURN);
                        i += 2;
                    } else {
                        token_list.push(Token::SUB);
                        i += 1;
                    }
                }

                '*' => {
                    token_list.push(Token::MUL);
                    i += 1;
                }

                '/' => {
                    token_list.push(Token::DIV);
                    i += 1;
                }

                '=' => {
                    if peek == '=' {
                        token_list.push(Token::EQ);
                        i += 2;
                    } else {
                        token_list.push(Token::ASSIGN);
                        i += 1;
                    }
                }

                '!' => {
                    if peek == '=' {
                        token_list.push(Token::NEQ);
                        i += 2;
                    } else {
                        token_list.push(Token::NOT);
                        i += 1;
                    }
                }

                '<' => {
                    if peek == '=' {
                        token_list.push(Token::LEQ);
                        i += 2;
                    } else {
                        token_list.push(Token::LESS);
                        i += 1;
                    }
                }

                '>' => {
                    if peek == '=' {
                        token_list.push(Token::GEQ);
                        i += 2;
                    } else {
                        token_list.push(Token::GREATER);
                        i += 1;
                    }
                }

                '"' => {
                    while peek != '"' {
                        j += 1;
                        peek = s[j];
                    }
                    if j - i == 1 {
                        token_list.push(Token::STR(String::new()));
                    } else {
                        let word = s[i..j].iter().collect::<String>();
                        token_list.push(Token::STR(
                           word
                        ));
                    }
                    i = j + 1;
                }

                '(' => {
                    token_list.push(Token::LPAREN);
                    i += 1;
                }

                ')' => {
                    token_list.push(Token::RPAREN);
                    i += 1;
                }

                '{' => {
                    token_list.push(Token::LBRACE);
                    i += 1;
                }

                '}' => {
                    token_list.push(Token::RBRACE);
                    i += 1;
                }

                ';' => {
                    token_list.push(Token::SEMI);
                    i += 1;
                }

                _ => {
                    //Numbers
                    if current.is_digit(10) {
                        let mut float = false;
                        while peek.is_digit(10) || (peek == '.' && float == false) {
                            if peek == '.' {
                                float = true;
                            }
                            j += 1;
                            peek = s[j];
                        }
                        if peek != ' ' && peek != ';' && j != s.len() - 1 {
                            return Err(format!("INVALID TOKEN AT: Ln {}, Col {}", ln_num, j));
                        }

                        let num = s[i..j].iter().collect::<String>();

                        if float {
                            token_list.push(Token::FLOAT(
                                num.parse::<f32>().unwrap(),
                            ));
                        } else {
                            token_list.push(Token::INT(
                                num.parse::<i32>().unwrap(),
                            ));
                        }

                        i = j;
                    } else if current.is_alphabetic() {
                        if current == 'i' {
                        } else if current == 't' {
                        } else if current == 'e' {
                        } else if current == 'w' {
                        } else {
                            while peek.is_alphanumeric() && j != s.len() - 1 {
                                j += 1;
                                peek = s[j];
                            }

                            let word  = s[i..j].iter().collect::<String>();

                            token_list.push(Token::IDENT(
                                word,
                            ));
                            i = j;
                        }
                    }
                }
            }
        }

        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_iter().collect::<String>().into_bytes();
        buf.clear();
    }

    Ok(token_list)
}

fn main() {
    let tokens = scanner(String::from("input"));
    match tokens {
        Ok(x) => {},
        Err(err) => println!("{}", err),
    }
}
