/// Integration tests for the lexer and parser
use nino::{
    lexer::tokenize,
    parser::{
        parse, parse_declaration, BinaryOperation, BinaryOperator, Declaration, Expression,
        FunctionDeclaration, FunctionParameter, Item, Match, Type,
    },
};

#[test]
fn test_parse_declaration() {
    let tokens = tokenize("let x:num = 3;");
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

#[test]
fn test_fail_parse_declaration() {
    let tokens = tokenize("let x: = 3.0;");
    let declaration = parse_declaration(&mut tokens.iter().peekable());
    assert!(declaration.is_err());
}

#[test]
fn test_type_number() {
    let tokens = tokenize("let x:num = 3;");
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

#[test]
fn test_type_number_float() {
    let tokens = tokenize("let x:num = 3.0;");
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

#[test]
fn test_type_negative_number_float() {
    let tokens = tokenize("let x:num = -3.0;");
    let declaration = parse_declaration(&mut tokens.iter().peekable());
    assert_eq!(
        declaration,
        Ok(Declaration {
            name: "x".to_string(),
            type_: Type::Number,
            expression: Box::new(Expression::Number(-3.0)),
        })
    );
}

#[test]
fn test_type_char() {
    let tokens = tokenize("let x:char = 'a';");
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

#[test]
fn test_type_bool() {
    let tokens = tokenize("let x:bool = true;");
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
    use nino::parser::Expression;
    let tokens = tokenize("let x:fn = (x:num, y:num):num => x+y;");
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
    let tokens = tokenize("let x:[num] = [1, 2, 3];");
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

/// Testing `let add:fn = (x:num, y:num):num => x+y;`
#[test]
fn test_function_declaration() {
    let tokens = tokenize("let add:fn = (x:num, y:num):num => x+y;");
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
    let tokens = tokenize("let x:num = 1 ? {1 => 2, 2 => 3, 4 };");

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
    let tokens = tokenize("let x:[num] = [1, 2, 3];");

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
    let tokens = tokenize("let x:[char] = \"nino\";");

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
    let tokens = tokenize("let x:bool = 1+3>2 == 1;");

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
