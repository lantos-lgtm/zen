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

// use std::error::Error;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
use std::collections::HashMap;



#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    Ellipse,
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
                if self.starts_with("..") {
                    self.pos += 2;
                    return Some(Token::Ellipse);
                } 
                self.pos += 1;
                return Some(Token::Period);
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

// parser

enum BodyValue {
    Ident(Ident),
    Ellipse(Ellipse),
}

enum BodyTypeValue {
    Assignment(Ident, Ident), // `ident` `:` `ident` 

}

// enum EvaluableValue {
//     Ident(Ident),
//     Ellipse(Ellipse),
//     Call(Call),
//     CallWithBody(CallWithBody),

// }

struct UnkownBody(Vec<BodyValue>);          // { `body` }
struct BodyDefinition(Vec<BodyValue>);      // { `body` }
struct BodyTypeDefinition(Vec<BodyValue>);  // { `body` }
struct Ellipse(Ident);                      // `...` `ident`
struct Ident(String);                       // `ident`
struct Call(Box<Expr>, Vec<Expr>);          // `ident` `(` `expr` `)`
struct CallWithBody(Call, UnkownBody);            // `call` `body`
struct Assignment(Ident, Expr);             // `ident` `:` `expr`
struct FieldAccess(Ident, Expr);           // `ident` `.` `expr`
struct FieldCall(Ident, Vec<Expr>);         //  `ident` `(` `expr` `)`
 

#[derive(Debug, PartialEq, Serialize)]
pub enum Expr {
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    Call(),
    CallWithBody(Box<Expr>, Vec<Expr>, HashMap<String, Expr>),
    Object(HashMap<String, Expr>),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

pub struct Parser<'a> {
    tokens: std::iter::Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.tokens.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn parse_string_literal(&mut self) -> Result<Expr, ParseError> {
        if let Token::StringLiteral(value) = self.next_token()? {
            Ok(Expr::StringLit(value))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_number_literal(&mut self) -> Result<Expr, ParseError> {
        if let Token::IntLiteral(value) = self.next_token()? {
            Ok(Expr::IntLit(value))
        } else if let Token::FloatLiteral(value) = self.tokens.next().unwrap() {
            Ok(Expr::FloatLit(value))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_ident(&mut self) -> Result<Expr, ParseError> {
        if let Token::Identifier(name) = self.next_token()? {
            Ok(Expr::Ident(name))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.tokens.peek() {
            Some(&Token::StringLiteral(_)) => self.parse_string_literal(),
            Some(&Token::IntLiteral(_)) | Some(&Token::FloatLiteral(_)) => {
                self.parse_number_literal()
            }
            Some(&Token::Identifier(_)) => self.parse_ident(),
            Some(&Token::CurlyBraceOpen) => self.parse_object(),
            Some(&Token::ParenOpen) => {
                self.tokens.next();
                let expr = self.parse_expr()?;
                if let Some(&Token::ParenClose) = self.tokens.peek() {
                    self.tokens.next();
                    Ok(expr)
                } else {
                    Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
                }
            }
            _ => Err(ParseError::UnexpectedToken(self.tokens.next().unwrap())),
        }
    }

    fn parse_call(&mut self, func: Expr) -> Result<Expr, ParseError> {
        let mut args = Vec::new();
        loop {
            if let Some(&Token::ParenClose) = self.tokens.peek() {
                self.tokens.next();
                break;
            }
            args.push(self.parse_expr()?);
            match self.tokens.next() {
                Some(Token::Comma) => (),
                Some(Token::ParenClose) => break,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEOF),
            }
        }
        Ok(Expr::Call(Box::new(func), args))
    }

    fn parse_object(&mut self) -> Result<Expr, ParseError> {
        let mut properties = HashMap::new();
        loop {
            match self.tokens.next() {
                Some(Token::Identifier(name)) => {
                    if self.tokens.next() != Some(Token::Colon) {
                        return Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()));
                    }
                    let value = self.parse_expr()?;
                    properties.insert(name, value);
                }
                Some(Token::CurlyBraceClose) => break,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEOF),
            }
            match self.tokens.next() {
                Some(Token::Comma) => (),
                Some(Token::CurlyBraceClose) => break,
                Some(token) => return Err(ParseError::UnexpectedToken(token)),
                None => return Err(ParseError::UnexpectedEOF),
            }
        }
        Ok(Expr::Object(properties))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.tokens.peek() {
                Some(&Token::ParenOpen) => {
                    self.tokens.next();
                    expr = self.parse_call(expr)?;
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr()
    }
}

#[test]
fn test_parser() {
    let input = "myResult: MyFunction(String(\"Value\")) {
        callBack: {
            io.print({String(\"Hello, world!\"), Int(123)})
        },
    }";

    let expected = Expr::Object({
        let mut ast = HashMap::new();
        ast.insert(
            "myResult".to_string(),
            Expr::CallWithBody(
                Box::new(Expr::Ident("MyFunction".to_string())),
                vec![
                    Expr::Call(
                        Box::new(Expr::Ident("String".to_string())), 
                        vec![Expr::StringLit("Value".to_string())]),
                ],
                {
                    let mut ast = HashMap::new();
                    ast.insert(
                        "callBack".to_string(),
                        Expr::Object({
                            let mut ast = HashMap::new();
                            ast.insert(
                                "io.print".to_string(),
                                Expr::Call(
                                    Box::new(Expr::Ident("io.print".to_string())),
                                    vec![
                                        Expr::Object({
                                            let mut ast = HashMap::new();
                                            ast.insert("String".to_string(), Expr::StringLit("Hello, world!".to_string()));
                                            ast.insert("Int".to_string(), Expr::IntLit(123));
                                            ast
                                        })
                                    ],
                                ),
                            );
                            ast
                        })
                    );
                    ast
                },
                
            ),
        );
        ast
    });

    let mut parser = Parser::new(input);
    let result = parser.parse();

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    println!("{}", serde_json::to_string_pretty(&expected).unwrap());
    assert_eq!(result, Ok(expected));
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
