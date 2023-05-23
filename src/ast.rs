use serde::{Serialize};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Literal {
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    CharLiteral(char),
    OctalLiteral(u32),
    HexLiteral(u8),
    BinaryLiteral(u32),
    StringLiteral(String),
}

#[derive(Debug, PartialEq, Serialize, Clone)]

pub enum Atom {
    Identifier(String),
    Literal(Literal),
    EndOfFile
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Unary {
    SpreadExpr(Box<Expr>), // ...
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum BinaryOp {
    Assignment,
    Accessor,
    Invoke,  // can be called on any Body defines
    FieldDef,
    TypeDef,
    // BodyDef //  Body {} an invokable defined, has access to sibling values
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Binary {
    pub op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum TernaryOp {
    // Conditional,
    InvokeDefine   // func (func-params) {func-def}
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Ternary {
    pub op: TernaryOp,
    pub left: Box<Expr>,
    pub middle: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum GroupOp{
    AssignmentBlock,
    StatementBlock,
    ParamBlock,
    AnonymousType,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Group {
    pub op: GroupOp,
    pub exprs: Vec<Expr>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Expr {
    // Atom
    Atom(Atom),             // Identifier, Literal, EndOfFile
    // Unary
    Unary(Unary),           // ...`a`
    // Binary
    Binary(Binary),         // `a` = `b`
    // Ternary
    Ternary(Ternary),       // `a` ? `b` : `c`  or  `a` ( `b` ) { `c` }
    // Grouping
    Group(Group),           // ( ... ) or { ... }
}