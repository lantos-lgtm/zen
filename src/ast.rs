use serde::{Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct Program(pub Vec<Expr>);

#[derive(Debug, PartialEq, Serialize)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Serialize)]
pub enum Key {
    Key(Identifier),
    DestructureKeys(Vec<Identifier>), // destructuring without type
    DestructureKeysAssign(Vec<Assignment>), // destructuring with type
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Assignment {
    pub key: Key,
    pub value: Box<Expr>,
}

// #[derive(Debug, PartialEq, Serialize)]
// pub struct Ellipse(pub Identifier);

#[derive(Debug, PartialEq, Serialize)]
pub enum SpreadOperator {
    // ...Identifier
    Identifier(Identifier),
    // ...{ ... }
    typeDef(Box<Expr>)
};

// #[derive(Debug, PartialEq, Serialize)]
// pub struct TypeExpr {
//     // : Type || Type { TypeExpr }
//     pub base_type: Box<TypeExpr>,
//     pub fields: Vec<Assignment>,
// }

#[derive(Debug, PartialEq, Serialize)]
pub struct Block(Vec<Expr>);

#[derive(Debug, PartialEq, Serialize)]
pub enum Body {
    Fields(Fields),
    Block(Block),
}


#[derive(Debug, PartialEq, Serialize)]

pub struct Fields(pub Vec<Assignment>);

#[derive(Debug, PartialEq, Serialize)]
pub struct Call {
    pub name: Identifier,
    pub args: Fields,
    pub body: Body,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TypeDef {
    pub name: Identifier,
    pub fields: Fields,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Accessor {
    pub object: Identifier,
    pub property: Box<Expr>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Literal {
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    // Bool(bool),
    // Hex(HexLiteral),
    // Octal(OctalLiteral),
    // Binary(BinaryLiteral),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Expr {
    // Atom
    Identifier(Identifier),
    Literal(Literal),

    // Unary
    SpreadOperator(SpreadOperator),

    // Binary
    Assignment(Assignment),
    Accessor(Accessor),

    // Grouping
    Body(Body),
    TypeDef(TypeDef),
    Call(Call),
}





// Expr            -> Atom | Binary | Unary | Grouping
// Binary          -> Expr Op Expr
// Unary           -> (! | -) Expr
// Grouping        -> ( Expr ) | { Expr }
// Atom            -> Identifier | Literal
// Identifier      -> String
// Literal         -> StringLiteral | IntLiteral | FloatLiteral
// Op              -> MathOps | EqualityOps | ComparisonOps | LogicalOps //| BitwiseOps
// ComparisonOps   -> < | > | <= | >=
// LogicalOps      -> && | ||
// EqualityOps     -> == | !=
// MathOps         -> + | - | * | **| / | % 
// BitwiseOps      -> & | | | ^ | ~ | << | >>