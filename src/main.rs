// // example of a valid program
// // myResult: MyFunction(String("Value")) {
// //     callBack: {
// //         io.print({String("Hello, world!"), Int(123)})
// //     },
// // }

// // Tokens:
// // "stringLiteral" -> stringLiteral
// // 123 | 1_000 -> intLiteral
// // 1.0 | 1.0e10 -> floatLiteral
// // Word -> identifier
// // { | } -> curlyBraceOpen, curlyBraceClose
// // ( | ) -> ParenOpen, ParenClose
// // : -> Colon
// // , -> comma
// // . -> period
// // // -> comment
// // \r\n | \n | \r  -> newline
// // \t, \v, \f, \u{A0} -> space
// // -> eof

// // valid patterns

// // Literals
// // StringLiteral
// //  -> ParenClose
// //  -> curlyBraceOpen

// // CharLiteral
// //  -> ParenClose
// //  -> curlyBraceOpen

// // IntLiteral & FloatLiteral
// //  -> ParenClose
// //  -> CurlyBraceClose

// // Identifiers
// // Identifier
// //  -> Colon
// //  -> CurlyBraceOpen
// //  -> CurlyBraceClose
// //  -> ParenOpen
// //  -> ParenClose

// // Symbols
// // CurlyBraceOpen
// //  -> CurlyBraceClose
// //  -> Identifier
// //  -> StringLiteral
// //  -> intLiteral
// //  -> floatLiteral
// //  -> curlyBraceOpen
// // CurlyBraceClose

// //  -> CurlyBraceClose
// //  -> space
// //  -> newline
// //  -> comment
// //  -> eof

// // ParenOpen
// //  -> ParenClose
// //  -> Identifier
// //  -> StringLiteral
// //  -> intLiteral
// //  -> floatLiteral
// //  -> curlyBraceOpen

// // ParenClose
// //  -> CurlyBraceClose
// //  -> ParenClose
// //  -> comma

// // Colon
// // -> Identifier
// // -> CurlyBraceOpen

// // Comma
// //  -> *
// //  -> eof
// // comment
// // eof

// // Dot
// //  -> Identifier
// //  -> Dot

mod tokenizer;
mod ast;
mod parser;
use parser::{Parser};


// // repl
// fn parse_repl() {
//     todo!()
// }
// // file
// fn parse_file(file_path: &String) {
//     let file = std::fs::read_to_string(file_path).expect("Failed to read file");
//     let mut parser = Parser::new(&file);
//     let ast = parser.parse();
//     println!("{:#?}", ast);

// }
// // string
// fn parse_string(string: &String) {
//     let mut parser = Parser::new(&string);
//     let ast = parser.parse();
//     println!("{:#?}", ast);
// }

fn main() {
    // parse args using clap
    // run, build, test
    //  -f file_path
    //  -s string
    // -h help
    // "" repl

    let matches = clap::Command::new("zen")
        .version("0.1.0")
        .author("Lyndon Leong <l.leong1618[at]gmail.com>")
        .about("A programming language")
        .arg(
            clap::Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("File to parse")
        )
        .arg(
            clap::Arg::new("string")
                .short('s')
                .long("string")
                .value_name("STRING")
                .help("String to parse")
        )
        .get_matches();

    // Match on args    

}
