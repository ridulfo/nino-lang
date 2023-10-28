#![allow(dead_code)]

use std::{iter::Peekable, slice::Iter};

use crate::lexer::TokenKind;

#[derive(Debug, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    Function,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    name: String,
    type_: Type,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<FunctionParameter>,
    return_type: Type,
    expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperation {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub type_: Type,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i32),
    Float(f32),
    Bool(bool),

    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),

    BinaryOperation(BinaryOperation),
}

#[derive(Debug, PartialEq)]
pub enum Item {
    Declaration(Declaration),
    Expression(Expression),
}
pub fn parse_primary(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    match tokens.next() {
        Some(TokenKind::Identifier(name)) => Expression::Identifier(name.clone()),
        Some(TokenKind::Integer(value)) => Expression::Integer(*value),
        Some(TokenKind::Float(value)) => Expression::Float(*value),
        Some(TokenKind::Bool(value)) => Expression::Bool(*value),
        Some(TokenKind::LeftParen) => {
            unimplemented!()
        }
        _ => panic!(
            "\nExpected identifier, integer, float, or left paren, got {:?}\n",
            tokens.peek().unwrap()
        ),
    }
}
pub fn parse_unary(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    parse_primary(tokens)
}
pub fn parse_factor(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut expression = parse_unary(tokens);

    loop {
        match tokens.peek() {
            Some(&&TokenKind::Multiplication) | Some(&&TokenKind::Division) => {
                let operator = match tokens.next() {
                    Some(TokenKind::Multiplication) => BinaryOperator::Multiply,
                    Some(TokenKind::Division) => BinaryOperator::Divide,
                    _ => panic!("Expected multiply or divide"),
                };
                let right = parse_unary(tokens);
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    expression
}
pub fn parse_term(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut expression = parse_factor(tokens);

    loop {
        match tokens.peek() {
            Some(&&TokenKind::Addition) | Some(&&TokenKind::Subtraction) => {
                let operator = match tokens.next() {
                    Some(TokenKind::Addition) => BinaryOperator::Add,
                    Some(TokenKind::Subtraction) => BinaryOperator::Subtract,
                    _ => panic!("Expected add or subtract"),
                };
                let right = parse_factor(tokens);
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    expression
}
pub fn parse_comparison(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut expression = parse_term(tokens);

    loop {
        match tokens.peek() {
            Some(&&TokenKind::LessThan)
            | Some(&&TokenKind::LessEqualThan)
            | Some(&&TokenKind::GreaterThan)
            | Some(&&TokenKind::GreaterEqualThan) => {
                let operator = match tokens.next() {
                    Some(TokenKind::LessThan) => BinaryOperator::LessThan,
                    Some(TokenKind::LessEqualThan) => BinaryOperator::LessEqualThan,
                    Some(TokenKind::GreaterThan) => BinaryOperator::GreaterThan,
                    Some(TokenKind::GreaterEqualThan) => BinaryOperator::GreaterEqualThan,
                    _ => panic!("Expected comparison operator"),
                };
                let right = parse_term(tokens);
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    expression
}
pub fn parse_equality(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut expression = parse_comparison(tokens);

    loop {
        match tokens.peek() {
            Some(&&TokenKind::Equal) | Some(&&TokenKind::NotEqual) => {
                let operator = match tokens.next() {
                    Some(TokenKind::Equal) => BinaryOperator::Equal,
                    Some(TokenKind::NotEqual) => BinaryOperator::NotEqual,
                    _ => panic!("Expected equal or not equal"),
                };
                let right = parse_comparison(tokens);
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    expression
}

pub fn parse_expression(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    parse_equality(tokens)
}

pub fn parse_declaration(tokens: &mut Peekable<Iter<TokenKind>>) -> Declaration {
    let _ = match tokens.next() {
        Some(TokenKind::Let) => {}
        _ => panic!("Expected let"),
    };
    let name = match tokens.next() {
        Some(TokenKind::Identifier(name)) => name.clone(),
        _ => panic!("Expected identifier"),
    };
    match tokens.next() {
        Some(TokenKind::Colon) => {}
        _ => panic!("Expected colon"),
    };
    let type_ = match tokens.next() {
        Some(TokenKind::Type(type_)) => match type_.as_str() {
            "i32" => Type::Integer,
            "f32" => Type::Float,
            "bool" => Type::Boolean,
            _ => panic!("Unknown type"),
        },
        _ => panic!("Expected type"),
    };
    match tokens.next() {
        Some(TokenKind::Equal) => {}
        _ => panic!("Expected equal"),
    };
    let expression = parse_expression(tokens);
    match tokens.next() {
        Some(TokenKind::Semicolon) => {}
        _ => panic!("Expected semicolon"),
    };
    Declaration {
        name,
        type_,
        expression: Box::new(expression),
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, TokenKind>>,
}

impl<'a> Parser<'a> {
    pub  fn new(tokens: &'a [TokenKind]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Item> {
        let mut items = vec![];
        while let Some(token) = self.tokens.peek() {
            match token {
                TokenKind::Let => {
                    let declaration = parse_declaration(&mut self.tokens);
                    items.push(Item::Declaration(declaration));
                }
                _ => items.push(Item::Expression(parse_expression(&mut self.tokens))),
            }
        }
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_declaration() {
        // Testing "let x:i32 = 3;"
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("i32".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(3),
            TokenKind::Semicolon,
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Declaration {
                name: "x".to_string(),
                type_: Type::Integer,
                expression: Box::new(Expression::Integer(3)),
            }
        );
    }

    #[test]
    fn test_equality() {
        let tokens = vec![
            TokenKind::Integer(1),
            TokenKind::Equal,
            TokenKind::Integer(1),
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parse_equality(&mut parser.tokens);
        assert_eq!(
            expression,
            Expression::BinaryOperation(BinaryOperation {
                operator: BinaryOperator::Equal,
                left: Box::new(Expression::Integer(1)),
                right: Box::new(Expression::Integer(1)),
            })
        );
    }

    #[test]
    fn test_parser() {
        // Testing "let x:bool = 1+3>2 == true;"
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("bool".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(1),
            TokenKind::Addition,
            TokenKind::Integer(3),
            TokenKind::GreaterThan,
            TokenKind::Integer(2),
            TokenKind::Equal,
            TokenKind::Bool(true),
            TokenKind::Semicolon,
        ];

        let mut parser = Parser::new(&tokens);
        let items = parser.parse();
        assert_eq!(
            items[0],
            Item::Declaration(Declaration {
                name: "x".to_string(),
                type_: Type::Boolean,
                expression: Box::new(Expression::BinaryOperation(BinaryOperation {
                    operator: BinaryOperator::Equal,
                    left: Box::new(Expression::BinaryOperation(BinaryOperation {
                        operator: BinaryOperator::GreaterThan,
                        left: Box::new(Expression::BinaryOperation(BinaryOperation {
                            operator: BinaryOperator::Add,
                            left: Box::new(Expression::Integer(1)),
                            right: Box::new(Expression::Integer(3)),
                        })),
                        right: Box::new(Expression::Integer(2)),
                    })),
                    right: Box::new(Expression::Bool(true)),
                })),
            })
        );
    }
}
