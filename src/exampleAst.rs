// example ast

struct Program(Vec<Expr>);

struct Identifier(String);

enum Key {
    key(Identifier),
    multipleKeys(Vec<Identifier>) // destructuring
}

struct Assignment {
    key: Key,
    val: Expr,
}

struct Ellipse(Identifier);

struct SpreadOperator(Identifier);

struct TypeExpr {
    // : Type || Type { TypeExpr }
    baseType: TypeExpr,
    fields: Vec<Assignment>,
}

struct Body {
    fields: Vec<Expr>,
}


struct Call {
    name: Identifier,
    args: Body,
    body: Body,
}

enum Literal {
    String(String),
    Int(i32),
    Float(f32),
    // Bool(bool),
    // Hex(HexLiteral),
    // Octal(OctalLiteral),
    // Binary(BinaryLiteral),
}

struct DestructureAssignment {
    keys: Vec<Identifier>,
    val: Expr,
}

enum Expr {
    Assignment(Assignment),
    Call(Call),
    TypeDef(TypeExpr),
    Literal(Literal),
    Ellipse(Ellipse),
    SpreadOperator(SpreadOperator),
}

#[test]
fn test_ast() {


    // {
    //     x, y
    // }: myPerson.otherFunc1() {
    //     io.print(result.x)
    //     io.print(result.y)
    // }
    let ast1 = Expr::Destructure(
    
    );


    // otherFunc2() {
    //     other1: Person,
    //     other2: String
    // }
    let otherFunc2Ast = Expr::CallWithBody(Call {
        name: Expr::Ident("otherFuc2"),
        args: Expr::Body(vec![]),
        body: Expr::Body(vec![
            Expr::Assignment(Assignment {
                key: Expr::Ident("otherField1".to_string()),
                val: Expr::Ident("Person".to_string()),
            }),
            Expr::Assignment(Assignment {
                key: Expr::Ident("otherField2".to_string()),
                val: Expr::Ident("String".to_string()),
            }),
        ]),
    });
}
