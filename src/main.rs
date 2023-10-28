use virtual_machine::VirtualMachine;

mod lexer;
mod parser;
mod virtual_machine;

fn main() {
    let input = "let x:i32 = 3.1;
    let y:i32=2;
    print(x+y/2);";
    let mut vm = VirtualMachine::new();
    vm.interpret(input);
}
