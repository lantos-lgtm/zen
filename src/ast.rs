use serde::{Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct Program(pub Vec<Expr>);

#[derive(Debug, PartialEq, Serialize)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Serialize)]
pub enum Key {
    Key(Identifier),
    DestructureKeys(Vec<Identifier>), // destructuring without type
    DestructureKeysAssignment(Vec<Assignment>), // destructuring with type
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Assignment {
    pub key: Key,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Ellipse(pub Identifier);

#[derive(Debug, PartialEq, Serialize)]
pub struct SpreadOperator(pub Identifier);

// #[derive(Debug, PartialEq, Serialize)]
// pub struct TypeExpr {
//     // : Type || Type { TypeExpr }
//     pub base_type: Box<TypeExpr>,
//     pub fields: Vec<Assignment>,
// }

#[derive(Debug, PartialEq, Serialize)]
pub struct Body {
    pub fields: Vec<Expr>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Call {
    pub name: Identifier,
    pub args: Box<Body>,
    pub body: Box<Body>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Accessor {
    pub name: Identifier,
    pub field: Box<Expr>,
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
    Identifier(Identifier),
    Assignment(Assignment),
    Call(Call),
    // TypeDef(TypeExpr),
    Literal(Literal),
    Ellipse(Ellipse),
    SpreadOperator(SpreadOperator),
    Body(Body),
    Accessor(Accessor),
}
