use crate::{ast::tree::*, scanner::token::{Token, TokenType}};
use std::any::{Any, TypeId};

pub struct Interpreter;

fn is_equal(left: Box<dyn Any>, right: Box<dyn Any>) -> bool {
    if (&*left).type_id() == (&*right).type_id() {
        let type_id = (&*left).type_id();
        if type_id == TypeId::of::<f64>() {
            return left.downcast_ref::<f64>().unwrap() == right.downcast_ref::<f64>().unwrap();
        }
        if type_id == TypeId::of::<bool>() {
            return left.downcast_ref::<bool>().unwrap() == right.downcast_ref::<bool>().unwrap();
        }
        if type_id == TypeId::of::<String>() {
            return left.downcast_ref::<String>().unwrap().eq(right.downcast_ref::<String>().unwrap());
        }
    }
    false
}

impl AstVisitor<Result<Option<Box<dyn Any>>, String>, ()> for Interpreter {
    fn process(&mut self, expr: &Expr, context: Option<&()>) -> Result<Option<Box<dyn Any>>, String> {
        expr.accept(self, context)
    }

    fn visit_binary(&mut self, binary: &Binary, context: Option<&()>) -> Result<Option<Box<dyn Any>>, String> {
        let left = binary.left.accept(self, context)?;
        let right = binary.right.accept(self, context)?;

        match binary.operator.token_type {
            // Boolean operation
            TokenType::Greater => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() > right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::GreaterEqual => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() >= right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::Less => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() < right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::LessEqual => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() <= right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            
            TokenType::BangEqual => {
                return Ok(Some(Box::new(
                    !is_equal(left.unwrap(), right.unwrap())
                )))
            }
            TokenType::EqualEqual => {
                return Ok(Some(Box::new(
                    is_equal(left.unwrap(), right.unwrap())
                )))
            }

            // Arithmetic operation
            TokenType::Minus => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() - right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::Slash => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() / right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::Star => {
                return Ok(Some(Box::new(
                    left.unwrap().downcast_ref::<f64>().unwrap() * right.unwrap().downcast_ref::<f64>().unwrap(),
                )))
            }
            TokenType::Plus => {
                let left_val = left.unwrap();
                let right_val = right.unwrap();
                let left_type = (&*left_val).type_id();
                let right_type = (&*right_val).type_id();

                if (left_type == TypeId::of::<f64>())
                    && (right_type == TypeId::of::<f64>())
                {
                    return Ok(Some(Box::new(
                        left_val.downcast_ref::<f64>().unwrap() + left_val.downcast_ref::<f64>().unwrap(),
                    )));
                } else if (left_type == TypeId::of::<String>())
                    && (right_type == TypeId::of::<String>())
                {
                    let mut concat_str: String = left_val.downcast_ref::<String>().unwrap().to_owned();
                    concat_str.push_str(left_val.downcast_ref::<String>().unwrap());
                    return Ok(Some(Box::new(concat_str)));
                }
            }
            _ => (),
        }

        Ok(None)
    }
    fn visit_unary(&mut self, unary: &Unary, context: Option<&()>) -> Result<Option<Box<dyn Any>>, String> {
        let right = unary.right.accept(self, context)?;

        match unary.operator.token_type {
            TokenType::Minus => return Ok(Some(Box::new(-(right.unwrap().downcast_ref::<f64>().unwrap())))),
            TokenType::Bang => {
                let mut truthy = true;

                match right {
                    Some(val) => {
                        if let Some(bool_val) = val.downcast_ref::<bool>() {
                            truthy = bool_val.to_owned();
                        }
                    }
                    None => truthy = false,
                }

                return Ok(Some(Box::new(truthy)));
            }
            _ => (),
        }

        Ok(None)
    }
    fn visit_grouping(
        &mut self,
        grouping: &Grouping,
        context: Option<&()>,
    ) -> Result<Option<Box<dyn Any>>, String> {
        grouping.expression.accept(self, context)
    }
    fn visit_literal(&mut self, literal: &Literal, _: Option<&()>) -> Result<Option<Box<dyn Any>>, String> {
        match literal {
            Literal::Boolean(val) => Ok(Some(Box::new(val.to_owned()))),
            Literal::Number(val) => Ok(Some(Box::new(val.to_owned()))),
            Literal::String(val) => Ok(Some(Box::new(val.to_owned()))),
            Literal::Nil => Ok(None),
        }
    }
}
