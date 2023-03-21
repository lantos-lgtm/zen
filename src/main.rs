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

mod lexer;
mod ast;
mod token;
mod parser;
mod codegen;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Zen")]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// runs a file or string
    Run {
        #[arg(short, long)]
        file: PathBuf,

        #[arg(short, long)]
        string: String,
    },
    // runs a repl session
    Repl,
}

fn parse_file(file_path: &PathBuf) {
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");
    let mut parser = parser::Parser::new(&file);
    let ast = parser.parse();
    println!("{:#?}", ast);
}
// // string
fn parse_string(string: &String) {
    let mut parser = parser::Parser::new(&string);
    let ast = parser.parse();
    println!("{:#?}", ast);
}

fn main() {

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run {file, string} => {
           if !file.is_file() {
               parse_file(file);
            } else if !string.is_empty() {
                parse_string(string);
            } else {
                panic!("No file or string provided");
            }
        },
        Commands::Repl => {
            todo!()
        }
    }

    
}
