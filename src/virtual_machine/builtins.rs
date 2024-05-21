use core::panic;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::Expression;

pub fn print(expression: Expression, end: &str) -> Expression {
    match &expression {
        Expression::Char(val) => print!("{}{}", *val as char, end),
        Expression::Number(val) => print!("{}{}", val, end),
        Expression::Bool(val) => print!("{}{}", val, end),
        Expression::Array(type_, val) => {
            let is_string = type_ == &crate::parser::Type::Char;

            if is_string {
                let collect: String = val
                    .iter()
                    .map(|x| match x {
                        Expression::Char(val) => *val as char,
                        _ => panic!("Cannot convert {:?} to string", x),
                    })
                    .collect();
                print!("{}", collect);
            } else {
                print!("[");
                for (i, item) in val.iter().enumerate() {
                    if i != 0 && !is_string {
                        print!(", ");
                    }
                    print(item.clone(), "");
                }
                print!("]");
            }
            print!("{}", end);
        }
        _ => print!("{:?}", expression),
    }
    return expression;
}

pub fn debug_print(expression: Expression) -> Expression {
    println!("{:#?}", expression);
    return expression;
}

pub fn time() -> Expression {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    return Expression::Number(in_ms as f64);
}

pub fn head(expression: &Expression) -> &Expression {
    let array = match expression {
        Expression::Array(.., array) => array,
        _ => panic!("Cannot take head of {:?}", expression),
    };

    match array.first() {
        Some(expression) => expression,
        None => &Expression::Bool(false),
    }
}

pub fn tail(expression: &Expression) -> Expression {
    let (type_, array) = match expression {
        Expression::Array(type_, array) => (type_, array),
        _ => panic!("Cannot take tail of {:?}", expression),
    };
    let mut array = array.clone();
    if array.len() > 0 {
        array.remove(0);
    }
    Expression::Array(type_.clone(), array)
}

pub fn last(expression: &Expression) -> &Expression {
    let array = match expression {
        Expression::Array(.., array) => array,
        _ => panic!("Cannot take last of {:?}", expression),
    };
    match array.last() {
        Some(expression) => expression,
        None => &Expression::Bool(false),
    }
}

pub fn len(expression: &Expression) -> Expression {
    let array = match expression {
        Expression::Array(.., array) => array,
        _ => panic!("Cannot take first of {:?}", expression),
    };
    Expression::Number(array.len() as f64)
}
