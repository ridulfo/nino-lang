use std::env;
use std::fs;

use nino::lexer::tokenize;
use nino::parser::parse;
use nino::virtual_machine::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let tokens = tokenize(&input).into_iter().map(|t| t.kind).collect::<Vec<_>>();

    let ast = match parse(&tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };


    let mut vm = VirtualMachine::new();
    vm.run(ast);
}
