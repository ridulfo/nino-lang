mod builtins;

use std::mem::discriminant;
use crate::parser::{BinaryOperator, Declaration, Expression, Item};
use crate::scoped_symbols::ScopedSymbols;

use self::builtins::{time, print};

fn binary_float_float(left_val: f64, right_val: f64, operator: BinaryOperator) -> Expression {
    match operator {
        BinaryOperator::Add => Expression::Number(left_val + right_val),
        BinaryOperator::Subtract => Expression::Number(left_val - right_val),
        BinaryOperator::Multiply => Expression::Number(left_val * right_val),
        BinaryOperator::Divide => Expression::Number(left_val / right_val),
        BinaryOperator::Modulo => Expression::Number(left_val % right_val),
        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
        BinaryOperator::NotEqual => Expression::Bool(left_val != right_val),
        BinaryOperator::GreaterThan => Expression::Bool(left_val > right_val),
        BinaryOperator::LessThan => Expression::Bool(left_val < right_val),
        BinaryOperator::GreaterEqualThan => Expression::Bool(left_val >= right_val),
        BinaryOperator::LessEqualThan => Expression::Bool(left_val <= right_val),
        _ => unimplemented!("Unknown operator {:?}", operator),
    }
}

fn evaluate(expression: Expression, symbols: &ScopedSymbols) -> Expression {
    let mut current_expression = expression;
    let mut current_symbols = ScopedSymbols::with_parent(&symbols);

    loop {
        return match current_expression {
            Expression::Number(..) | Expression::Bool(..) | Expression::Array(..) => {
                current_expression
            }
            Expression::Identifier(identifier) => {
                let declaration = current_symbols.get(&identifier).unwrap();
                let expression = declaration.expression.clone();
                current_expression = *expression;
                continue;
            }
            Expression::FunctionCall(ref function_call) => match function_call.name.as_str() {
                "print" => print(
                    evaluate(function_call.arguments[0].clone(), &current_symbols),
                    "\n",
                ),
                "time" => time(),
                "sqrt" => {
                    let expression = evaluate(function_call.arguments[0].clone(), &current_symbols);
                    match expression {
                        Expression::Number(val) => Expression::Number(val.sqrt()),
                        _ => panic!("Invalid type"),
                    }
                }
                _ => {
                    // Get the function declaration
                    let declaration = current_symbols.get(&function_call.name).unwrap();
                    // The expression which we know to be a function declaration
                    let function_declaration_expression = *declaration.expression;

                    let function = match function_declaration_expression {
                        Expression::FunctionDeclaration(function) => function,
                        _ => panic!("Invalid function"),
                    };

                    // First the arguments need to be evaluated
                    let mut temp = vec![];
                    for (i, argument) in function_call.arguments.iter().enumerate() {
                        let name = &function.parameters[i].name;
                        let type_ = &function.parameters[i].type_;
                        let expression = evaluate(argument.clone(), &current_symbols);
                        let declaration = Declaration {
                            name: name.clone(),
                            type_: type_.clone(),
                            expression: Box::new(expression),
                        };
                        temp.push(declaration);
                    }
                    // Then the arguments can be inserted into the current symbols
                    for (i, argument) in temp.iter().enumerate() {
                        current_symbols
                            .insert(function.parameters[i].name.clone(), argument.clone());
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
                    (Expression::Number(left_val), Expression::Number(right_val)) => {
                        binary_float_float(left_val, right_val, operator)
                    }
                    (
                        Expression::Array(left_type, left_val),
                        Expression::Array(right_type, right_val),
                    ) => match operator {
                        BinaryOperator::Equal => Expression::Bool(left_val == right_val),
                        BinaryOperator::Add => {
                            assert_eq!(left_type, right_type, "Invalid types");
                            let mut result = left_val.clone();
                            result.extend(right_val);
                            Expression::Array(left_type, result)
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
                if match_.default.is_some() {
                    current_expression = *match_.default.unwrap();
                } else {
                    panic!("No matching pattern found. You should add a default pattern.");
                }
                continue;
            }
            _ => panic!("Unknown expression {:?}", current_expression),
        };
    }
}

pub struct VirtualMachine<'a> {
    pub symbols: ScopedSymbols<'a>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new() -> VirtualMachine<'a> {
        VirtualMachine {
            symbols: ScopedSymbols::new(),
        }
    }

    pub fn evaluate(&mut self, expression: Expression) -> Expression {
        evaluate(expression, &self.symbols)
    }

    pub fn run(&mut self, program: Vec<Item>) {
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
    use crate::parser::BinaryOperation;

    use super::*;

    /// Testing a evalutation of a simple expression (1 + 2)
    #[test]
    fn test_expression() {
        // Test adding two integers
        let expression = Expression::BinaryOperation(BinaryOperation {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::Number(1.0)),
            right: Box::new(Expression::Number(2.0)),
        });

        let mut vm = VirtualMachine::new();
        let result = vm.evaluate(expression);

        assert_eq!(result, Expression::Number(3.0));
    }
}
