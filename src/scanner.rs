use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum Token {
    IF,
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
    EXP,
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

pub fn scan(file_name: String) -> Result<Vec<Token>, Error> {
    let buf = std::fs::read(file_name)?;
    let mut token_list = Vec::new();
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
    while i < file_bound - 1 {
        current = &s[i];
        j = i + 1;
        peek = s[j];
        if current == &' ' || current == &'\n' {
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
                if peek == '*' {
                    token_list.push(Token::EXP);
                    i += 2;
                } else {
                    token_list.push(Token::MUL);
                    i += 1;
                }
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
                    let word = s[i + 1..j].iter().collect::<String>();
                    token_list.push(Token::STR(word));
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
                    if peek != ' ' && peek != ';' && peek != '{' && j != s.len() - 1 {
                        return Err(std::io::Error::new(
                            ErrorKind::InvalidInput,
                            format!("INVALID TOKEN AT: Ln {}, Col {}", ln_num, j),
                        ));
                    }

                    let num = s[i..j].iter().collect::<String>();

                    if float {
                        token_list.push(Token::FLOAT(num.parse::<f32>().unwrap()));
                    } else {
                        token_list.push(Token::INT(num.parse::<i32>().unwrap()));
                    }

                    i = j;
                } else if current.is_alphabetic() || current == &'_' {
                    if current == &'i' && peek == 'f' {
                        j += 1;
                        if j < s.len() - 1 {
                            peek = s[j];
                            if !peek.is_alphanumeric() {
                                token_list.push(Token::IF);
                                i = j;
                                continue;
                            }
                        } else {
                            i = j;
                            token_list.push(Token::IF);
                            continue;
                        }
                    } else if current == &'e' && peek == 'l' {
                        j += 1;
                        if j < file_bound {
                            peek = s[j];
                            if peek == 's' {
                                j += 1;
                                if j < file_bound {
                                    peek = s[j];
                                    if peek == 'e' {
                                        j += 1;
                                        if j < file_bound {
                                            peek = s[j];
                                            if !peek.is_alphanumeric() {
                                                token_list.push(Token::ELSE);
                                                i = j;
                                                continue;
                                            }
                                        } else {
                                            token_list.push(Token::ELSE);
                                        }
                                    }
                                } else {
                                    i = j;
                                    token_list.push(Token::IDENT(String::from("els")));
                                    continue;
                                }
                            }
                        }
                    } else if current == &'w' && peek == 'h' {
                        j += 1;
                        if j < file_bound {
                            peek = s[j];
                            if peek == 'i' {
                                j += 1;
                                if j < file_bound {
                                    peek = s[j];
                                    if peek == 'l' {
                                        j += 1;
                                        if j < file_bound {
                                            peek = s[j];
                                            if peek == 'e' {
                                                j += 1;
                                                if j < file_bound {
                                                    peek = s[j];
                                                    if !peek.is_alphanumeric() {
                                                        token_list.push(Token::WHILE);
                                                        i = j;
                                                        continue;
                                                    }
                                                } else {
                                                    token_list.push(Token::WHILE);
                                                    continue;
                                                }
                                            }
                                        } else {
                                            i = j;
                                            token_list.push(Token::IDENT(String::from("whil")));
                                            continue;
                                        }
                                    }
                                } else {
                                    i = j;
                                    token_list.push(Token::IDENT(String::from("whi")));
                                    continue;
                                }
                            }
                        } else {
                            i = j;
                            token_list.push(Token::IDENT(String::from("wh")));
                            continue;
                        }
                    }

                    while peek.is_alphanumeric() && j != file_bound {
                        j += 1;
                        peek = s[j];
                    }

                    let word = s[i..j].iter().collect::<String>();

                    token_list.push(Token::IDENT(word));
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

    Ok(token_list)
}
