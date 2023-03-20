use serde::{Deserialize, Serialize};

use crate::token::Token;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    finished: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, pos: 0, finished: false }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn read_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(ch) = self.next_char() {
            if !test(ch) {
                break;
            }
            result.push(ch);
            self.pos += ch.len_utf8();
        }
        result
    }
    fn read_number(&mut self) -> Token {
        // numVal: [0-9]+(\.[0-9]+)?
        // num: numVal | numVal [eE] [+-]? numVal | hexVal | octVal | binVal
        // hexVal: 0[xX] [0-9a-fA-F]+
        // octVal: 0[oO] [0-7]+
        // binVal: 0[bB] [01]+
        // so we can consume until we hit a non number
        // we can parse the number to int, float, hex, oct, bin in parser
        let s = self.read_while(|ch| {
            ch.is_ascii_digit()
                || ch == '.'
                || ch == 'e'
                || ch == 'x'
                || ch == 'o'
                || ch == 'b'
                || ch == '+'
                || ch == '-'
        });
        Token::NumberLiteral(s)
    }

    fn read_string(&mut self) -> Token {
        let s = self.read_while(|ch| ch != '"');
        self.pos += 1; // Skip the closing "
        Token::StringLiteral(s)
    }
    fn read_char(&mut self) -> Token {
        let s = self.read_while(|ch| ch != '\'' && ch != '\n');
        Token::CharLiteral(s.chars().next().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let s = self.read_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');
        Token::Identifier(s)
    }

    fn read_whitespace(&mut self) -> Token {
        // ifis newline
        let newline = self.read_while(|ch| ['\r', '\n', '\u{A0}'].contains(&ch));
        if newline.len() > 0 {
            return Token::NewLine(newline.len());
        }
        let white_space = self.read_while(|ch| ch.is_whitespace());
        if white_space.len() > 0 {
            return Token::WhiteSpace(white_space.len());
        }
        Token::WhiteSpace(0)
    }

    fn read_comment(&mut self) -> Token {
        self.pos += 2; // Skip "//"
        let comment = self.read_while(|ch| ch != '\n');
        Token::Comment(comment)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    

    fn next(&mut self) -> Option<Token> {
        let white_space = self.read_whitespace();
        if white_space != Token::WhiteSpace(0) {
            return Some(white_space);
        }
        match self.next_char() {
            None => {
                if self.finished {
                    return None;
                }
                self.finished = true;
                return Some(Token::EndOfFile);
            },
            Some(ch) => match ch {

                '{' => {
                    self.pos += 1;
                    Some(Token::CurlyBraceOpen)
                }
                '}' => {
                    self.pos += 1;
                    Some(Token::CurlyBraceClose)
                }
                '(' => {
                    self.pos += 1;
                    Some(Token::ParenOpen)
                }
                ')' => {
                    self.pos += 1;
                    Some(Token::ParenClose)
                }
                ':' => {
                    self.pos += 1;
                    Some(Token::Colon)
                }
                ',' => {
                    self.pos += 1;

                    Some(Token::Comma)
                }
                '"' => {
                    self.pos += 1;
                    Some(self.read_string())
                }
                '\'' => {
                    self.pos += 1;
                    Some(self.read_char())
                }
                '/' => {
                    if self.starts_with("//") {
                        Some(self.read_comment())
                    } else {
                        self.pos += 1;
                        return Some(Token::Divide);
                    }
                }
                '.' => {
                    if self.starts_with("..") {
                        self.pos += 2;
                        return Some(Token::Ellipse);
                    }
                    self.pos += 1;
                    return Some(Token::Dot);
                }
                // bitwise
                '&' => {
                    if self.starts_with("&&") {
                        self.pos += 2;
                        return Some(Token::And);
                    }
                    self.pos += 1;
                    return Some(Token::BitwiseAnd);
                }
                '|' => {
                    if self.starts_with("||") {
                        self.pos += 2;
                        return Some(Token::Or);
                    }
                    self.pos += 1;
                    return Some(Token::BitwiseOr);
                }
                '^' => {
                    self.pos += 1;
                    return Some(Token::BitwiseXor);
                }
                '~' => {
                    self.pos += 1;
                    return Some(Token::BitwiseNot);
                }
                // arithmetic
                '+' => {
                    self.pos += 1;
                    return Some(Token::Plus);
                }
                '-' => {
                    self.pos += 1;
                    return Some(Token::Minus);
                }
                '*' => {
                    self.pos += 1;
                    return Some(Token::Multiply);
                }
                '%' => {
                    self.pos += 1;
                    return Some(Token::Modulo);
                }
                // comparison
                '!' => {
                    if self.starts_with("!=") {
                        self.pos += 2;
                        return Some(Token::NotEqual);
                    }
                    self.pos += 1;
                    return Some(Token::Not);
                }
                '>' => {
                    if self.starts_with(">=") {
                        self.pos += 2;
                        return Some(Token::GreaterThanOrEqual);
                    }
                    self.pos += 1;
                    return Some(Token::GreaterThan);
                }
                '<' => {
                    if self.starts_with("<=") {
                        self.pos += 2;
                        return Some(Token::LessThanOrEqual);
                    }
                    self.pos += 1;
                    return Some(Token::LessThan);
                }
                ch if ch.is_digit(10) => Some(self.read_number()),
                ch if ch.is_ascii_alphabetic() => Some(self.read_identifier()),
                ch => panic!("Unexpected character: {}", ch),
            }
        }
    }
}

#[test]
fn test_lexer() {
    let input = r#"

myResult: MyFunction(String("Value")) {
    callBack: {
        io.print({String("Hello, world!"), Int(123)})
    },
}
"#;
    let expected = vec![
        Token::NewLine(2),
        Token::Identifier("myResult".to_string()),
        Token::Colon,
        Token::WhiteSpace(1),
        Token::Identifier("MyFunction".to_string()),
        Token::ParenOpen,
        Token::Identifier("String".to_string()),
        Token::ParenOpen,
        Token::StringLiteral("Value".to_string()),
        Token::ParenClose,
        Token::ParenClose,
        Token::WhiteSpace(1),
        Token::CurlyBraceOpen,
        Token::NewLine(1),
        Token::WhiteSpace(4),
        Token::Identifier("callBack".to_string()),
        Token::Colon,
        Token::WhiteSpace(1),
        Token::CurlyBraceOpen,
        Token::NewLine(1),
        Token::WhiteSpace(8),
        Token::Identifier("io".to_string()),
        Token::Dot,
        Token::Identifier("print".to_string()),
        Token::ParenOpen,
        Token::CurlyBraceOpen,
        Token::Identifier("String".to_string()),
        Token::ParenOpen,
        Token::StringLiteral("Hello, world!".to_string()),
        Token::ParenClose,
        Token::Comma,
        Token::WhiteSpace(1),
        Token::Identifier("Int".to_string()),
        Token::ParenOpen,
        Token::NumberLiteral("123".to_string()),
        Token::ParenClose,
        Token::CurlyBraceClose,
        Token::ParenClose,
        Token::NewLine(1),
        Token::WhiteSpace(4),
        Token::CurlyBraceClose,
        Token::Comma,
        Token::NewLine(1),
        Token::CurlyBraceClose,
        Token::NewLine(1),
        Token::EndOfFile,
    ];
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<Token>>();
    assert_eq!(tokens, expected);
}
