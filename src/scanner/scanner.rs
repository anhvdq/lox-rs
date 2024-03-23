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
            let token: TokenType = match c {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::Semicolon,
                '*' => TokenType::Star,
                '!' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => TokenType::BangEqual,
                    None => TokenType::Bang,
                },
                '=' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => TokenType::EqualEqual,
                    None => TokenType::Equal,
                },
                '<' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => TokenType::LessEqual,
                    None => TokenType::Less,
                },
                '>' => match char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_) => TokenType::GreaterEqual,
                    None => TokenType::Greater,
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
                        TokenType::Comment(s)
                    }
                    None => TokenType::Slash,
                },
                ' ' | '\r' | '\t' | '\n' => {
                    if c == '\n' {
                        line = line + 1;
                    }
                    TokenType::Ignore
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
                        '"' => TokenType::StringLiteral(s),
                        _ => TokenType::Invalid(String::from("Unterminated literal.")),
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

                    TokenType::Number(number_str.parse().unwrap())
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
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "for" => TokenType::For,
                        "fun" => TokenType::Fun,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While,
                        // Return an identifier if not reserved keyword
                        _ => TokenType::Identifier(identifier)
                    }
                }
                _ => TokenType::Invalid(format!("Unexpected token: {}", c)),
            };

            if let TokenType::Invalid(err_msg) = token {
                // Invalid token here
                return Err(ScanErr {
                    line: line,
                    message: err_msg
                });
            }
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
                lexeme: None,
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
