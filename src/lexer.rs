use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    Let,

    // types and values
    Type(String),
    Character(u8),
    Number(f64),
    String(String),
    Boolean(bool),
    Function,

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
    // Quote, // Is this needed?
    Pipe,

    // identifiers
    Identifier(String),

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
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' | '.' => string.push(*c),
            _ => break,
        }
        chars.next();
    }
    TokenKind::Number(string.parse::<f64>().unwrap())
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
            chars.next();
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
            'a'..='z' | '0'..='9' | '_' | '[' | ']' => string.push(c),
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
        "fn" => TokenKind::Function,
        "mod" => TokenKind::Modulus,
        "true" => TokenKind::Boolean(true),
        "false" => TokenKind::Boolean(false),
        _ => TokenKind::Identifier(string),
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

    pub fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = Vec::new();

        while self.chars.peek().is_some() {
            consume_whitespace(&mut self.chars); // Needed because early continue skips the bottom consume_whitespace
            let c = self.chars.peek().unwrap();

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
                    tokens.push(match self.chars.peek() {
                        Some('=') => {
                            self.chars.next(); // consume the '='
                            TokenKind::NotEqual
                        }
                        _ => TokenKind::Not,
                    });
                    continue;
                }
                '=' => {
                    self.chars.next(); // consume the '='
                    tokens.push(match self.chars.peek() {
                        Some('=') => {
                            self.chars.next(); // consume the '='
                            TokenKind::Equal
                        }
                        Some('>') => {
                            self.chars.next(); // consume the '>'
                            TokenKind::Arrow
                        }
                        _ => TokenKind::Assignment,
                    });
                    continue;
                }
                '<' => {
                    self.chars.next();
                    tokens.push(match self.chars.peek() {
                        Some('=') => {
                            self.chars.next();
                            TokenKind::LessEqualThan
                        }
                        _ => TokenKind::LessThan,
                    });
                    continue;
                }
                '>' => {
                    self.chars.next();
                    tokens.push(match self.chars.peek() {
                        Some('=') => {
                            self.chars.next();
                            TokenKind::GreaterEqualThan
                        }
                        _ => TokenKind::GreaterThan,
                    });
                    continue;
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
                '?' => TokenKind::Question,
                '#' => {
                    while let Some(&c) = self.chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.chars.next();
                    }
                    continue;
                }
                _ => panic!("Unexpected character: {}", c),
            };
            tokens.push(token);
            self.chars.next();
            consume_whitespace(&mut self.chars); // if the last character is a whitespace
        }
        tokens.push(TokenKind::EOF);

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_tokens(actual: Vec<TokenKind>, expected: Vec<TokenKind>) {
        for (i, (actual, expected)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                actual, expected,
                "Expected {:?} but got {:?} at index {}",
                expected, actual, i
            );
        }
        assert_eq!(actual.len(), expected.len());
    }

    #[test]
    fn test_parse_number() {
        let input = "123 345";
        let chars = &mut input.chars().peekable();
        let token = parse_number(chars);
        assert_eq!(token, TokenKind::Number(123.0));
        assert_eq!(chars.next(), Some(' '));
    }

    #[test]
    fn test_parse_float() {
        let input = "123.456";
        let token = parse_number(&mut input.chars().peekable());
        assert_eq!(token, TokenKind::Number(123.456));
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
        let input = "let x:num = 3;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::Assignment,
                TokenKind::Number(3.0),
                TokenKind::Semicolon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_parse_equalities() {
        let input = "1 == 2 != 3 > 4 < 5 >= 6 <= 7";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Number(1.0),
                TokenKind::Equal,
                TokenKind::Number(2.0),
                TokenKind::NotEqual,
                TokenKind::Number(3.0),
                TokenKind::GreaterThan,
                TokenKind::Number(4.0),
                TokenKind::LessThan,
                TokenKind::Number(5.0),
                TokenKind::GreaterEqualThan,
                TokenKind::Number(6.0),
                TokenKind::LessEqualThan,
                TokenKind::Number(7.0),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_equality_expression() {
        let input = "let x:bool = 1+3>2 == 1;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("bool".to_string()),
                TokenKind::Assignment,
                TokenKind::Number(1.0),
                TokenKind::Addition,
                TokenKind::Number(3.0),
                TokenKind::GreaterThan,
                TokenKind::Number(2.0),
                TokenKind::Equal,
                TokenKind::Number(1.0),
                TokenKind::Semicolon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_array() {
        let input = "let x:[num] = [1,2,3];";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("[num]".to_string()),
                TokenKind::Assignment,
                TokenKind::LeftBracket,
                TokenKind::Number(1.0),
                TokenKind::Comma,
                TokenKind::Number(2.0),
                TokenKind::Comma,
                TokenKind::Number(3.0),
                TokenKind::RightBracket,
                TokenKind::Semicolon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_comment() {
        let input = "let x:num = 1; # This is a comment
let y:num = 2; # This is another comment";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        compare_tokens(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::Assignment,
                TokenKind::Number(1.0),
                TokenKind::Semicolon,
                TokenKind::Let,
                TokenKind::Identifier("y".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::Assignment,
                TokenKind::Number(2.0),
                TokenKind::Semicolon,
                TokenKind::EOF,
            ],
        );
    }

    #[test]
    fn test_match_expression() {
        let input = "let x:num = 1 ? {
    1 => 2,
    2 => 3,
    4
};";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::Assignment,
                TokenKind::Number(1.0),
                TokenKind::Question,
                TokenKind::LeftBrace,
                TokenKind::Number(1.0),
                TokenKind::Arrow,
                TokenKind::Number(2.0),
                TokenKind::Comma,
                TokenKind::Number(2.0),
                TokenKind::Arrow,
                TokenKind::Number(3.0),
                TokenKind::Comma,
                TokenKind::Number(4.0),
                TokenKind::RightBrace,
                TokenKind::Semicolon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_string() {
        let input = "let x:[char] = \"hello world\";";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        compare_tokens(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("[char]".to_string()),
                TokenKind::Assignment,
                TokenKind::String("hello world".to_string()),
                TokenKind::Semicolon,
                TokenKind::EOF,
            ],
        );
    }

    #[test]
    fn test_is_prime() {
        let input = "let is_prime:fn = (x:num):bool =>
    | let sqrt_x:f32 = sqrt(x);
    | let sqrt_x_int:num = floor(sqrt_x);
    => true ? {
        x==1 => false,
        x==2 => true,
        x mod 2 == 0 => false,
        true => is_prime_helper(x, 3, sqrt_x_int)
    };";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        compare_tokens(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("is_prime".to_string()),
                TokenKind::Colon,
                TokenKind::Type("fn".to_string()),
                TokenKind::Assignment,
                TokenKind::LeftParen,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::RightParen,
                TokenKind::Colon,
                TokenKind::Type("bool".to_string()),
                TokenKind::Arrow,
                TokenKind::Pipe,
                TokenKind::Let,
                TokenKind::Identifier("sqrt_x".to_string()),
                TokenKind::Colon,
                TokenKind::Type("f32".to_string()),
                TokenKind::Assignment,
                TokenKind::Identifier("sqrt".to_string()),
                TokenKind::LeftParen,
                TokenKind::Identifier("x".to_string()),
                TokenKind::RightParen,
                TokenKind::Semicolon,
                TokenKind::Pipe,
                TokenKind::Let,
                TokenKind::Identifier("sqrt_x_int".to_string()),
                TokenKind::Colon,
                TokenKind::Type("num".to_string()),
                TokenKind::Assignment,
                TokenKind::Identifier("floor".to_string()),
                TokenKind::LeftParen,
                TokenKind::Identifier("sqrt_x".to_string()),
                TokenKind::RightParen,
                TokenKind::Semicolon,
                TokenKind::Arrow,
                TokenKind::Boolean(true),
                TokenKind::Question,
                TokenKind::LeftBrace,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Equal,
                TokenKind::Number(1.0),
                TokenKind::Arrow,
                TokenKind::Boolean(false),
                TokenKind::Comma,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Equal,
                TokenKind::Number(2.0),
                TokenKind::Arrow,
                TokenKind::Boolean(true),
                TokenKind::Comma,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Modulus,
                TokenKind::Number(2.0),
                TokenKind::Equal,
                TokenKind::Number(0.0),
                TokenKind::Arrow,
                TokenKind::Boolean(false),
                TokenKind::Comma,
                TokenKind::Boolean(true),
                TokenKind::Arrow,
                TokenKind::Identifier("is_prime_helper".to_string()),
                TokenKind::LeftParen,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Comma,
                TokenKind::Number(3.0),
                TokenKind::Comma,
                TokenKind::Identifier("sqrt_x_int".to_string()),
                TokenKind::RightParen,
                TokenKind::RightBrace,
                TokenKind::Semicolon,
                TokenKind::EOF,
            ],
        );
    }
}
