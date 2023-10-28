use std::collections::HashMap;

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
                            _ => unimplemented!("Unknown operator {:?}", operator),
                        }
                    }
                    (Expression::Float(left_val), Expression::Float(right_val)) => match operator {
                        BinaryOperator::Add => Expression::Float(left_val + right_val),
                        BinaryOperator::Subtract => Expression::Float(left_val - right_val),
                        BinaryOperator::Multiply => Expression::Float(left_val * right_val),
                        BinaryOperator::Divide => Expression::Float(left_val / right_val),
                        _ => unimplemented!("Unknown operator {:?}", operator),
                    },
                    (Expression::Integer(left_val), Expression::Float(right_val))
                    | (Expression::Float(right_val), Expression::Integer(left_val)) => {
                        match operator {
                            BinaryOperator::Add => Expression::Float((left_val as f32) + right_val),
                            BinaryOperator::Subtract => Expression::Float((left_val as f32) - right_val),
                            BinaryOperator::Multiply => Expression::Float((left_val as f32) * right_val),
                            BinaryOperator::Divide => Expression::Float((left_val as f32) / right_val),
                            _ => unimplemented!("Unknown operator {:?}", operator),
                        }
                    }
                    _ => panic!("Invalid types or operation"), // Handle other cases or operations
                }
            }
            _ => panic!("Unknown expression {:?}", expression),
        }
    }
    pub fn interpret(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        for statement in program {
            match statement {
                Item::Declaration(declaration) => {
                    self.symbols.insert(declaration.name.clone(), declaration);
                }
                Item::Expression(expression) => {
                    println!("Expression {:?}", self.evaluate(expression));
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let input = "3;";
        let mut vm = VirtualMachine::new();
        vm.interpret(input);
    }
}
