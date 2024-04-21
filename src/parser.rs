use crate::ast::tree::*;
use crate::scanner::token::Token;
use crate::scanner::token::TokenType;

pub struct ParserErr {
    line: usize,
    where_err: String,
    message: String,
}
impl ParserErr {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            line: token.line,
            where_err: match token.token_type {
                TokenType::Eof => String::from(" at end"),
                _ => format!(" at '{}'", token.lexeme.as_deref().unwrap_or("_")),
            },
            message: message
        }
    }
}
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_eof() && self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        self.pos += 1;
        self.previous()
    }

    fn is_eof(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParserErr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParserErr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParserErr> {
        let mut expr = self.comparison()?;

        while self.check(TokenType::BangEqual) || self.check(TokenType::EqualEqual) {
            let operator = self.advance();

            expr = Box::new(Expr::Binary(Binary::new(
                expr,
                Token {
                    token_type: operator.token_type.clone(),
                    lexeme: operator.lexeme.clone(),
                    line: operator.line,
                },
                self.comparison()?,
            )));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParserErr> {
        
        let mut expr = self.term()?;
        while self.check(TokenType::Greater)
            || self.check(TokenType::GreaterEqual)
            || self.check(TokenType::Less)
            || self.check(TokenType::LessEqual)
        {
            let operator = self.advance();

            expr = Box::new(Expr::Binary(Binary::new(
                expr,
                Token {
                    token_type: operator.token_type.clone(),
                    lexeme: operator.lexeme.clone(),
                    line: operator.line,
                },
                self.term()?,
            )));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Box<Expr>, ParserErr> {
        let mut expr = self.factor()?;
        while self.check(TokenType::Minus) || self.check(TokenType::Plus) {
            let operator = self.advance();

            expr = Box::new(Expr::Binary(Binary::new(
                expr,
                Token {
                    token_type: operator.token_type.clone(),
                    lexeme: operator.lexeme.clone(),
                    line: operator.line,
                },
                self.factor()?,
            )));
        }
        return Ok(expr);
    }
    fn factor(&mut self) -> Result<Box<Expr>, ParserErr> {
        
        let mut expr = self.unary()?;
        while self.check(TokenType::Slash) || self.check(TokenType::Star) {
            let operator = self.advance();

            expr = Box::new(Expr::Binary(Binary::new(
                expr,
                Token {
                    token_type: operator.token_type.clone(),
                    lexeme: operator.lexeme.clone(),
                    line: operator.line,
                },
                self.unary()?,
            )));
        }
        return Ok(expr);
    }
    fn unary(&mut self) -> Result<Box<Expr>, ParserErr> {
        if self.check(TokenType::Bang) || self.check(TokenType::Minus) {
            let operator = self.advance();

            return Ok(Box::new(Expr::Unary(Unary::new(
                Token {
                    token_type: operator.token_type.clone(),
                    lexeme: operator.lexeme.clone(),
                    line: operator.line,
                },
                self.unary()?,
            ))));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParserErr> {
        let result = match self.peek().token_type {
            TokenType::False => Some(Expr::Literal(Literal::Boolean(false))),
            TokenType::True => Some(Expr::Literal(Literal::Boolean(true))),
            TokenType::Nil => Some(Expr::Literal(Literal::Nil)),
            TokenType::Number(val) => Some(Expr::Literal(Literal::Number(val))),
            TokenType::StringLiteral(ref val) => Some(Expr::Literal(Literal::String(val.to_string()))),
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                if self.check(TokenType::RightParen) {
                    self.advance();
                    Ok(())
                } else {
                    Err(ParserErr::new(self.peek(), String::from("Expect expression.")))
                }?;
                return Ok(Box::new(Expr::Grouping(Grouping::new(expr))));
            }
            _ => None
        };

        if let Some(val) = result {
            self.advance();
            return Ok(Box::new(val))
        }
        
        Err(ParserErr::new(self.peek(), String::from("Expect expression.")))
    }
}
