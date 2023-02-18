// example of a valid program
// myResult: MyFunction(String("Value")) {
//     callBack: {
//         io.print({String("Hello, world!"), Int(123)})
//     },
// }

// Tokens:
// IntLiteral, FloatLiteral, CharLiteral, StringLiteral, Identifier, curlyBraceOpen, curlyBraceClose, ParenOpen, ParenClose, Colon, Comma, Period, WhiteSpace, Comment, Eof
// "stringLiteral" -> stringLiteral
// 123 | 1_000 -> intLiteral
// 1.0 | 1.0e10 -> floatLiteral
// Word -> identifier
// { | } -> curlyBraceOpen, curlyBraceClose
// ( | ) -> ParenOpen, ParenClose
// : -> Colon
// , -> comma
// . -> period
//  -> WhiteSpace
// // -> comment
// \r\n | \n | \r  -> newline
// \t, \v, \f, \u{A0} -> space
// -> eof

// valid patterns

// Literals
// StringLiteral
//  -> ParenOpen
//  -> curlyBraceOpen
//  -> WhiteSpace

// CharLiteral
//  -> ParenOpen
//  -> curlyBraceOpen
//  -> WhiteSpace

// IntLiteral & FloatLiteral
//  -> CurlyBraceClose
//  -> ParenClose
//  -> WhiteSpace

// Identifiers
// Identifier
//  -> Colon
//  -> CurlyBraceOpen
//  -> CurlyBraceClose
//  -> ParenOpen
//  -> ParenClose
//  -> WhiteSpace

// Symbols
// CurlyBraceOpen
//  -> CurlyBraceClose
//  -> Identifier
//  -> StringLiteral
//  -> intLiteral
//  -> floatLiteral
//  -> curlyBraceOpen
// CurlyBraceClose

//  -> CurlyBraceClose
//  -> WhiteSpace
//  -> space
//  -> newline
//  -> comment
//  -> eof

// ParenOpen
//  -> ParenClose
//  -> Identifier
//  -> StringLiteral
//  -> intLiteral
//  -> floatLiteral
//  -> curlyBraceOpen

// ParenClose
//  -> CurlyBraceClose
//  -> ParenClose
//  -> WhiteSpace
//  -> comma

// Colon
// -> Identifier
// -> CurlyBraceOpen
// -> WhiteSpace

// Comma
//  -> *
//  -> eof
// WhiteSpace
// comment
// eof

// Period
//  -> Identifier
//  -> WhiteSpace

// WhiteSpace
//  -> *

use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    CharLiteral(char),
    Identifier(String),
    CurlyBraceOpen,
    CurlyBraceClose,
    ParenOpen,
    ParenClose,
    Colon,
    Comma,
    Period,
    Comment(String),
    WhiteSpace(usize),
    Newline(usize),
    // Eof,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, pos: 0 }
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
        let number = self.read_while(|ch| ch.is_digit(10) || ch == '_');
        if self.starts_with(".") {
            self.pos += 1;
            let fraction = self.read_while(|ch| ch.is_digit(10) || ch == '_');
            // the next character must be a whitespace | curlyBraceClose | ParenClose | comma
            if self.starts_with(" ")
                || self.starts_with("}")
                || self.starts_with(")")
                || self.starts_with(",")
            {
                let number = format!("{}.{}", number, fraction);
                Token::FloatLiteral(f64::from_str(&number).unwrap())
            } else {
                panic!("Invalid number literal");
            }
        } else {
            Token::IntLiteral(i64::from_str(&number.replace("_", "")).unwrap())
        }
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
        // if is newline
        let newline = self.read_while(|ch| ['\r', '\n', '\u{A0}'].contains(&ch));
        if newline.len() > 0 {
            return Token::Newline(newline.len());
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

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let white_space = self.read_whitespace();
        if white_space != Token::WhiteSpace(0) {
            return Some(white_space);
        }
        match self.next_char()? {
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
                    panic!("Unexpected character: /");
                }
            }
            '.' => {
                self.pos += 1;
                Some(Token::Period)
            }
            // '\x00' => {
            //     self.pos += 1;
            //     Some(Token::Eof)
            // }
            ch if ch.is_digit(10) => Some(self.read_number()),
            ch if ch.is_ascii_alphabetic() => Some(self.read_identifier()),
            ch => panic!("Unexpected character: {}", ch),
        }
    }
}

#[test]
fn test_tokenize() {
    let input = r#"
myResult: MyFunction(String("Value")) {
    callBack: {
        io.print({String("Hello, world!"), Int(123)})
    },
}
"#;
    let expected = vec![
        Token::Newline(1),
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
        Token::Newline(1),
        Token::WhiteSpace(4),
        Token::Identifier("callBack".to_string()),
        Token::Colon,
        Token::WhiteSpace(1),
        Token::CurlyBraceOpen,
        Token::Newline(1),
        Token::WhiteSpace(8),
        Token::Identifier("io".to_string()),
        Token::Period,
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
        Token::IntLiteral(123),
        Token::ParenClose,
        Token::CurlyBraceClose,
        Token::ParenClose,
        Token::Newline(1),
        Token::WhiteSpace(4),
        Token::CurlyBraceClose,
        Token::Comma,
        Token::Newline(1),
        Token::CurlyBraceClose,
        Token::Newline(1),
        // Token::Eof,
    ];
    let tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.collect::<Vec<Token>>();
    assert_eq!(tokens, expected);
}

#[derive(Debug, PartialEq)]
enum Ast {
    Program(Vec<AstNode>),
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Function {
        name: String,
        args: Vec<String>,
        body: Vec<AstNode>,
    },

    Literal(Literal),
    Identifier(String),
    Newline(usize),
    WhiteSpace(usize),
    Comment(String),
}

#[derive(Debug, PartialEq)]
enum Literal {
    StringLiteral(String),
    IntLiteral(i64),
    CharLiteral(char),
}

#[derive(Debug, PartialEq)]
struct AstNode {
    kind: Ast,
    line: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
    col: usize,
}

// parser
impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn consume(&mut self, expect: Token) -> Result<(), String> {
        match self.peek() {
            Some(token) => {
                if *token == expect {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err(format!("Expected {:?}, got {:?}", expect, token))
                }
            }
            None => Err(format!(
                "Unexpected end of input at line {}, col {}",
                self.line, self.col
            )),
        }
    }

    fn parse_function_call(&mut self) -> Result<AstNode, String> {
        todo!()
    }

    fn parse_function(&mut self) -> Result<AstNode, String> {
        todo!()
    }

    fn parse_program(&mut self) -> Result<Ast, String> {
        let mut nodes = vec![];
        while let Some(token) = self.peek() {
            match token {
                Token::Newline(_) => {
                    // skip
                    self.pos += 1;
                }
                Token::WhiteSpace(_) | Token::Comment(_) => {
                    // add the usize value of token::whiteSpace to pos and col
                    // skip
                    self.pos += 1;
                }
                Token::Identifier(_) => {
                    let node = self.parse_function_call()?;
                    nodes.push(node);
                }
                Token::CurlyBraceOpen => {
                    let node = self.parse_function()?;
                    nodes.push(node);
                }
                _ => {
                    return Err(format!(
                        "Unexpected token {:?} at line {}, col {}",
                        token, self.line, self.col
                    ))
                }
            }
        }
        Ok(Ast::Program(nodes))
    }
}

#[test]
fn test_parser() {
    let input = r#"
myResult: MyFunction(String("Value")) {
    callBack: {
        io.print({String("Hello, world!"), Int(123)})
    },
}
"#;

    let tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().unwrap();
    println!("{:#?}", ast);
}

fn main() {
    println!("Hello, world!");
    // #[derive(Debug, Clone, PartialEq)]
    // enum MyEnum {
    //     Comment(String),
    // }

    // let my_enum = MyEnum::Comment("Hello World".to_string());
    // println!("{:#?}", my_enum);
    // if let MyEnum::Comment(value) = my_enum {
    //     println!("{:?}", value)
    // }
}
