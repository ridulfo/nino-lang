use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::lexer::Lexer;
use crate::parser::{BinaryOperator, Declaration, Expression, Item, Parser};

pub struct VirtualMachine {
    symbols: HashMap<String, Declaration>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            symbols: HashMap::new(),
        }
    }
    fn evaluate(&self, expression: Expression) -> Expression {
        match expression {
            Expression::Integer(_) | Expression::Float(_) | Expression::Bool(_) => expression,
            Expression::Identifier(identifier) => {
                let declaration = self.symbols.get(&identifier).unwrap();
                let expression = declaration.expression.clone();
                self.evaluate(*expression)
            }
            Expression::FunctionCall(function_call) => {
                match function_call.name.as_str() {
                    "print" => {
                        let arg = self.evaluate(function_call.arguments[0].clone());
                        match arg {
                            Expression::Integer(val) => println!("{}", val),
                            Expression::Float(val) => println!("{}", val),
                            Expression::Bool(val) => println!("{}", val),
                            _ => panic!("Invalid type"),
                        }
                        return arg
                    }
                    "time" => {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        let in_ms = since_the_epoch.as_millis();
                        return Expression::Float(in_ms as f32);
                    }
                    _ => {
                        // Get the function declaration
                        let declaration = self.symbols.get(&function_call.name).unwrap();
                        // The expression which we know to be a function declaration
                        let function_declaration_expression = (*declaration.expression).clone();

                        let function = match function_declaration_expression {
                            Expression::FunctionDeclaration(function) => function,
                            _ => panic!("Invalid function"),
                        };

                        let mut local_symbols: HashMap<String, Declaration> = self.symbols.clone();
                        for (i, argument) in function_call.arguments.iter().enumerate() {
                            let name = function.parameters[i].name.clone();
                            let type_ = function.parameters[i].type_.clone();
                            let expression = self.evaluate(argument.clone());
                            let declaration = Declaration {
                                name: name.clone(),
                                type_: type_.clone(),
                                expression: Box::new(expression).clone(),
                            };
                            local_symbols.insert(name.clone(), declaration);
                        }

                        let vm = VirtualMachine {
                            symbols: local_symbols,
                        };
                        return vm.evaluate(*function.expression);
                    }
                }
                Expression::Integer(0)
            }
            Expression::BinaryOperation(binary) => {
                let left = self.evaluate(*binary.left);
                let right = self.evaluate(*binary.right);
                let operator = binary.operator;
                match (left, right) {
                    // Perform operations based on the types and the operator
                    (Expression::Integer(left_val), Expression::Integer(right_val)) => {
                        match operator {
                            BinaryOperator::Add => Expression::Integer(left_val + right_val),
                            BinaryOperator::Subtract => Expression::Integer(left_val - right_val),
                            BinaryOperator::Multiply => Expression::Integer(left_val * right_val),
                            BinaryOperator::Divide => {
                                Expression::Float((left_val as f32) / (right_val as f32))
                            } // Implement other operations as needed
                            BinaryOperator::Modulo => Expression::Integer(left_val % right_val),
                            BinaryOperator::Equal => Expression::Bool(left_val == right_val),
                            _ => unimplemented!("Unknown operator {:?}", operator),
                        }
                    }
                    (Expression::Float(left_val), Expression::Float(right_val)) => match operator {
                        BinaryOperator::Add => Expression::Float(left_val + right_val),
                        BinaryOperator::Subtract => Expression::Float(left_val - right_val),
                        BinaryOperator::Multiply => Expression::Float(left_val * right_val),
                        BinaryOperator::Divide => Expression::Float(left_val / right_val),
                        BinaryOperator::Modulo => Expression::Float(left_val % right_val),
                        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
                        _ => unimplemented!("Unknown operator {:?}", operator),
                    },
                    (Expression::Integer(left_val), Expression::Float(right_val)) => match operator
                    {
                        BinaryOperator::Add => Expression::Float((left_val as f32) + right_val),
                        BinaryOperator::Subtract => {
                            Expression::Float((left_val as f32) - right_val)
                        }
                        BinaryOperator::Multiply => {
                            Expression::Float((left_val as f32) * right_val)
                        }
                        BinaryOperator::Divide => Expression::Float((left_val as f32) / right_val),
                        BinaryOperator::Modulo => Expression::Float((left_val as f32) % right_val),
                        BinaryOperator::Equal => Expression::Bool(left_val == (right_val as i32)),
                        _ => unimplemented!("Unknown operator {:?}", operator),
                    },
                    (Expression::Float(left_val), Expression::Integer(right_val)) => match operator
                    {
                        BinaryOperator::Add => Expression::Float(left_val + (right_val as f32)),
                        BinaryOperator::Subtract => {
                            Expression::Float(left_val - (right_val as f32))
                        }
                        BinaryOperator::Multiply => {
                            Expression::Float(left_val * (right_val as f32))
                        }
                        BinaryOperator::Divide => Expression::Float(left_val / (right_val as f32)),
                        BinaryOperator::Modulo => Expression::Float(left_val % (right_val as f32)),
                        BinaryOperator::Equal => Expression::Bool(left_val == (right_val as f32)),
                        _ => unimplemented!("Unknown operator {:?}", operator),
                    },
                    _ => panic!("Invalid types or operation"), // Handle other cases or operations
                }
            }
            Expression::Match(match_) => {
                let expression = self.evaluate(*match_.value);
                for case in match_.patterns {
                    let left = self.evaluate(case.0);
                    if expression == left {
                        return self.evaluate(case.1);
                    }
                }
                self.evaluate(*match_.default.unwrap())
            }
            _ => panic!("Unknown expression {:?}", expression),
        }
    }

    pub fn interpret(&mut self, program: Vec<Item>) {
        for statement in program {
            match statement {
                Item::Declaration(declaration) => {
                    self.symbols.insert(declaration.name.clone(), declaration);
                }
                Item::Expression(expression) => {
                    self.evaluate(expression);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::parser::BinaryOperation;

    use super::*;

    #[test]
    fn test_expression() {
        // Test adding two integers
        let expression = Expression::BinaryOperation(BinaryOperation {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::Integer(1)),
            right: Box::new(Expression::Integer(2)),
        });

        let vm = VirtualMachine::new();
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
