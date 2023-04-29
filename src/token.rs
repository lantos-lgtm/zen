

use serde::Serialize;
#[derive(Debug, PartialEq, Serialize)]
pub enum Token {
    // Literals
    StringLiteral(String),
    NumberLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
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
    // NOT,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,

    // Comparison
    Equality,
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