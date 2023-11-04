use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{BinaryOperator, Declaration, Expression, Item};

fn print(expression: Expression) -> Expression {
    match expression {
        Expression::Integer(val) => println!("{}", val),
        Expression::Float(val) => println!("{}", val),
        Expression::Bool(val) => println!("{}", val),
        _ => panic!("Invalid type"),
    }
    return expression;
}

fn time() -> Expression {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    return Expression::Float(in_ms as f32);
}

fn binary_integer_integer(left_val: i32, right_val: i32, operator: BinaryOperator) -> Expression {
    match operator {
        BinaryOperator::Add => Expression::Integer(left_val + right_val),
        BinaryOperator::Subtract => Expression::Integer(left_val - right_val),
        BinaryOperator::Multiply => Expression::Integer(left_val * right_val),
        BinaryOperator::Divide => Expression::Float((left_val as f32) / (right_val as f32)),
        BinaryOperator::Modulo => Expression::Integer(left_val % right_val),
        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn binary_float_float(left_val: f32, right_val: f32, operator: BinaryOperator) -> Expression {
    match operator {
        BinaryOperator::Add => Expression::Float(left_val + right_val),
        BinaryOperator::Subtract => Expression::Float(left_val - right_val),
        BinaryOperator::Multiply => Expression::Float(left_val * right_val),
        BinaryOperator::Divide => Expression::Float(left_val / right_val),
        BinaryOperator::Modulo => Expression::Float(left_val % right_val),
        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn binary_integer_float(left_val: i32, right_val: f32, operator: BinaryOperator) -> Expression {
    match operator {
        BinaryOperator::Add => Expression::Float((left_val as f32) + right_val),
        BinaryOperator::Subtract => Expression::Float((left_val as f32) - right_val),
        BinaryOperator::Multiply => Expression::Float((left_val as f32) * right_val),
        BinaryOperator::Divide => Expression::Float((left_val as f32) / right_val),
        BinaryOperator::Modulo => Expression::Float((left_val as f32) % right_val),
        BinaryOperator::Equal => Expression::Bool(left_val == (right_val as i32)),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn binary_float_integer(left_val: f32, right_val: i32, operator: BinaryOperator) -> Expression {
    match operator {
        BinaryOperator::Add => Expression::Float(left_val + (right_val as f32)),
        BinaryOperator::Subtract => Expression::Float(left_val - (right_val as f32)),
        BinaryOperator::Multiply => Expression::Float(left_val * (right_val as f32)),
        BinaryOperator::Divide => Expression::Float(left_val / (right_val as f32)),
        BinaryOperator::Modulo => Expression::Float(left_val % (right_val as f32)),
        BinaryOperator::Equal => Expression::Bool(left_val == (right_val as f32)),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn evaluate(expression: Expression, symbols: &HashMap<String, Declaration>) -> Expression {
    loop {
        return match expression {
            Expression::Integer(_) | Expression::Float(_) | Expression::Bool(_) => expression,
            Expression::Identifier(identifier) => {
                let declaration = symbols.get(&identifier).unwrap();
                let expression = declaration.expression.clone();
                return evaluate(*expression, symbols);
            }
            Expression::FunctionCall(ref function_call) => match function_call.name.as_str() {
                "print" => print(evaluate(function_call.arguments[0].clone(), symbols)),
                "time" => time(),
                _ => {
                    // Get the function declaration
                    let declaration = symbols.get(&function_call.name).unwrap();
                    // The expression which we know to be a function declaration
                    let function_declaration_expression = (*declaration.expression).clone();

                    let function = match function_declaration_expression {
                        Expression::FunctionDeclaration(function) => function,
                        _ => panic!("Invalid function"),
                    };

                    let mut local_symbols: HashMap<String, Declaration> = symbols.clone();
                    for (i, argument) in function_call.arguments.iter().enumerate() {
                        let name = function.parameters[i].name.clone();
                        let type_ = function.parameters[i].type_.clone();
                        let expression = evaluate(argument.clone(), symbols);
                        let declaration = Declaration {
                            name: name.clone(),
                            type_: type_.clone(),
                            expression: Box::new(expression),
                        };
                        local_symbols.insert(name.clone(), declaration);
                    }
                    return evaluate(*function.expression, &local_symbols);
                }
            },
            Expression::BinaryOperation(binary) => {
                let left = evaluate(*binary.left, symbols);
                let right = evaluate(*binary.right, symbols);
                let operator = binary.operator;
                return match (left, right) {
                    // Perform operations based on the types and the operator
                    (Expression::Integer(left_val), Expression::Integer(right_val)) => {
                        binary_integer_integer(left_val, right_val, operator)
                    }
                    (Expression::Float(left_val), Expression::Float(right_val)) => {
                        binary_float_float(left_val, right_val, operator)
                    }
                    (Expression::Integer(left_val), Expression::Float(right_val)) => {
                        binary_integer_float(left_val, right_val, operator)
                    }
                    (Expression::Float(left_val), Expression::Integer(right_val)) => {
                        binary_float_integer(left_val, right_val, operator)
                    }
                    _ => panic!("Invalid types or operation"), // Handle other cases or operations
                };
            }

            Expression::Match(match_) => {
                let expression = evaluate(*match_.value, symbols);
                for case in match_.patterns {
                    let left = evaluate(case.0, symbols);
                    if expression == left {
                        return evaluate(case.1, symbols);
                    }
                }
                return evaluate(*match_.default.unwrap(), symbols);
            }
            _ => panic!("Unknown expression {:?}", expression),
        };
    }
}

pub struct VirtualMachine {
    symbols: HashMap<String, Declaration>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            symbols: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, expression: Expression) -> Expression {
        evaluate(expression, &self.symbols)
    }

    pub fn interpret(&mut self, program: Vec<Item>) {
        for statement in program {
            match statement {
                Item::Declaration(declaration) => {
                    self.symbols.insert(declaration.name.clone(), declaration);
                }
                Item::Expression(expression) => {
                    evaluate(expression, &self.symbols);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        parser::{BinaryOperation, Parser},
    };

    use super::*;

    #[test]
    fn test_expression() {
        // Test adding two integers
        let expression = Expression::BinaryOperation(BinaryOperation {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        });

        let mut vm = VirtualMachine::new();
        let result = vm.evaluate(expression);

        assert_eq!(result, Expression::Integer(3));
    }

    #[test]
    fn test_recursion() {
        let declare = "let factorial:fn = (n:i32):i32 => n ? {
    0 => 1,
    n * factorial(n - 1)
};
        ";
        let mut lexer = Lexer::new(declare);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let declaration_ast = parser.parse();

        let input = "factorial(5);";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.interpret(declaration_ast);
        println!("{:?}", vm.symbols);
        vm.interpret(expression);
    }
}
