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

fn parse_group(tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, ParserError> {
    let expression = match parse_expression(tokens) {
        Ok(expression) => expression,
        Err(error) => return Err(error),
    };

    match tokens.next().unwrap() {
        Token {
            kind: TokenKind::RightParen,
            ..
        } => {}
        token => {
            return Err(ParserError {
                message: format!("Expected right parenthesis, got {:?}", token.kind),
                token: Some(token.clone()),
            })
        }
    }

    Ok(expression)
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
            // TODO: This is a hack, we should be able to parse a group without cloning the tokens

            // Try parsing a group, if that fails, try parsing a function declaration
            let mut experiment = tokens.clone();
            let possible_group = parse_group(&mut experiment);

            match possible_group {
                Ok(_) => parse_group(tokens),
                Err(_) => parse_function_declaration(tokens),
            }?
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



    /// Testing `(1+1) ? {1 => 2, 2 => 3, 4 };`
    #[test]
    fn test_match_group() {
        let tokens = vec![
            Token::new(TokenKind::LeftParen, 0, 0),
            Token::new(TokenKind::Number(1.0), 1, 1),
            Token::new(TokenKind::Addition, 2, 2),
            Token::new(TokenKind::Number(1.0), 3, 3),
            Token::new(TokenKind::RightParen, 4, 4),
            Token::new(TokenKind::Question, 6, 6),
            Token::new(TokenKind::LeftBrace, 8, 8),
            Token::new(TokenKind::Number(1.0), 9, 9),
            Token::new(TokenKind::Arrow, 11, 12),
            Token::new(TokenKind::Number(2.0), 14, 14),
            Token::new(TokenKind::Comma, 16, 16),
            Token::new(TokenKind::Number(2.0), 18, 18),
            Token::new(TokenKind::Arrow, 20, 21),
            Token::new(TokenKind::Number(3.0), 23, 23),
            Token::new(TokenKind::Comma, 25, 25),
            Token::new(TokenKind::Number(4.0), 27, 27),
            Token::new(TokenKind::RightBrace, 28, 28),
            Token::new(TokenKind::Semicolon, 29, 29),
            Token::new(TokenKind::EOF, 30, 30),
        ];

        let items = parse(&tokens).unwrap();

        assert_eq!(
            items[0],
            Item::Expression(Expression::Match(Match {
                value: Box::new(Expression::BinaryOperation(BinaryOperation {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::Number(1.0)),
                    right: Box::new(Expression::Number(1.0)),
                })),
                patterns: vec![
                    (Expression::Number(1.0), Expression::Number(2.0),),
                    (Expression::Number(2.0), Expression::Number(3.0),),
                ],
                default: Some(Box::new(Expression::Number(4.0))),
            }))
        );
    }

    #[test]
    fn test_group(){
        let tokens = vec![
            Token::new(TokenKind::LeftParen, 0, 0),
            Token::new(TokenKind::Number(1.0), 1, 1),
            Token::new(TokenKind::Addition, 3, 3),
            Token::new(TokenKind::Number(2.0), 5, 5),
            Token::new(TokenKind::RightParen, 6, 6),
            Token::new(TokenKind::EOF, 7, 7),
        ];

        let expression = parse_expression(&mut tokens.iter().peekable());
        assert_eq!(
            expression,
            Ok(Expression::BinaryOperation(BinaryOperation {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::Number(1.0)),
                right: Box::new(Expression::Number(2.0)),
            }))
        );
    }
}
