#![allow(dead_code)]

use std::{iter::Peekable, slice::Iter};

use crate::lexer::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    Char,
    Boolean,
    Function,
    Array(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Match {
    pub value: Box<Expression>,
    pub patterns: Vec<(Expression, Expression)>,
    pub default: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub name: String,
    pub type_: Type,
    pub expression: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),

    Number(f64),
    Char(u8),
    Bool(bool),

    Array(Type, Vec<Expression>),

    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    Match(Match),

    BinaryOperation(BinaryOperation),
}

#[derive(Debug, PartialEq)]
pub enum Item {
    Declaration(Declaration),
    Expression(Expression),
}

fn parse_function_declaration(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut arguments = vec![];
    loop {
        match tokens.peek() {
            Some(&&TokenKind::RightParen) => {
                let _ = tokens.next();
                break;
            }
            Some(&&TokenKind::Comma) => {
                let _ = tokens.next();
            }
            _ => {
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
                        "num" => Type::Number,
                        "char" => Type::Char,
                        "bool" => Type::Boolean,
                        _ => panic!("Unknown type: {:?}", type_),
                    },
                    _ => panic!("Expected type"),
                };
                arguments.push(FunctionParameter { name, type_ });
            }
        }
    }
    match tokens.next() {
        Some(TokenKind::Colon) => {}
        _ => panic!("Expected colon"),
    };
    let return_type = match tokens.next() {
        Some(TokenKind::Type(type_)) => match type_.as_str() {
            "num" => Type::Number,
            "char" => Type::Char,
            "bool" => Type::Boolean,
            _ => panic!("Unknown type: {:?}", type_),
        },
        _ => panic!("Expected type"),
    };
    match tokens.next() {
        Some(TokenKind::Arrow) => {}
        _ => panic!("Expected arrow"),
    };
    let expression = parse_expression(tokens);
    Expression::FunctionDeclaration(FunctionDeclaration {
        parameters: arguments,
        return_type,
        expression: Box::new(expression),
    })
}

pub fn parse_primary(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let expression = match tokens.next() {
        Some(TokenKind::Identifier(name)) => match tokens.peek() {
            Some(&&TokenKind::LeftParen) => {
                let _ = tokens.next();
                let mut arguments = vec![];
                loop {
                    match tokens.peek() {
                        Some(&&TokenKind::RightParen) => {
                            let _ = tokens.next();
                            break;
                        }
                        Some(&&TokenKind::Comma) => {
                            let _ = tokens.next();
                        }
                        _ => {
                            let expression = parse_expression(tokens);
                            arguments.push(expression);
                        }
                    }
                }
                Expression::FunctionCall(FunctionCall {
                    name: name.clone(),
                    arguments,
                })
            }
            _ => Expression::Identifier(name.clone()),
        },
        Some(TokenKind::LeftParen) => {
            // Parsing (identifier:type, identifier:type) => expression
            parse_function_declaration(tokens)
        }
        Some(TokenKind::Number(value)) => Expression::Number(*value),
        Some(TokenKind::Character(value)) => Expression::Char(*value),
        Some(TokenKind::String(value)) => Expression::Array(
            Type::Char,
            value.chars().map(|c| Expression::Char(c as u8)).collect(),
        ),
        Some(TokenKind::LeftBracket) => {
            let mut elements = vec![];
            loop {
                match tokens.peek() {
                    Some(&&TokenKind::RightBracket) => {
                        let _ = tokens.next();
                        break;
                    }
                    Some(&&TokenKind::Comma) => {
                        let _ = tokens.next();
                    }
                    _ => {
                        let expression = parse_expression(tokens);
                        elements.push(expression);
                    }
                }
            }
            Expression::Array(Type::Number, elements)
        }
        _ => panic!(
            "\nExpected identifier, integer, float, or left paren, got {:?}\n",
            tokens.peek().unwrap()
        ),
    };

    if tokens.peek() == Some(&&TokenKind::Question) {
        let _ = tokens.next(); // Consumes question mark
        let mut patterns = vec![];
        let mut default = None;
        match tokens.next() {
            Some(TokenKind::LeftBrace) => {}
            _ => panic!("Expected left brace"),
        }
        loop {
            match tokens.peek() {
                Some(&&TokenKind::RightBrace) => {
                    let _ = tokens.next();
                    break;
                }
                Some(&&TokenKind::Comma) => {
                    let _ = tokens.next();
                }
                _ => {
                    let value = parse_expression(tokens);
                    match tokens.peek() {
                        Some(TokenKind::RightBrace) => {
                            default = Some(Box::new(value));
                            continue;
                        }
                        Some(TokenKind::Arrow) => {
                            let _ = tokens.next(); // Consume arrow
                        }
                        _ => panic!("Expected arrow or default"),
                    };
                    let expression = parse_expression(tokens);
                    patterns.push((value, expression));
                }
            }
        }
        return Expression::Match(Match {
            value: Box::new(expression),
            patterns,
            default,
        });
    }

    expression
}
pub fn parse_unary(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    parse_primary(tokens)
}
pub fn parse_factor(tokens: &mut Peekable<Iter<TokenKind>>) -> Expression {
    let mut expression = parse_unary(tokens);

    loop {
        match tokens.peek() {
            Some(&&TokenKind::Multiplication)
            | Some(&&TokenKind::Division)
            | Some(&&TokenKind::Modulus) => {
                let operator = match tokens.next() {
                    Some(TokenKind::Multiplication) => BinaryOperator::Multiply,
                    Some(TokenKind::Division) => BinaryOperator::Divide,
                    Some(TokenKind::Modulus) => BinaryOperator::Modulo,
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
            "num" => Type::Number,
            "char" => Type::Char,
            "fn" => Type::Function,
            "bool" => Type::Boolean,
            "[num]" => Type::Array(Box::new(Type::Number)),
            "[bool]" => Type::Array(Box::new(Type::Boolean)),
            "[char]" => Type::Array(Box::new(Type::Char)),

            _ => panic!("Unknown type: {:?}", type_),
        },
        _ => panic!("Expected type"),
    };
    match tokens.next() {
        Some(TokenKind::Assignment) => {}
        _ => panic!("Expected equal, got {:?}", tokens.peek()),
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
    pub fn new(tokens: &'a [TokenKind]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Vec<Item> {
        let mut items = vec![];
        while let Some(token) = self.tokens.peek() {
            match token {
                TokenKind::EOF => break,
                TokenKind::Let => {
                    let declaration = parse_declaration(&mut self.tokens);
                    items.push(Item::Declaration(declaration));
                }
                _ => {
                    let expression = parse_expression(&mut self.tokens);
                    items.push(Item::Expression(expression));
                    self.tokens.next(); // Consume semicolon
                }
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
        // Testing "let x:num = 3;"
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("num".to_string()),
            TokenKind::Assignment,
            TokenKind::Number(3.0),
            TokenKind::Semicolon,
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Declaration {
                name: "x".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_equality() {
        let tokens = vec![
            TokenKind::Number(1.0),
            TokenKind::Equal,
            TokenKind::Number(1.0),
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parse_equality(&mut parser.tokens);
        assert_eq!(
            expression,
            Expression::BinaryOperation(BinaryOperation {
                operator: BinaryOperator::Equal,
                left: Box::new(Expression::Number(1.0)),
                right: Box::new(Expression::Number(1.0)),
            })
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = vec![
            TokenKind::Identifier("print".to_string()),
            TokenKind::LeftParen,
            TokenKind::Number(1.0),
            TokenKind::RightParen,
        ];
        let mut parser = Parser::new(&tokens);
        let expression = parse_expression(&mut parser.tokens);
        assert_eq!(
            expression,
            Expression::FunctionCall(FunctionCall {
                name: "print".to_string(),
                arguments: vec![Expression::Number(1.0)],
            })
        );
    }

    #[test]
    fn test_function_declaration() {
        // Testing "let add:fn = (x:num, y:num):num => x+y;"
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("add".to_string()),
            TokenKind::Colon,
            TokenKind::Type("fn".to_string()),
            TokenKind::Assignment,
            TokenKind::LeftParen,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("num".to_string()),
            TokenKind::Comma,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Colon,
            TokenKind::Type("num".to_string()),
            TokenKind::RightParen,
            TokenKind::Colon,
            TokenKind::Type("num".to_string()),
            TokenKind::Arrow,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Addition,
            TokenKind::Identifier("y".to_string()),
            TokenKind::Semicolon,
        ];

        let mut parser = Parser::new(&tokens);
        let items = parser.parse();
        assert_eq!(
            items[0],
            Item::Declaration(Declaration {
                name: "add".to_string(),
                type_: Type::Function,
                expression: Box::new(Expression::FunctionDeclaration(FunctionDeclaration {
                    parameters: vec![
                        FunctionParameter {
                            name: "x".to_string(),
                            type_: Type::Number,
                        },
                        FunctionParameter {
                            name: "y".to_string(),
                            type_: Type::Number,
                        }
                    ],
                    return_type: Type::Number,
                    expression: Box::new(Expression::BinaryOperation(BinaryOperation {
                        operator: BinaryOperator::Add,
                        left: Box::new(Expression::Identifier("x".to_string())),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    })),
                }))
            })
        );
    }

    #[test]
    fn test_match() {
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("num".to_string()),
            TokenKind::Assignment,
            TokenKind::Number(1.0),
            TokenKind::Question,
            TokenKind::LeftBrace,
            TokenKind::Number(1.0),
            TokenKind::Arrow,
            TokenKind::Number(2.0),
            TokenKind::Comma,
            TokenKind::Number(2.0),
            TokenKind::Arrow,
            TokenKind::Number(3.0),
            TokenKind::Comma,
            TokenKind::Number(4.0),
            TokenKind::RightBrace,
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];

        let mut parser = Parser::new(&tokens);
        let items = parser.parse();

        assert_eq!(
            items[0],
            Item::Declaration(Declaration {
                name: "x".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Match(Match {
                    value: Box::new(Expression::Number(1.0)),
                    patterns: vec![
                        (Expression::Number(1.0), Expression::Number(2.0),),
                        (Expression::Number(2.0), Expression::Number(3.0),),
                    ],
                    default: Some(Box::new(Expression::Number(4.0))),
                }))
            })
        );
    }

    #[test]
    fn test_array() {
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("[num]".to_string()),
            TokenKind::Assignment,
            TokenKind::LeftBracket,
            TokenKind::Number(1.0),
            TokenKind::Comma,
            TokenKind::Number(2.0),
            TokenKind::Comma,
            TokenKind::Number(3.0),
            TokenKind::RightBracket,
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];

        let mut parser = Parser::new(&tokens);
        let items = parser.parse();

        assert_eq!(
            items[0],
            Item::Declaration(Declaration {
                name: "x".to_string(),
                type_: Type::Array(Box::new(Type::Number)),
                expression: Box::new(Expression::Array(
                    Type::Number,
                    vec![
                        Expression::Number(1.0),
                        Expression::Number(2.0),
                        Expression::Number(3.0),
                    ]
                ))
            })
        );
    }

    #[test]
    fn test_string() {
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("[char]".to_string()),
            TokenKind::Assignment,
            TokenKind::String("nino".to_string()),
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];

        let mut parser = Parser::new(&tokens);
        let items = parser.parse();

        assert_eq!(
            items[0],
            Item::Declaration(Declaration {
                name: "x".to_string(),
                type_: Type::Array(Box::new(Type::Char)),
                expression: Box::new(Expression::Array(
                    Type::Char,
                    vec![
                        Expression::Char('n' as u8),
                        Expression::Char('i' as u8),
                        Expression::Char('n' as u8),
                        Expression::Char('o' as u8),
                    ]
                ))
            })
        );
    }

    #[test]
    fn test_parser() {
        // Testing "let x:bool = 1+3>2 == 1;"
        let tokens = vec![
            TokenKind::Let,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Colon,
            TokenKind::Type("bool".to_string()),
            TokenKind::Assignment,
            TokenKind::Number(1.0),
            TokenKind::Addition,
            TokenKind::Number(3.0),
            TokenKind::GreaterThan,
            TokenKind::Number(2.0),
            TokenKind::Equal,
            TokenKind::Character(1),
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
                            left: Box::new(Expression::Number(1.0)),
                            right: Box::new(Expression::Number(3.0)),
                        })),
                        right: Box::new(Expression::Number(2.0)),
                    })),
                    right: Box::new(Expression::Char(1)),
                })),
            })
        );
    }
}
