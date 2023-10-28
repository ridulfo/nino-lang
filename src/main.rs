mod lexer;
mod parser;
use lexer::Lexer;

fn main() {
    let input = "let x:i32 = 3;";
    let mut lexer = Lexer::new(input);
    println!("{:?}", lexer.tokenize());
}
