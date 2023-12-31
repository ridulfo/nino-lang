use std::collections::HashMap;
use std::mem::discriminant;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{BinaryOperator, Declaration, Expression, Item, Type};

fn print(expression: Expression) -> Expression {
    match &expression {
        Expression::Integer(val) => println!("{}", val),
        Expression::Float(val) => println!("{}", val),
        Expression::Bool(val) => println!("{}", val),
        Expression::Array(arr) => {
            let type_ = arr.type_.clone();
            match type_ {
                Type::Array(type_) => match *type_ {
                    Type::Integer => {
                        let mut result = String::from("[");
                        for (i, element) in arr.elements.iter().enumerate() {
                            let value = match element {
                                Expression::Integer(val) => val.to_string(),
                                _ => panic!("Invalid type"),
                            };
                            result.push_str(&value);
                            if i != arr.elements.len() - 1 {
                                result.push_str(", ");
                            }
                        }
                        result.push_str("]");
                        println!("{}", result);
                    }
                    Type::Float => {
                        let mut result = String::from("[");
                        for (i, element) in arr.elements.iter().enumerate() {
                            let value = match element {
                                Expression::Float(val) => val.to_string(),
                                _ => panic!("Invalid type"),
                            };
                            result.push_str(&value);
                            if i != arr.elements.len() - 1 {
                                result.push_str(", ");
                            }
                        }
                        result.push_str("]");
                        println!("{}", result);
                    }
                    Type::Boolean => {
                        let mut result = String::from("[");
                        for (i, element) in arr.elements.iter().enumerate() {
                            let value = match element {
                                Expression::Bool(val) => val.to_string(),
                                _ => panic!("Invalid type"),
                            };
                            result.push_str(&value);
                            if i != arr.elements.len() - 1 {
                                result.push_str(", ");
                            }
                        }
                        result.push_str("]");
                        println!("{}", result);
                    }
                    _ => panic!("Invalid type"),
                },
                _ => panic!("Invalid type"),
            }
        }
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
        BinaryOperator::GreaterThan => Expression::Bool(left_val > right_val),
        BinaryOperator::LessThan => Expression::Bool(left_val < right_val),
        BinaryOperator::GreaterEqualThan => Expression::Bool(left_val >= right_val),
        BinaryOperator::LessEqualThan => Expression::Bool(left_val <= right_val),
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
        BinaryOperator::GreaterThan => Expression::Bool(left_val > right_val),
        BinaryOperator::LessThan => Expression::Bool(left_val < right_val),
        BinaryOperator::GreaterEqualThan => Expression::Bool(left_val >= right_val),
        BinaryOperator::LessEqualThan => Expression::Bool(left_val <= right_val),
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
        BinaryOperator::GreaterThan => Expression::Bool((left_val as f32) > right_val),
        BinaryOperator::LessThan => Expression::Bool((left_val as f32) < right_val),
        BinaryOperator::GreaterEqualThan => Expression::Bool((left_val as f32) >= right_val),
        BinaryOperator::LessEqualThan => Expression::Bool((left_val as f32) <= right_val),
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
        BinaryOperator::GreaterThan => Expression::Bool(left_val > (right_val as f32)),
        BinaryOperator::LessThan => Expression::Bool(left_val < (right_val as f32)),
        BinaryOperator::GreaterEqualThan => Expression::Bool(left_val >= (right_val as f32)),
        BinaryOperator::LessEqualThan => Expression::Bool(left_val <= (right_val as f32)),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn evaluate(expression: Expression, symbols: &HashMap<String, Declaration>) -> Expression {
    let mut current_expression = expression;
    let mut current_symbols = symbols.clone();

    loop {
        return match current_expression {
            Expression::Integer(_)
            | Expression::Float(_)
            | Expression::Bool(_)
            | Expression::Array(_) => current_expression,
            Expression::Identifier(identifier) => {
                let declaration = current_symbols.get(&identifier).unwrap();
                let expression = declaration.expression.clone();
                current_expression = *expression;
                continue;
            }
            Expression::FunctionCall(ref function_call) => match function_call.name.as_str() {
                "print" => print(evaluate(
                    function_call.arguments[0].clone(),
                    &current_symbols,
                )),
                "time" => time(),
                "sqrt" => {
                    let expression = evaluate(function_call.arguments[0].clone(), &current_symbols);
                    match expression {
                        Expression::Integer(val) => Expression::Float((val as f32).sqrt()),
                        Expression::Float(val) => Expression::Float(val.sqrt()),
                        _ => panic!("Invalid type"),
                    }
                }
                _ => {
                    // Get the function declaration
                    let declaration = current_symbols.get(&function_call.name).unwrap();
                    // The expression which we know to be a function declaration
                    let function_declaration_expression = (*declaration.expression).clone();

                    let function = match function_declaration_expression {
                        Expression::FunctionDeclaration(function) => function,
                        _ => panic!("Invalid function"),
                    };

                    for (i, argument) in function_call.arguments.iter().enumerate() {
                        let name = function.parameters[i].name.clone();
                        let type_ = function.parameters[i].type_.clone();
                        let expression = evaluate(argument.clone(), &current_symbols);
                        let declaration = Declaration {
                            name: name.clone(),
                            type_: type_.clone(),
                            expression: Box::new(expression),
                        };
                        current_symbols.insert(name.clone(), declaration);
                    }
                    current_expression = *function.expression;
                    continue;
                }
            },
            Expression::BinaryOperation(binary) => {
                let left = evaluate(*binary.left, &current_symbols);
                let right = evaluate(*binary.right, &current_symbols);
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
                    (Expression::Array(left_val), Expression::Array(right_val)) => match operator {
                        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
                        BinaryOperator::Add => {
                            let mut result = left_val.clone();
                            result.elements.extend(right_val.elements);
                            Expression::Array(result)
                        }
                        _ => panic!("Invalid operation"),
                    },
                    _ => panic!("Invalid types or operation"), // Handle other cases or operations
                };
            }

            Expression::Match(match_) => {
                let expression = evaluate(*match_.value, &current_symbols);
                for case in match_.patterns {
                    let left = evaluate(case.0, &current_symbols);
                    // TODO: perform this check in the parser
                    if discriminant(&expression) != discriminant(&left) {
                        panic!("Invalid types: {:?} and {:?}", expression, left);
                    } else if expression == left {
                        return evaluate(case.1, &current_symbols);
                    }
                }
                current_expression = *match_.default.unwrap();
                continue;
            }
            _ => panic!("Unknown expression {:?}", current_expression),
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
                    let declaration = match *declaration.expression {
                        Expression::FunctionDeclaration(_) => declaration,
                        _ => Declaration {
                            name: declaration.name.clone(),
                            type_: declaration.type_.clone(),
                            expression: Box::new(evaluate(*declaration.expression, &self.symbols)),
                        },
                    };
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

    #[test]
    fn test_array_operations() {
        let declare = "let array:[i32] = [1, 2, 3, 4, 5];
        let array2:[i32] = [6, 7, 8, 9, 10];
        let array3:[i32] = array + array2;
        print(array3);";
        let mut lexer = Lexer::new(declare);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.interpret(program);
    }

    #[test]
    fn test_string() {
        let declare = "let string:[u8] = \"Hello, World!\";";

        let mut lexer = Lexer::new(declare);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();
        vm.interpret(program);
    }

    #[test]
    fn tail_optimization() {
        let declare = "let increment:fn = (x:i32, i:i32):i32 => i ? {
    0 => x,
    increment(x + 1, i - 1)
};

print(increment(0, 20000));";
        let mut lexer = Lexer::new(declare);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.interpret(program);
    }
}
