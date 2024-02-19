use nino::{lexer::tokenize, parser::{BinaryOperator, Expression}};
use std::env;
static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn fishify_expression(expression: Expression) -> String {
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut result = String::new();

    match expression {
        Expression::Identifier(identifier) => {
            result.push_str(&identifier);
        }

        Expression::Number(number) => {
            result.push_str(format!("{}_{}[{}]", number, count, number).as_str());
        }
        Expression::BinaryOperation(binary_operation) => {
            let operator = match binary_operation.operator {
                BinaryOperator::Add => "Add",
                BinaryOperator::Subtract => "Subtract",
                BinaryOperator::Multiply => "Multiply",
                BinaryOperator::Divide => "Divide",
                BinaryOperator::Modulo => "Modulo",
                _ => unimplemented!(),
            };

            // Add the counter to the node name to make it unique
            let node_name = format!("{}_{}[{}]", operator, count, operator);

            result.push_str(node_name.as_str());
            result.push_str(" --> ");
            result.push_str(&fishify_expression(*binary_operation.left));
            result.push_str("\n");

            result.push_str(node_name.as_str());
            result.push_str(" --> ");
            result.push_str(&fishify_expression(*binary_operation.right));
        }
        _ => unimplemented!(),
    };

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: mermaid <code>");
        return;
    }
    let code = &args[1];
    let mut chart = String::new();
    chart.push_str(&code);
    chart.push_str("\n");
    chart.push_str("```mermaid\nflowchart TD\n");

    let tokens = tokenize(code).into_iter().map(|t| t.kind).collect::<Vec<_>>();
    let mut parser = nino::parser::Parser::new(&tokens);
    let ast = parser.parse();

    let expression = ast.first().unwrap();
    match expression {
        nino::parser::Item::Expression(expression) => {
            chart.push_str(&fishify_expression(expression.clone()));
        }
        _ => {}
    }

    let chart = chart + "\n```";

    println!("{}", chart);
}
