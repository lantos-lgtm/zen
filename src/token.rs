

use serde::Serialize;
#[derive(Debug, PartialEq, Serialize)]
pub enum Token {
    // Literals
    StringLiteral(String),
    // IntLiteral(i64),
    // FloatLiteral(f64),
    NumberLiteral(String),
    CharLiteral(char),
    Identifier(String),

    // Binary
    Colon,
    Dot,

    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Logical
    And,
    Or,
    Xor,
    // NOT,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Unary
    Not,
    Ellipse,

    // Group
    CurlyBraceOpen,
    CurlyBraceClose,
    ParenOpen,
    ParenClose,

    // markup
    Comma,
    Comment(String),
    WhiteSpace(usize),
    NewLine(usize),
    // Eof,
    EndOfFile,
}