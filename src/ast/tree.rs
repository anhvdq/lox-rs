use crate::scanner::token::Token;

pub trait AstVisitor<R, C> {
    fn process(&mut self, expr: &Expr, context: Option<&C>) -> R;

    fn visit_binary(&mut self, binary: &Binary, context: Option<&C>) -> R;
    fn visit_unary(&mut self, unary: &Unary, context: Option<&C>) -> R;
    fn visit_grouping(&mut self, grouping: &Grouping, context: Option<&C>) -> R;
    fn visit_literal(&mut self, literal: &Literal, context: Option<&C>) -> R;
}

pub trait Walkable {
    fn accept<R, C>(&self, visitor: &mut dyn AstVisitor<R, C>, context: Option<&C>) -> R;
}

pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

impl Walkable for Expr {
    fn accept<R, C>(&self, visitor: &mut dyn AstVisitor<R, C>, context: Option<&C>) -> R {
        match self {
            Expr::Binary(val) => visitor.visit_binary(val, context),
            Expr::Unary(val) => visitor.visit_unary(val, context),
            Expr::Grouping(val) => visitor.visit_grouping(val, context),
            Expr::Literal(val) => visitor.visit_literal(val, context),
        }
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self {
            left: left,
            operator: operator,
            right: right,
        }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self {
            operator: operator,
            right: right,
        }
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Self {
            expression: expression,
        }
    }
}

pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct AstPrinter;
impl AstVisitor<String, ()> for AstPrinter {
    fn process(&mut self, expr: &Expr, context: Option<&()>) -> String {
        expr.accept(self, context)
    }

    fn visit_binary(&mut self, binary: &Binary, context: Option<&()>) -> String {
        print!("binary\n");
        let operator = binary.operator.lexeme.as_ref().unwrap();
        let left = binary.left.accept(self, context);
        let right = binary.right.accept(self, context);
        format!("({} {} {})", operator, left, right)
    }
    fn visit_unary(&mut self, unary: &Unary, context: Option<&()>) -> String {
        print!("unary\n");
        let operator = unary.operator.lexeme.as_ref().unwrap();
        let right = unary.right.accept(self, context);
        format!("({} {})", operator, right)
    }
    fn visit_grouping(&mut self, grouping: &Grouping, context: Option<&()>) -> String {
        print!("group\n");
        format!("(group {})", grouping.expression.accept(self, context))
    }
    fn visit_literal(&mut self, literal: &Literal, _: Option<&()>) -> String {
        print!("lit\n");
        match literal {
            Literal::Boolean(val) => val.to_string(),
            Literal::Number(val) => val.to_string(),
            Literal::String(val) => val.to_string(),
            Literal::Nil => String::from("nil"),
        }
    }
}

// #[test]
// fn test() {
//     let mut visitor = AstPrinter {};
//     let expression = Expr::Binary(Binary {
//         left: Box::new(Expr::Unary(Unary {
//             operator: Token {
//                 token_type: TokenType::Minus,
//                 lexeme: Option::Some(String::from("-")),
//                 line: 1,
//             },
//             right: Box::new(Expr::Literal(Literal::Number(123.0))),
//         })),
//         operator: Token {
//             token_type: TokenType::Star,
//             lexeme: Option::Some(String::from("*")),
//             line: 1,
//         },
//         right: Box::new(Expr::Grouping(Grouping {
//             expression: Box::new(Expr::Literal(Literal::Number(45.67))),
//         })),
//     });

//     let result = visitor.process(&expression, None);
//     print!("{}\n", result);
// }
