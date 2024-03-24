use std::fmt;

pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    StringLiteral(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Comment(String),
    // In case invalid token
    Invalid(String),
    Ignore,
    
}

impl TokenType {
    fn to_string(&self) -> String {
        let result: String;
        match self {
            TokenType::LeftParen => result = String::from("LeftParen"),
            TokenType::RightParen => result = String::from("RightParen"),
            TokenType::LeftBrace => result = String::from("LeftBrace"),
            TokenType::RightBrace => result = String::from("RightBrace"),
            TokenType::Comma => result = String::from("Comma"),
            TokenType::Dot => result = String::from("Dot"),
            TokenType::Minus => result = String::from("Minus"),
            TokenType::Plus => result = String::from("Plus"),
            TokenType::Semicolon => result = String::from("Semicolon"),
            TokenType::Slash => result = String::from("Slash"),
            TokenType::Star => result = String::from("Star"),
            TokenType::Bang => result = String::from("Bang"),
            TokenType::BangEqual => result = String::from("BangEqual"),
            TokenType::Equal => result = String::from("Equal"),
            TokenType::EqualEqual => result = String::from("EqualEqual"),
            TokenType::Greater => result = String::from("Greater"),
            TokenType::GreaterEqual => result = String::from("GreaterEqual"),
            TokenType::Less => result = String::from("Less"),
            TokenType::LessEqual => result = String::from("LessEqual"),
            TokenType::Identifier(val) => result = format!("Identifier({})", val),
            TokenType::StringLiteral(val) => result = format!("StringLiteral({})", val),
            TokenType::Number(val) => result = format!("Number({})", val),
            TokenType::And => result = String::from("And"),
            TokenType::Class => result = String::from("Class"),
            TokenType::Else => result = String::from("Else"),
            TokenType::False => result = String::from("False"),
            TokenType::Fun => result = String::from("Fun"),
            TokenType::For => result = String::from("For"),
            TokenType::If => result = String::from("If"),
            TokenType::Nil => result = String::from("Nil"),
            TokenType::Or => result = String::from("Or"),
            TokenType::Print => result = String::from("Print"),
            TokenType::Return => result = String::from("Return"),
            TokenType::Super => result = String::from("Super"),
            TokenType::This => result = String::from("This"),
            TokenType::True => result = String::from("True"),
            TokenType::Var => result = String::from("Var"),
            TokenType::While => result = String::from("While"),
            TokenType::Eof => result = String::from("Eof"),
            TokenType::Invalid(val) => result = format!("Invalid({})", val),
            TokenType::Comment(val) => result = format!("Comment({})", val),
            _ => result = String::from("Unknown"),
        };

        result
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = self.token_type.to_string();
        write!(f, "{} {}", self.lexeme.as_deref().unwrap_or("_"), type_name)
    }
}
