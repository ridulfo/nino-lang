use std::{iter::Peekable, str::CharIndices};

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

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub begin: usize,
    pub end: usize,
}

fn parse_number(chars: &mut Peekable<CharIndices>) -> Token {
    let begin = chars.peek().unwrap().0;
    let mut end = begin;
    let mut string = String::new();
    while let Some((i, c)) = chars.peek() {
        match c {
            '0'..='9' | '.' => string.push(*c),
            _ => break,
        }
        end = *i;
        chars.next();
    }
    Token {
        kind: TokenKind::Number(string.parse::<f64>().unwrap()),
        begin,
        end,
    }
}

fn consume_whitespace(chars: &mut Peekable<CharIndices>) {
    while let Some(&(_, c)) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }
}

/// First token is a quote, so we consume it and then we consume all the
/// string until we find another quote.
fn parse_string(chars: &mut Peekable<CharIndices>) -> Token {
    let begin = chars.peek().unwrap().0;
    assert_eq!(chars.next().unwrap().1, '"');
    let mut end = begin;
    let mut string = String::new();
    while let Some(&(i, c)) = chars.peek() {
        if c == '"' {
            end = i;
            chars.next();
            break;
        }
        string.push(c);
        chars.next();
    }
    Token {
        kind: TokenKind::String(string),
        begin,
        end,
    }
}

/// Parses a type, which is a string of characters that are alphanumeric
fn parse_type(chars: &mut Peekable<CharIndices>) -> Token {
    let begin = chars.peek().unwrap().0;
    let mut end = begin;
    let mut string = String::new();
    while let Some(&(i, c)) = chars.peek() {
        match c {
            'a'..='z' | '0'..='9' | '_' | '[' | ']' => string.push(c),
            _ => break,
        }
        end = i;
        chars.next();
    }
    Token {
        kind: TokenKind::Type(string),
        begin,
        end,
    }
}

fn parse_word(chars: &mut Peekable<CharIndices>) -> Token {
    let begin = chars.peek().unwrap().0;
    let mut end = begin;
    let mut string = String::new();
    while let Some(&(i, s)) = chars.peek() {
        if !s.is_alphanumeric() && s != '_' {
            break;
        }
            end = i;
        string.push(s);
        chars.next();
    }

    let kind = match string.as_str() {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Function,
        "mod" => TokenKind::Modulus,
        "true" => TokenKind::Boolean(true),
        "false" => TokenKind::Boolean(false),
        _ => TokenKind::Identifier(string),
    };

    Token {
        kind,
        begin,
        end,
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars: Peekable<CharIndices> = input.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while chars.peek().is_some() {
        consume_whitespace(&mut chars); // Needed because early continue skips the bottom consume_whitespace
        let &(begin, c) = chars.peek().unwrap();

        let token = match c {
            'a'..='z' => {
                tokens.push(parse_word(&mut chars));
                continue;
            }
            '0'..='9' => {
                tokens.push(parse_number(&mut chars));
                continue;
            }
            '"' => {
                tokens.push(parse_string(&mut chars));
                continue;
            }
            ':' => {
                tokens.push(Token {
                    kind: TokenKind::Colon,
                    begin,
                    end: begin,
                });
                chars.next();
                consume_whitespace(&mut chars);
                tokens.push(parse_type(&mut chars));
                continue;
            }
            '!' => {
                let mut end = begin;
                chars.next();
                tokens.push(Token {
                    kind: match chars.peek().unwrap().1 {
                        '=' => {
                            end = chars.peek().unwrap().0;
                            chars.next(); // consume the '='
                            TokenKind::NotEqual
                        }
                        _ => TokenKind::Not,
                    },
                    begin,
                    end,
                });
                continue;
            }
            '=' => {
                let mut end = begin;
                chars.next(); // consume the '='
                let kind = match chars.peek().unwrap().1 {
                    '=' => {
                        end = chars.peek().unwrap().0;
                        chars.next(); // consume the '='
                        TokenKind::Equal
                    }
                    '>' => {
                        end = chars.peek().unwrap().0;
                        chars.next(); // consume the '>'
                        TokenKind::Arrow
                    }
                    _ => TokenKind::Assignment,
                };
                tokens.push(Token {
                    kind,
                    begin,
                    end,
                });
                continue;
            }
            '<' => {
                chars.next();
                let kind = match chars.peek().unwrap().1 {
                    '=' => {
                        chars.next();
                        TokenKind::LessEqualThan
                    }
                    _ => TokenKind::LessThan,
                };
                tokens.push(Token {
                    kind,
                    begin,
                    end: chars.peek().unwrap().0,
                });
                continue;
            }
            '>' => {
                chars.next();
                let kind = match chars.peek().unwrap().1 {
                    '=' => {
                        chars.next();
                        TokenKind::GreaterEqualThan
                    }
                    _ => TokenKind::GreaterThan,
                };
                tokens.push(Token {
                    kind,
                    begin,
                    end: chars.peek().unwrap().0,
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
                while let Some(&(_, c)) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
            _ => panic!("Unexpected character: {}", c),
        };
        tokens.push(Token {
            kind: token,
            begin,
            end: chars.peek().unwrap().0,
        });
        chars.next();
        consume_whitespace(&mut chars); // if the last character is a whitespace
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        begin: input.len()-1,
        end: input.len()-1,
    });

    tokens
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
        let mut chars = input.char_indices().peekable();
        let token = parse_number(&mut chars);
        assert_eq!(token, Token { kind: TokenKind::Number(123.0), begin: 0, end: 2 });
        assert_eq!(chars.next(), Some((3, ' ')));
        consume_whitespace(&mut chars);
        let token = parse_number(&mut chars);
        assert_eq!(token, Token { kind: TokenKind::Number(345.0), begin: 4, end: 6 });
    }

    #[test]
    fn test_parse_float() {
        let input = "123.456";
        let mut chars = input.char_indices().peekable();
        let token = parse_number(&mut chars);
        assert_eq!(token, Token { kind: TokenKind::Number(123.456), begin: 0, end: 6 });
    }

    #[test]
    fn test_consumes_whitespace() {
        let input = "   123";
        let chars = &mut input.char_indices().peekable();
        consume_whitespace(chars);
        let next = chars.next().unwrap();
        assert_eq!(next, (3, '1'));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello world\"";
        let token = parse_string(&mut input.char_indices().peekable());
        assert_eq!(token, Token { kind: TokenKind::String("hello world".to_string()), begin: 0, end: 12 });
    }

    #[test]
    fn test_parse_expression() {
        let input = "let x:num = 3;";
        let tokens = tokenize(input);

        assert_eq!(tokens.len(), 8);

        assert_eq!(tokens[0], Token { kind: TokenKind::Let, begin: 0, end: 2 });

        assert_eq!(tokens[1], Token { kind: TokenKind::Identifier("x".to_string()), begin: 4, end: 4 });
        
        assert_eq!(tokens[2], Token { kind: TokenKind::Colon, begin: 5, end: 5 });

        assert_eq!(tokens[3], Token { kind: TokenKind::Type("num".to_string()), begin: 6, end: 8 });

        assert_eq!(tokens[4], Token { kind: TokenKind::Assignment, begin: 10, end: 10 });

        assert_eq!(tokens[5], Token { kind: TokenKind::Number(3.0), begin: 12, end: 12 });

        assert_eq!(tokens[6], Token { kind: TokenKind::Semicolon, begin: 13, end: 13 });

        assert_eq!(tokens[7], Token { kind: TokenKind::EOF, begin: 13, end: 13 });
    }

    #[test]
    fn test_parse_equalities() {
        let input = "1 == 2 != 3 > 4 < 5 >= 6 <= 7";
        let tokens = tokenize(input);
        
        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0], Token { kind: TokenKind::Number(1.0), begin: 0, end: 0 });
        assert_eq!(tokens[1], Token { kind: TokenKind::Equal, begin: 2, end: 3 });
        assert_eq!(tokens[2], Token { kind: TokenKind::Number(2.0), begin: 5, end: 5 });
        assert_eq!(tokens[3], Token { kind: TokenKind::NotEqual, begin: 7, end: 8 });
        assert_eq!(tokens[4], Token { kind: TokenKind::Number(3.0), begin: 10, end: 10 });
        assert_eq!(tokens[5], Token { kind: TokenKind::GreaterThan, begin: 12, end: 13 });
        assert_eq!(tokens[6], Token { kind: TokenKind::Number(4.0), begin: 14, end: 14 });
        assert_eq!(tokens[7], Token { kind: TokenKind::LessThan, begin: 16, end: 17 });
        assert_eq!(tokens[8], Token { kind: TokenKind::Number(5.0), begin: 18, end: 18 });
        assert_eq!(tokens[9], Token { kind: TokenKind::GreaterEqualThan, begin: 20, end: 22 });
        assert_eq!(tokens[10], Token { kind: TokenKind::Number(6.0), begin: 23, end: 23 });
        assert_eq!(tokens[11], Token { kind: TokenKind::LessEqualThan, begin: 25, end: 27 });
        assert_eq!(tokens[12], Token { kind: TokenKind::Number(7.0), begin: 28, end: 28 });
        assert_eq!(tokens[13], Token { kind: TokenKind::EOF, begin: 28, end: 28 });
    }

    #[test]
    fn test_equality_expression() {
        let input = "let x:bool = 1+3>2 == 1;";
        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();
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
        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();
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

        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();

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
        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();

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
        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();

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

        let tokens = tokenize(input).into_iter().map(|t| t.kind).collect::<Vec<_>>();

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
