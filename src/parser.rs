#![allow(dead_code)]

use std::{iter::Peekable, slice::Iter};

use crate::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub token: Option<Token>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {:?}", self.message, self.token)
    }
}

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

fn parse_function_declaration(
    tokens: &mut Peekable<Iter<Token>>,
) -> Result<Expression, ParserError> {
    let mut arguments = vec![];
    loop {
        match tokens.peek().unwrap() {
            Token {
                kind: TokenKind::RightParen,
                ..
            } => {
                let _ = tokens.next();
                break;
            }
            Token {
                kind: TokenKind::Comma,
                ..
            } => {
                let _ = tokens.next();
            }
            _ => {
                let name = match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Identifier(name),
                        ..
                    } => name.clone(),
                    token => {
                        return Err(ParserError {
                            message: format!("Expected identifier, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Colon,
                        ..
                    } => {}
                    token => {
                        return Err(ParserError {
                            message: format!("Expected colon, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                let type_ = match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Type(type_),
                        begin,
                        end,
                    } => match type_.as_str() {
                        "num" => Type::Number,
                        "char" => Type::Char,
                        "bool" => Type::Boolean,
                        "fn" => Type::Function,
                        "[num]" => Type::Array(Box::new(Type::Number)),
                        "[bool]" => Type::Array(Box::new(Type::Boolean)),
                        "[char]" => Type::Array(Box::new(Type::Char)),
                        "[fn]" => Type::Array(Box::new(Type::Function)),
                        _ => {
                            return Err(ParserError {
                                message: format!("Unknown type: {:?}", type_),
                                token: Some(Token {
                                    kind: TokenKind::Type(type_.clone()),
                                    begin: *begin,
                                    end: *end,
                                }),
                            })
                        }
                    },
                    token => {
                        return Err(ParserError {
                            message: format!("Expected type, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                arguments.push(FunctionParameter { name, type_ });
            }
        }
    }

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Colon,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected colon, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    let return_type = match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Type(type_),
            begin,
            end,
        } => match type_.as_str() {
            "num" => Type::Number,
            "char" => Type::Char,
            "bool" => Type::Boolean,
            "fn" => Type::Function,
            "[num]" => Type::Array(Box::new(Type::Number)),
            "[bool]" => Type::Array(Box::new(Type::Boolean)),
            "[char]" => Type::Array(Box::new(Type::Char)),
            "[fn]" => Type::Array(Box::new(Type::Function)),
            _ => {
                return Err(ParserError {
                    message: format!("Unknown type: {:?}", type_),
                    token: Some(Token {
                        kind: TokenKind::Type(type_.clone()),
                        begin: *begin,
                        end: *end,
                    }),
                })
            }
        },
        token => {
            return Err(ParserError {
                message: format!("Expected type, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Arrow,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected arrow, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    let expression = match parse_expression(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    Ok(Expression::FunctionDeclaration(FunctionDeclaration {
        parameters: arguments,
        return_type,
        expression: Box::new(expression),
    }))
}

pub fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let expression = match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Identifier(name),
            ..
        } => match tokens.peek().unwrap() {
            Token {
                kind: TokenKind::LeftParen,
                ..
            } => {
                let _ = tokens.next();
                let mut arguments = vec![];
                loop {
                    match tokens.peek().unwrap() {
                        Token {
                            kind: TokenKind::RightParen,
                            ..
                        } => {
                            let _ = tokens.next();
                            break;
                        }
                        Token {
                            kind: TokenKind::Comma,
                            ..
                        } => {
                            let _ = tokens.next();
                        }
                        _ => {
                            let expression = match parse_expression(tokens) {
                                Ok(expression) => expression,
                                Err(error) => return Err(error),
                            };
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
        Token {
            kind: TokenKind::LeftParen,
            ..
        } => {
            // Parsing (identifier:type, identifier:type) => expression
            match parse_function_declaration(tokens) {
                Ok(expression) => expression,
                Err(error) => return Err(error),
            }
        }
        Token {
            kind: TokenKind::Number(value),
            ..
        } => Expression::Number(*value),
        Token {
            kind: TokenKind::Character(value),
            ..
        } => Expression::Char(*value),
        Token {
            kind: TokenKind::Boolean(value),
            ..
        } => Expression::Bool(*value),
        Token {
            kind: TokenKind::String(value),
            ..
        } => Expression::Array(
            Type::Char,
            value.chars().map(|c| Expression::Char(c as u8)).collect(),
        ),
        Token {
            kind: TokenKind::LeftBracket,
            ..
        } => {
            let mut elements = vec![];
            loop {
                match tokens.peek().unwrap() {
                    Token {
                        kind: TokenKind::RightBracket,
                        ..
                    } => {
                        let _ = tokens.next();
                        break;
                    }
                    Token {
                        kind: TokenKind::Comma,
                        ..
                    } => {
                        let _ = tokens.next();
                    }
                    _ => {
                        let expression = match parse_expression(tokens) {
                            Ok(expression) => expression,
                            Err(error) => return Err(error),
                        };
                        elements.push(expression);
                    }
                }
            }
            Expression::Array(Type::Number, elements)
        }
        token => {
            return Err(ParserError {
                message: format!("Unexpected token: {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    if let Some(token) = tokens.peek() {
        if token.kind == TokenKind::Question {
            let _ = tokens.next(); // Consumes question mark
            let mut patterns = vec![];
            let mut default = None;
            match tokens.next().unwrap() {
                Token {
                    kind: TokenKind::LeftBrace,
                    ..
                } => {}
                token => {
                    return Err(ParserError {
                        message: format!("Expected left brace, got {:?}", token.kind),
                        token: Some(token.clone()),
                    })
                }
            }
            loop {
                match tokens.peek().unwrap() {
                    Token {
                        kind: TokenKind::RightBrace,
                        ..
                    } => {
                        let _ = tokens.next();
                        break;
                    }
                    Token {
                        kind: TokenKind::Comma,
                        ..
                    } => {
                        let _ = tokens.next();
                    }
                    _ => {
                        let value = match parse_expression(tokens) {
                            Ok(expression) => expression,
                            Err(error) => return Err(error),
                        };
                        match tokens.peek().unwrap() {
                            Token {
                                kind: TokenKind::RightBrace,
                                ..
                            } => {
                                default = Some(Box::new(value));
                                continue;
                            }
                            Token {
                                kind: TokenKind::Arrow,
                                ..
                            } => {
                                let _ = tokens.next(); // Consume arrow
                            }
                            token => {
                                return Err(ParserError {
                                    message: format!(
                                        "Expected arrow or default, got {:?}",
                                        token.kind
                                    ),
                                    token: Some((**token).clone()),
                                })
                            }
                        };
                        let expression = match parse_expression(tokens) {
                            Ok(expression) => expression,
                            Err(error) => return Err(error),
                        };
                        patterns.push((value, expression));
                    }
                }
            }
            return Ok(Expression::Match(Match {
                value: Box::new(expression),
                patterns,
                default,
            }));
        }
    }

    Ok(expression)
}
pub fn parse_unary(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    parse_primary(tokens)
}

pub fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let mut expression = match parse_unary(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    while let Some(token) = tokens.peek() {
        match token {
            Token {
                kind: TokenKind::Multiplication | TokenKind::Division | TokenKind::Modulus,
                ..
            } => {
                let operator = match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Multiplication,
                        ..
                    } => BinaryOperator::Multiply,
                    Token {
                        kind: TokenKind::Division,
                        ..
                    } => BinaryOperator::Divide,
                    Token {
                        kind: TokenKind::Modulus,
                        ..
                    } => BinaryOperator::Modulo,
                    token => {
                        return Err(ParserError {
                            message: format!(
                                "Expected multiplication, division or modulus, got {:?}",
                                token.kind
                            ),
                            token: Some(token.clone()),
                        })
                    }
                };
                let right = match parse_unary(tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    Ok(expression)
}
pub fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let mut expression = match parse_factor(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    while let Some(token) = tokens.peek() {
        match token {
            Token {
                kind: TokenKind::Addition | TokenKind::Subtraction,
                ..
            } => {
                let operator = match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Addition,
                        ..
                    } => BinaryOperator::Add,
                    Token {
                        kind: TokenKind::Subtraction,
                        ..
                    } => BinaryOperator::Subtract,
                    token => {
                        return Err(ParserError {
                            message: format!("Expected add or subtract, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                let right = match parse_factor(tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    Ok(expression)
}
pub fn parse_comparison(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let mut expression = match parse_term(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    while let Some(token) = tokens.peek() {
        match token {
            Token {
                kind: TokenKind::LessThan,
                ..
            }
            | Token {
                kind: TokenKind::LessEqualThan,
                ..
            }
            | Token {
                kind: TokenKind::GreaterThan,
                ..
            }
            | Token {
                kind: TokenKind::GreaterEqualThan,
                ..
            } => {
                let operator = match tokens.next().unwrap() {
                    Token{kind: TokenKind::LessThan, ..} => BinaryOperator::LessThan,
                    Token{kind: TokenKind::LessEqualThan, ..} => BinaryOperator::LessEqualThan,
                    Token{kind: TokenKind::GreaterThan, ..} => BinaryOperator::GreaterThan,
                    Token{kind: TokenKind::GreaterEqualThan, ..} => BinaryOperator::GreaterEqualThan,
                    token => {
                        return Err(ParserError {
                            message: format!("Expected less than, less equal than, greater than or greater equal than, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                let right = match parse_term(tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    Ok(expression)
}
pub fn parse_equality(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let mut expression = match parse_comparison(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    while let Some(token) = tokens.peek() {
        match token {
            Token {
                kind: TokenKind::Equal | TokenKind::NotEqual,
                ..
            } => {
                let operator = match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Equal,
                        ..
                    } => BinaryOperator::Equal,
                    Token {
                        kind: TokenKind::NotEqual,
                        ..
                    } => BinaryOperator::NotEqual,
                    token => {
                        return Err(ParserError {
                            message: format!("Expected equal or not equal, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                };
                let right = match parse_comparison(tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };
                expression = Expression::BinaryOperation(BinaryOperation {
                    operator,
                    left: Box::new(expression),
                    right: Box::new(right),
                });
            }
            _ => break,
        }
    }

    Ok(expression)
}

pub fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    parse_equality(tokens)
}

pub fn parse_declaration(tokens: &mut Peekable<Iter<Token>>) -> Result<Declaration, ParserError> {
    let _ = match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Let,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected let, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    let name = match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Identifier(name),
            ..
        } => name.clone(),
        token => {
            return Err(ParserError {
                message: format!("Expected identifier, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Colon,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected colon, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    let type_ = match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Type(type_),
            begin,
            end,
        } => match type_.as_str() {
            "num" => Type::Number,
            "char" => Type::Char,
            "fn" => Type::Function,
            "bool" => Type::Boolean,
            "[num]" => Type::Array(Box::new(Type::Number)),
            "[bool]" => Type::Array(Box::new(Type::Boolean)),
            "[char]" => Type::Array(Box::new(Type::Char)),
            "[fn]" => Type::Array(Box::new(Type::Function)),
            _ => {
                return Err(ParserError {
                    message: format!("Unknown type: {:?}", type_),
                    token: Some(Token {
                        kind: TokenKind::Type(type_.clone()),
                        begin: *begin,
                        end: *end,
                    }),
                })
            }
        },
        token => {
            return Err(ParserError {
                message: format!("Expected type, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Assignment,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected assignment, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };

    let expression = match parse_expression(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::Semicolon,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected semicolon, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    };
    Ok(Declaration {
        name,
        type_,
        expression: Box::new(expression),
    })
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Item>, ParserError> {
    let mut tokens = tokens.iter().peekable();
    let mut items = vec![];
    while let Some(token) = tokens.peek() {
        match token.kind {
            TokenKind::EOF => break,
            TokenKind::Let => match parse_declaration(&mut tokens) {
                Ok(declaration) => items.push(Item::Declaration(declaration)),
                Err(error) => return Err(error),
            },
            _ => {
                let expression = match parse_expression(&mut tokens) {
                    Ok(expression) => expression,
                    Err(error) => return Err(error),
                };
                items.push(Item::Expression(expression));
                match tokens.next().unwrap() {
                    Token {
                        kind: TokenKind::Semicolon,
                        ..
                    } => {}
                    token => {
                        return Err(ParserError {
                            message: format!("Expected semicolon, got {:?}", token.kind),
                            token: Some(token.clone()),
                        })
                    }
                }
            }
        }
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Testing `let x:num = 3;`
    #[test]
    fn test_parse_declaration() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("num".to_string()), 6, 8),
            Token::new(TokenKind::Assignment, 10, 10),
            Token::new(TokenKind::Number(3.0), 12, 12),
            Token::new(TokenKind::Semicolon, 13, 13),
            Token::new(TokenKind::EOF, 14, 14),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
                name: "x".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Number(3.0)),
            })
        );
    }

    /// Testing type number `let x:num = 3;`
    #[test]
    fn test_type_number() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("num".to_string()), 6, 8),
            Token::new(TokenKind::Assignment, 10, 10),
            Token::new(TokenKind::Number(3.0), 12, 12),
            Token::new(TokenKind::Semicolon, 13, 13),
            Token::new(TokenKind::EOF, 14, 14),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
                name: "x".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Number(3.0)),
            })
        );
    }

    /// Testing type char `let x:char = 'a';`
    #[test]
    fn test_type_char() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("char".to_string()), 6, 9),
            Token::new(TokenKind::Assignment, 10, 10),
            Token::new(TokenKind::Character('a' as u8), 12, 12),
            Token::new(TokenKind::Semicolon, 13, 13),
            Token::new(TokenKind::EOF, 14, 14),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
                name: "x".to_string(),
                type_: Type::Char,
                expression: Box::new(Expression::Char('a' as u8)),
            })
        );
    }

    /// Testing type bool `let x:bool = true;`
    #[test]
    fn test_type_bool() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("bool".to_string()), 6, 9),
            Token::new(TokenKind::Assignment, 11, 11),
            Token::new(TokenKind::Boolean(true), 13, 16),
            Token::new(TokenKind::Semicolon, 17, 17),
            Token::new(TokenKind::EOF, 18, 18),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
                name: "x".to_string(),
                type_: Type::Boolean,
                expression: Box::new(Expression::Bool(true)),
            })
        );
    }

    /// Testing type fn `let x:fn = (x:num, y:num):num => x+y;`
    #[test]
    fn test_type_fn() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("fn".to_string()), 6, 8),
            Token::new(TokenKind::Assignment, 10, 10),
            Token::new(TokenKind::LeftParen, 12, 12),
            Token::new(TokenKind::Identifier("x".to_string()), 13, 13),
            Token::new(TokenKind::Colon, 14, 14),
            Token::new(TokenKind::Type("num".to_string()), 15, 17),
            Token::new(TokenKind::Comma, 19, 19),
            Token::new(TokenKind::Identifier("y".to_string()), 21, 21),
            Token::new(TokenKind::Colon, 22, 22),
            Token::new(TokenKind::Type("num".to_string()), 23, 25),
            Token::new(TokenKind::RightParen, 26, 26),
            Token::new(TokenKind::Colon, 28, 28),
            Token::new(TokenKind::Type("num".to_string()), 29, 31),
            Token::new(TokenKind::Arrow, 33, 34),
            Token::new(TokenKind::Identifier("x".to_string()), 36, 36),
            Token::new(TokenKind::Addition, 37, 37),
            Token::new(TokenKind::Identifier("y".to_string()), 38, 38),
            Token::new(TokenKind::Semicolon, 39, 39),
            Token::new(TokenKind::EOF, 40, 40),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
                name: "x".to_string(),
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

    /// Testing type [num] `let x:[num] = [1, 2, 3];`
    #[test]
    fn test_type_array() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("[num]".to_string()), 6, 10),
            Token::new(TokenKind::Assignment, 12, 12),
            Token::new(TokenKind::LeftBracket, 14, 14),
            Token::new(TokenKind::Number(1.0), 15, 15),
            Token::new(TokenKind::Comma, 16, 16),
            Token::new(TokenKind::Number(2.0), 18, 18),
            Token::new(TokenKind::Comma, 19, 19),
            Token::new(TokenKind::Number(3.0), 21, 21),
            Token::new(TokenKind::RightBracket, 22, 22),
            Token::new(TokenKind::Semicolon, 23, 23),
            Token::new(TokenKind::EOF, 24, 24),
        ];
        let declaration = parse_declaration(&mut tokens.iter().peekable());
        assert_eq!(
            declaration,
            Ok(Declaration {
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

    /// Testing `1 == 1`
    #[test]
    fn test_equality() {
        let tokens = vec![
            Token::new(TokenKind::Number(1.0), 0, 0),
            Token::new(TokenKind::Equal, 2, 3),
            Token::new(TokenKind::Number(1.0), 5, 5),
        ];
        let expression = parse_equality(&mut tokens.iter().peekable());
        assert_eq!(
            expression,
            Ok(Expression::BinaryOperation(BinaryOperation {
                operator: BinaryOperator::Equal,
                left: Box::new(Expression::Number(1.0)),
                right: Box::new(Expression::Number(1.0)),
            }))
        );
    }

    /// Testing `print(1);`
    #[test]
    fn test_function_call() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("print".to_string()), 0, 4),
            Token::new(TokenKind::LeftParen, 5, 5),
            Token::new(TokenKind::Number(1.0), 6, 6),
            Token::new(TokenKind::RightParen, 7, 7),
            Token::new(TokenKind::Semicolon, 8, 8),
        ];
        let expression = parse_expression(&mut tokens.iter().peekable());
        assert_eq!(
            expression,
            Ok(Expression::FunctionCall(FunctionCall {
                name: "print".to_string(),
                arguments: vec![Expression::Number(1.0)],
            }))
        );
    }

    /// Testing `let add:fn = (x:num, y:num):num => x+y;`
    #[test]
    fn test_function_declaration() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("add".to_string()), 4, 6),
            Token::new(TokenKind::Colon, 7, 7),
            Token::new(TokenKind::Type("fn".to_string()), 8, 10),
            Token::new(TokenKind::Assignment, 12, 12),
            Token::new(TokenKind::LeftParen, 14, 14),
            Token::new(TokenKind::Identifier("x".to_string()), 15, 15),
            Token::new(TokenKind::Colon, 16, 16),
            Token::new(TokenKind::Type("num".to_string()), 17, 19),
            Token::new(TokenKind::Comma, 21, 21),
            Token::new(TokenKind::Identifier("y".to_string()), 23, 23),
            Token::new(TokenKind::Colon, 24, 24),
            Token::new(TokenKind::Type("num".to_string()), 25, 27),
            Token::new(TokenKind::RightParen, 28, 28),
            Token::new(TokenKind::Colon, 30, 30),
            Token::new(TokenKind::Type("num".to_string()), 31, 33),
            Token::new(TokenKind::Arrow, 35, 36),
            Token::new(TokenKind::Identifier("x".to_string()), 38, 38),
            Token::new(TokenKind::Addition, 39, 39),
            Token::new(TokenKind::Identifier("y".to_string()), 40, 40),
            Token::new(TokenKind::Semicolon, 41, 41),
            Token::new(TokenKind::EOF, 42, 42),
        ];

        let items = parse(&tokens).unwrap();
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

    /// Testing `let x:num = 1 ? {1 => 2, 2 => 3, 4 };`
    #[test]
    fn test_match() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("num".to_string()), 6, 8),
            Token::new(TokenKind::Assignment, 10, 10),
            Token::new(TokenKind::Number(1.0), 12, 12),
            Token::new(TokenKind::Question, 14, 14),
            Token::new(TokenKind::LeftBrace, 16, 16),
            Token::new(TokenKind::Number(1.0), 17, 17),
            Token::new(TokenKind::Arrow, 19, 20),
            Token::new(TokenKind::Number(2.0), 22, 22),
            Token::new(TokenKind::Comma, 24, 24),
            Token::new(TokenKind::Number(2.0), 26, 26),
            Token::new(TokenKind::Arrow, 28, 29),
            Token::new(TokenKind::Number(3.0), 31, 31),
            Token::new(TokenKind::Comma, 33, 33),
            Token::new(TokenKind::Number(4.0), 35, 35),
            Token::new(TokenKind::RightBrace, 36, 36),
            Token::new(TokenKind::Semicolon, 37, 37),
            Token::new(TokenKind::EOF, 38, 38),
        ];

        let items = parse(&tokens).unwrap();

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

    /// Testing `let x:[num] = [1, 2, 3];`
    #[test]
    fn test_array() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("[num]".to_string()), 6, 10),
            Token::new(TokenKind::Assignment, 12, 12),
            Token::new(TokenKind::LeftBracket, 14, 14),
            Token::new(TokenKind::Number(1.0), 15, 15),
            Token::new(TokenKind::Comma, 16, 16),
            Token::new(TokenKind::Number(2.0), 18, 18),
            Token::new(TokenKind::Comma, 19, 19),
            Token::new(TokenKind::Number(3.0), 21, 21),
            Token::new(TokenKind::RightBracket, 22, 22),
            Token::new(TokenKind::Semicolon, 23, 23),
            Token::new(TokenKind::EOF, 24, 24),
        ];

        let items = parse(&tokens).unwrap();

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

    /// Testing `let x:[char] = "nino";`
    #[test]
    fn test_string() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("[char]".to_string()), 6, 10),
            Token::new(TokenKind::Assignment, 12, 12),
            Token::new(TokenKind::String("nino".to_string()), 14, 18),
            Token::new(TokenKind::Semicolon, 19, 19),
            Token::new(TokenKind::EOF, 20, 20),
        ];

        let items = parse(&tokens).unwrap();

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

    /// Testing `let x:bool = 1+3>2 == 1;`
    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(TokenKind::Let, 0, 2),
            Token::new(TokenKind::Identifier("x".to_string()), 4, 4),
            Token::new(TokenKind::Colon, 5, 5),
            Token::new(TokenKind::Type("bool".to_string()), 6, 9),
            Token::new(TokenKind::Assignment, 11, 11),
            Token::new(TokenKind::Number(1.0), 13, 13),
            Token::new(TokenKind::Addition, 14, 14),
            Token::new(TokenKind::Number(3.0), 15, 15),
            Token::new(TokenKind::GreaterThan, 16, 16),
            Token::new(TokenKind::Number(2.0), 17, 17),
            Token::new(TokenKind::Equal, 19, 20),
            Token::new(TokenKind::Number(1.0), 22, 22),
            Token::new(TokenKind::Semicolon, 23, 23),
            Token::new(TokenKind::EOF, 24, 24),
        ];

        let items = parse(&tokens).unwrap();
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
                    right: Box::new(Expression::Number(1.0)),
                })),
            })
        );
    }
}
