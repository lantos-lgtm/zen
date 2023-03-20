use serde::{Serialize};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Program(pub Vec<Expr>);

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Serialize, Clone)]
// lets not support destructuring for now
pub struct Assignment {
    pub key     : Box<Expr>,     // Identifier,   // a:
    pub value   : Box<Expr>,    // Int.I32(1)
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Accessor {
    pub object  : Box<Expr>,    // Identifier,   // a.
    pub property: Box<Expr>,    // b
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct AssignmentBlock(pub Vec<Expr>);

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct StatementBlock(pub Vec<Expr>);

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct ParamBlock(pub Vec<Expr>);


#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct TypeDef {
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
    Bool(bool),
    OctalLiteral(u32),
    HexLiteral(u8),
    BinaryLiteral(u32),
    StringLiteral(String),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Expr {
    // Atom
    Identifier(Identifier),
    Literal(Literal),

    // Unary
    SpreadExpr(Box<Expr>), // ...

    // Binary
    Assignment(Assignment),     // a: Int.I32(1)
    Accessor(Accessor),         // a.b

    // Ternary

    // Grouping
    AssignmentBlock(AssignmentBlock),                   // { ... }
    StatementBlock(StatementBlock), // { ... }
    ParamBlock(ParamBlock),         // ( ... )
    
    TypeDef(TypeDef),           // Type { ... }
    FuncCall(FuncCall),         // fn ( ... ) { ... }?

    EndOfFile
}
