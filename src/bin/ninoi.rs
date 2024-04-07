use std::env;
use std::fs;

use nino::lexer::tokenize;
use nino::parser::parse;
use nino::virtual_machine::VirtualMachine;

fn generate_error_location_display(input: &str, token: &nino::lexer::Token) -> String {
    let mut line_start = token.begin;
    while line_start > 0 && input.chars().nth(line_start).unwrap() != '\n' {
        line_start -= 1;
    }
    let mut line_end = token.end;
    while line_end < input.len() && input.chars().nth(line_end).unwrap() != '\n' {
        line_end += 1;
    }
    let line = input[line_start..line_end].to_string();

    let mut pointer = String::new();
    for _ in 0..(token.begin - line_start)-2 {
        pointer.push(' ');
    }
    for _ in token.begin..token.end+1 {
        pointer.push('^');
    }
    format!("{}\n{}", line, pointer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let tokens = tokenize(&input);

    let ast = match parse(&tokens) {
        Ok(ast) => ast,
        Err(e) => {
            match e.token {
                Some(token) => eprintln!(
                    "Parser error!\n{}\nHere: {}",
                    e.message,
                    generate_error_location_display(&input, &token)
                ),
                None => eprintln!("{}", e.message),
            }
            std::process::exit(1);
        }
    };

    let mut vm = VirtualMachine::new();
    vm.run(ast);
}
