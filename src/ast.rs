use serde::{Serialize};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct TypeDef { // is this a binary expression?
    pub name    : Box<Expr>,    // Identifier, // Type
    pub fields  : Box<Expr>     // Block, // { ... }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FuncCall {
    pub name    : Box<Expr>,                // Identifier, // Fn
    pub args    : Box<Expr>,                // ParamBlock, // ( ... )
    pub fields  : Option<Box<Expr>>,         // Option<Block>, // { ... }?
    pub body    : Option<Box<Expr>>          // Option<StatementBlock>,   // { ... }?
}

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
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Unary {
    SpreadExpr(Box<Expr>), // ...
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum BinaryOp {
    Assignment,
    Accessor,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Binary {
    pub op: BinaryOp,
    pub left: Box<Expr>,
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
    Atom(Atom),             // Identifier, Literal
    // Unary
    Unary(Unary),           // ...a
    // Binary
    Binary(Binary),         // a = b
    // Ternary
    // Ternary(Ternary),       // a ? b : c
    // Grouping
    Group(Group),           // ( ... )
    // Compound
    TypeDef(TypeDef),                   // Type { ... }
    FuncCall(FuncCall),                 // fn ( ... ) { ... }?

    EndOfFile
}