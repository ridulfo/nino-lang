use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    Let,

    // types and values
    Type(String),
    Integer(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Function,

    // builtins
    Print,

    // separators
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Quote,
    Pipe,

    // identifiers
    Identifier,

    // operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Not,
    Modulus,

    // don't know what to call these
    Assignment,
    Arrow,
    Question,

    // equality
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,

    // end of file
    EOF,
}

fn parse_number(chars: &mut Peekable<Chars>) -> TokenKind {
    let mut string = String::new();
    let mut seen_dot = false;
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => {
                string.push(*c);
            }
            '.' => {
                if seen_dot {
                    panic!("Numbers can only have one dot.");
                }
                seen_dot = true;
                string.push(*c);
            }
            _ => break,
        }
        chars.next();
    }

    if seen_dot {
        TokenKind::Float(string.parse::<f32>().unwrap())
    } else {
        TokenKind::Integer(string.parse::<i32>().unwrap())
    }
}

fn consume_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
}

/// First token is a quote, so we consume it and then we consume all the
/// string until we find another quote.
fn parse_string(chars: &mut Peekable<Chars>) -> TokenKind {
    assert_eq!(chars.next(), Some('"'));
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        if c == '"' {
            break;
        }
        string.push(c);
        chars.next();
    }
    TokenKind::String(string)
}

/// Parses a type, which is a string of characters that are alphanumeric
fn parse_type(chars: &mut Peekable<Chars>) -> TokenKind {
    let mut string = String::new();
    while let Some(&c) = chars.peek() {
        match c {
            'a'..='z' | '0'..='9' | '_' => string.push(c),
            _ => break,
        }
        chars.next();
    }
    TokenKind::Type(string)
}

fn parse_word(chars: &mut Peekable<Chars>) -> TokenKind {
    let mut string = String::new();
    while let Some(&s) = chars.peek() {
        if !s.is_alphanumeric() && s != '_' {
            break;
        }
        string.push(s);
        chars.next();
    }
    match string.as_str() {
        "let" => TokenKind::Let,
        "print" => TokenKind::Print,
        "fn" => TokenKind::Function,
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),
        _ => TokenKind::Identifier,
    }
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    // All the parsing and consuming functions will take a string that starts at
    // the location that their pattern starts and will consume the characters up
    // to but NOT INCLUDING the last one in the pattern. This is because in the
    // main match statement we follow it up with a next().

    pub fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = Vec::new();
        // consume whitespace before parsing because `c` will won't change if
        // whitespace is consumed after it is peeked.
        while !self.chars.peek().is_none() {
            consume_whitespace(&mut self.chars);
            let &c = self.chars.peek().unwrap();

            let token = match c {
                'a'..='z' => {
                    tokens.push(parse_word(&mut self.chars));
                    continue;
                }
                '0'..='9' => {
                    tokens.push(parse_number(&mut self.chars));
                    continue;
                }
                '"' => {
                    tokens.push(parse_string(&mut self.chars));
                    continue;
                }
                ':' => {
                    tokens.push(TokenKind::Colon);
                    self.chars.next();
                    consume_whitespace(&mut self.chars);
                    tokens.push(parse_type(&mut self.chars));
                    continue;
                }
                '!' => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('=') => TokenKind::NotEqual,
                        _ => TokenKind::Not,
                    }
                }
                '=' => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('=') => TokenKind::Equal,
                        Some('>') => TokenKind::Arrow,
                        _ => TokenKind::Assignment,
                    }
                }
                '<' => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('=') => TokenKind::LessEqualThan,
                        _ => TokenKind::LessThan,
                    }
                }
                '>' => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('=') => TokenKind::GreaterEqualThan,
                        _ => TokenKind::GreaterThan,
                    }
                }
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ',' => TokenKind::Comma,
                ';' => TokenKind::Semicolon,
                '|' => TokenKind::Pipe,
                '+' => TokenKind::Addition,
                '-' => TokenKind::Subtraction,
                '*' => TokenKind::Multiplication,
                '/' => TokenKind::Division,
                '%' => TokenKind::Modulus,
                '?' => TokenKind::Question,
                _ => panic!("Unexpected character: {}", c),
            };
            tokens.push(token);
            self.chars.next();
        }
        tokens.push(TokenKind::EOF);

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let input = "123 345";
        let chars = &mut input.chars().peekable();
        let token = parse_number(chars);
        assert_eq!(token, TokenKind::Integer(123));
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_parse_float() {
        let input = "123.456";
        let token = parse_number(&mut input.chars().peekable());
        assert_eq!(token, TokenKind::Float(123.456));
    }

    #[test]
    fn test_consumes_whitespace() {
        let input = "   123";
        let chars = &mut input.chars().peekable();
        consume_whitespace(chars);
        assert_eq!(chars.next(), Some('1'));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello world\"";
        let token = parse_string(&mut input.chars().peekable());
        assert_eq!(token, TokenKind::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_expression() {
        let input = "let x:i32 = 3;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Colon,
                TokenKind::Type("i32".to_string()),
                TokenKind::Assignment,
                TokenKind::Integer(3),
                TokenKind::Semicolon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_parse_equalities(){
        let input = "1 == 2 != 3 > 4 < 5 >= 6 <= 7";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Integer(1),
                TokenKind::Equal,
                TokenKind::Integer(2),
                TokenKind::NotEqual,
                TokenKind::Integer(3),
                TokenKind::GreaterThan,
                TokenKind::Integer(4),
                TokenKind::LessThan,
                TokenKind::Integer(5),
                TokenKind::GreaterEqualThan,
                TokenKind::Integer(6),
                TokenKind::LessEqualThan,
                TokenKind::Integer(7),
                TokenKind::EOF,
            ]
        );
    }
}

fn main() {
    let input = "let x:i32 = 3;";
    let mut lexer = Lexer::new(input);
    println!("{:?}", lexer.tokenize());
}
