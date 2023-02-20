// example of a valid program
// myResult: MyFunction(String("Value")) {
//     callBack: {
//         io.print({String("Hello, world!"), Int(123)})
//     },
// }

// Tokens:
// IntLiteral, FloatLiteral, CharLiteral, StringLiteral, Identifier, curlyBraceOpen, curlyBraceClose, ParenOpen, ParenClose, Colon, Comma, Dot, WhiteSpace, Comment, Eof
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

// Dot
//  -> Identifier
//  -> WhiteSpace

// WhiteSpace
//  -> *

mod tokenizer;
mod ast;
mod parser;
use parser::{Parser};


// repl
fn parse_repl() {
    todo!()
}
// file
fn parse_file(file_path: &String) {
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");
    let mut parser = Parser::new(&file);
    let ast = parser.parse();
    println!("{:#?}", ast);

}
// string
fn parse_string(string: &String) {
    let mut parser = Parser::new(&string);
    let ast = parser.parse();
    println!("{:#?}", ast);
}

fn main() {
    // parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter();
    args.next();
    let mut file_path = None;
    let mut repl = false;
    let mut string = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" => {
                file_path = Some(args.next().expect("Expected file name after -f"));
            }
            "-s" => {
                string = Some(args.next().expect("Expected string after -s"));
            }
            _ => {
                repl = true;
            }
        }
    }
    if repl {
        parse_repl();
    } else {
        if let Some(file_path) = file_path {
            // load file
            parse_file(file_path);
        } else if let Some(string) = string {
            parse_string(string);
        }
    }

    
}
