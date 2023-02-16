
// example of a valid program
// myResult: MyFunction(String("Value")) {
//     callBack: {
//         io.print({String("Hello, world!"), Int(123)})
//     },
// }

// Tokens:
// IntLiteral, FloatLiteral, CharLiteral, StringLiteral, Identifier, curlyBraceOpen, curlyBraceClose, ParenOpen, ParenClose, Colon, Comma, Period, Whitespace, Comment, Eof
// "stringLiteral" -> stringLiteral
// 123 | 1_000 -> intLiteral
// 1.0 | 1.0e10 -> floatLiteral
// Word -> identifier
// { | } -> curlyBraceOpen, curlyBraceClose
// ( | ) -> ParenOpen, ParenClose
// : -> Colon
// , -> comma
// . -> period
//  -> Whitespace
// // -> comment
// \r\n | \n | \r  -> newline
// \t, \v, \f, \u{A0} -> space
// -> eof

// valid patterns

// Literals
// StringLiteral
//  -> ParenOpen
//  -> curlyBraceOpen
//  -> Whitespace

// CharLiteral
//  -> ParenOpen
//  -> curlyBraceOpen
//  -> Whitespace

// IntLiteral & FloatLiteral
//  -> CurlyBraceClose
//  -> ParenClose
//  -> Whitespace

// Identifiers
// Identifier
//  -> Colon
//  -> CurlyBraceOpen
//  -> CurlyBraceClose
//  -> ParenOpen
//  -> ParenClose
//  -> Whitespace


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
//  -> Whitespace
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
//  -> Whitespace
//  -> comma

// Colon
// -> Identifier
// -> CurlyBraceOpen
// -> Whitespace

// Comma
//  -> *
//  -> eof
// Whitespace
// comment
// eof

// Period
//  -> Identifier
//  -> Whitespace

// Whitespace
//  -> *



use std::str::FromStr;
use std::error::Error;

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
    Whitespace,
    Comment(String),
    Eof,
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
            if self.starts_with(" ") || self.starts_with("}") || self.starts_with(")") || self.starts_with(",") {
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
        let s = self.read_while(|ch| ch != '"' && ch != '\n');
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

    fn skip_whitespace(&mut self) {
        self.read_while(|ch| ch.is_whitespace());
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
        self.skip_whitespace();
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
            },
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
            ch if ch.is_digit(10) => Some(self.read_number()),
            ch if ch.is_ascii_alphabetic() => Some(self.read_identifier()),
            ch => panic!("Unexpected character: {}", ch),

        }
    }
}


fn test_ast() {

    let token_string = "
        Person: Type {
        name: String
        age: Int.U8
    }";

    let mut tokenizer = Tokenizer::new(token_string);
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next() {
        tokens.push(token);
    }
    println!("{:?}", tokens);

}

fn main() {
    test_ast(); 
    println!("Hello, world!");
}
