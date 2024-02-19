use std::mem::discriminant;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{BinaryOperator, Declaration, Expression, Item};
use crate::scoped_symbols::ScopedSymbols;

fn print(expression: Expression, end: &str) -> Expression {
    match &expression {
        Expression::Char(val) => print!("{}{}", *val as char, end),
        Expression::Number(val) => print!("{}{}", val, end),
        Expression::Bool(val) => print!("{}{}", val, end),
        Expression::Array(type_, val) => {
            let is_string = type_ == &crate::parser::Type::Char;
            if !is_string {
                print!("[");
            }
            for (i, item) in val.iter().enumerate() {
                if i != 0 && !is_string {
                    print!(", ");
                }
                print(item.clone(), "");
            }
            if !is_string {
                print!("]");
            }
            print!("{}", end);
        }
        _ => print!("{:?}", expression),
    }
    return expression;
}

fn time() -> Expression {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    return Expression::Number(in_ms as f64);
}

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
                current_expression = *match_.default.unwrap();
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
    use crate::{
        lexer::tokenize,
        parser::{BinaryOperation, Parser, Type},
    };

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

    /// Testing a declaration of factorial function and calling it
    #[test]
    fn test_recursion() {
        let declare = "let factorial:fn = (n:num):num => n ? {
    0 => 1,
    n * factorial(n - 1)
};
        ";
        let tokens = tokenize(declare)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let declaration_ast = parser.parse();

        let input = "let result:num = factorial(5);";
        let tokens = tokenize(input)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.run(declaration_ast);
        vm.run(expression);
        let result = *vm.symbols.get("result").unwrap().expression.clone();
        assert_eq!(result, Expression::Number(120.0));

        let function = vm.symbols.get("factorial").unwrap();
        assert_eq!(function.name, "factorial");
        assert_eq!(function.type_, Type::Function);
    }

    /// Testing decalration of arrays and concatenation
    #[test]
    fn test_array_operations() {
        let declare = "let array:[num] = [1, 2, 3, 4, 5];
        let array2:[num] = [6, 7, 8, 9, 10];
        let array3:[num] = array + array2;";

        let tokens = tokenize(declare)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.run(program);

        let result = *vm.symbols.get("array3").unwrap().expression.clone();
        assert_eq!(
            result,
            Expression::Array(
                Type::Number,
                vec![
                    Expression::Number(1.0),
                    Expression::Number(2.0),
                    Expression::Number(3.0),
                    Expression::Number(4.0),
                    Expression::Number(5.0),
                    Expression::Number(6.0),
                    Expression::Number(7.0),
                    Expression::Number(8.0),
                    Expression::Number(9.0),
                    Expression::Number(10.0)
                ]
            )
        );
    }

    /// Testing declaration of a string and concatenation
    #[test]
    fn test_string() {
        let declare = "let string:[char] = \"Hello\" + \", World!\";";

        let tokens = tokenize(declare)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();
        vm.run(program);

        let result = *vm.symbols.get("string").unwrap().expression.clone();
        assert_eq!(
            result,
            Expression::Array(
                Type::Char,
                vec![
                    Expression::Char(b'H'),
                    Expression::Char(b'e'),
                    Expression::Char(b'l'),
                    Expression::Char(b'l'),
                    Expression::Char(b'o'),
                    Expression::Char(b','),
                    Expression::Char(b' '),
                    Expression::Char(b'W'),
                    Expression::Char(b'o'),
                    Expression::Char(b'r'),
                    Expression::Char(b'l'),
                    Expression::Char(b'd'),
                    Expression::Char(b'!')
                ]
            )
        );
    }

    /// Testing tail call optimization
    #[test]
    fn tail_optimization() {
        let declare = "let increment:fn = (x:num, i:num):num => i ? {
    0 => x,
    increment(x + 1, i - 1)
};
let incremented:num = increment(0, 20000);";
        let tokens = tokenize(declare)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.run(program);

        let result = *vm.symbols.get("incremented").unwrap().expression.clone();
        assert_eq!(result, Expression::Number(20000.0));
    }

    /// Testing scope
    #[test]
    fn test_scope() {
        let declare = "let x:num = 0;
let func:fn = (x:num):num=>x+1;
func(1);
";

        let tokens = tokenize(declare)
            .into_iter()
            .map(|t| t.kind)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse();

        let mut vm = VirtualMachine::new();

        vm.run(program);

        let result = *vm.symbols.get("x").unwrap().expression.clone();
        assert_eq!(result, Expression::Number(0.0));
    }
}
