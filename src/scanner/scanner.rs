use super::token::{Token, TokenType};
use std::iter::from_fn;

pub struct Scanner {
    pub source: String,
}

pub struct ScanErr {
    pub line: usize,
    pub message: String,
}

impl Scanner {
    pub fn scan_tokens(self) -> Result<Vec<Token>, ScanErr> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut line: usize = 1;
        let mut char_indices = self.source.char_indices().peekable();

        while let Some((pos, c)) = char_indices.next() {
            let (token, lexeme): (TokenType, String) = match c {
                '(' => (TokenType::LeftParen, c.to_string()),
                ')' => (TokenType::RightParen, c.to_string()),
                '{' => (TokenType::LeftBrace, c.to_string()),
                '}' => (TokenType::RightBrace, c.to_string()),
                ',' => (TokenType::Comma, c.to_string()),
                '.' => (TokenType::Dot, c.to_string()),
                '-' => (TokenType::Minus, c.to_string()),
                '+' => (TokenType::Plus, c.to_string()),
                ';' => (TokenType::Semicolon, c.to_string()),
                '*' => (TokenType::Star, c.to_string()),
                '!' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => (TokenType::BangEqual, String::from("!=")),
                    None => (TokenType::Bang, c.to_string()),
                },
                '=' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => (TokenType::EqualEqual, String::from("==")),
                    None => (TokenType::Equal, c.to_string()),
                },
                '<' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => (TokenType::LessEqual, String::from("<=")),
                    None => (TokenType::Less, c.to_string()),
                },
                '>' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => (TokenType::GreaterEqual, String::from(">=")),
                    None => (TokenType::Greater, c.to_string()),
                },
                '/' => match char_indices.next_if_eq(&(pos + 1, '/')) {
                    Some(_) => {
                        let s: String = char_indices
                            .by_ref()
                            .take_while(|(_pos, c)| {
                                // Take until we reached a newline char
                                *c != '\n'
                            })
                            .map(|(_pos, c)| c)
                            .collect();
                        (TokenType::Comment(s.clone()), format!("//{}", s))
                    }
                    None => (TokenType::Slash, c.to_string()),
                },
                ' ' | '\r' | '\t' | '\n' => {
                    if c == '\n' {
                        line = line + 1;
                    }
                    (TokenType::Ignore, c.to_string())
                }
                '"' => {
                    let mut last_matched: char = '\0';
                    let s: String = char_indices
                        .by_ref()
                        .take_while(|(_pos, c)| {
                            last_matched = *c;
                            *c != '"'
                        })
                        .map(|(_pos, c)| c)
                        .collect();

                    match last_matched {
                        '"' => (TokenType::StringLiteral(s.clone()), format!("\"{}\"", s)),
                        _ => {
                            return Err(ScanErr {
                                line: line,
                                message: String::from("Unterminated string."),
                            })
                        }
                    }
                }
                x if x.is_digit(10) => {
                    let mut number_str: String = x.to_string();
                    let integral: String =
                        from_fn(|| char_indices.by_ref().next_if(|(_pos, c)| c.is_digit(10)))
                            .map(|(_pos, c)| c)
                            .collect();

                    // Extract the fractional part of the number
                    let fractional = match char_indices.by_ref().next_if(|(_pos, c)| (*c) == '.') {
                        Some(_) => {
                            from_fn(|| char_indices.by_ref().next_if(|(_pos, c)| c.is_digit(10)))
                                .map(|(_pos, c)| c)
                                .collect()
                        }
                        None => "0".to_owned(),
                    };

                    number_str.push_str(&integral);
                    number_str.push_str(".");
                    number_str.push_str(&fractional);
                    // FIXME: This is not correct as the fractional part are '.0' by default
                    (TokenType::Number(number_str.parse().unwrap()), number_str)
                }
                x if Self::is_alpha(x) => {
                    let mut identifier: String = x.to_string();
                    let rest: String = from_fn(|| {
                        char_indices
                            .by_ref()
                            .next_if(|(_pos, c)| Self::is_alpha_numeric(*c))
                    })
                    .map(|(_pos, c)| c)
                    .collect();
                    identifier.push_str(&rest);

                    match identifier.as_str() {
                        // Reserved keywords
                        "and" => (TokenType::And, identifier),
                        "class" => (TokenType::Class, identifier),
                        "else" => (TokenType::Else, identifier),
                        "false" => (TokenType::False, identifier),
                        "for" => (TokenType::For, identifier),
                        "fun" => (TokenType::Fun, identifier),
                        "if" => (TokenType::If, identifier),
                        "nil" => (TokenType::Nil, identifier),
                        "or" => (TokenType::Or, identifier),
                        "print" => (TokenType::Print, identifier),
                        "return" => (TokenType::Return, identifier),
                        "super" => (TokenType::Super, identifier),
                        "this" => (TokenType::This, identifier),
                        "true" => (TokenType::True, identifier),
                        "var" => (TokenType::Var, identifier),
                        "while" => (TokenType::While, identifier),
                        // Return an identifier if not reserved keyword
                        _ => (TokenType::Identifier(identifier.clone()), identifier),
                    }
                }
                _ => {
                    return Err(ScanErr {
                        line: line,
                        message: format!("Unexpected token: {}", c),
                    })
                }
            };

            if let TokenType::Ignore = token {
                continue;
            }

            let is_comment = if let TokenType::Comment(_) = token {
                true
            } else {
                false
            };

            tokens.push(Token {
                token_type: token,
                lexeme: Some(lexeme),
                line: line,
            });

            if is_comment {
                // The comment should be all on 1 line, so add 1 more line
                line = line + 1;
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: None,
            line: line,
        });

        Ok(tokens)
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || c.is_digit(10)
    }
}
