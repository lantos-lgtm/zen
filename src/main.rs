

#[derive(PartialEq, Debug)]
enum TokenKind {
    Number,
    String,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Whitespace,
    Comment,
    Eof,
}

#[derive(PartialEq, Debug)]
struct Parser {
    input: String,
    pos: usize,
}

#[derive(PartialEq, Debug)]
struct Token {
    kind: TokenKind,
    value: String,
}

#[derive(PartialEq, Debug)]
enum AstKind {
    Number,
    String,
    Identifier,
    Operator,
    Keyword,
    Punctuation,
    Whitespace,
    Comment,
    Eof,
}

#[derive(PartialEq, Debug)]
struct Ast {
    kind: AstKind,
    name: String,
    fields: Vec<Ast>
}

// unit test for ast
// next_token
impl Parser {
    fn next_token(&mut self) -> Token {
        // collect the next token
        let mut token = Token {
            kind: TokenKind::Eof,
            value: "".to_string(),
        };
        return token;
    }
}


fn parse( input: String ) -> Ast {
    let mut parser = Parser {
        input: input,
        pos: 0,
    };
    let mut ast = Ast {
        kind: AstKind::Eof,
        name: "".to_string(),
        fields: vec![]
    };
    // parse the first token
    let token = parser.next_token();

    return ast;
}



fn test_ast() {

    let expected_string = "
        Person: Type {
        name: String
        age: Int.U8
    }";
    let ast = parse(expected_string.to_string());
    let expected_ast = Ast {
        kind: AstKind::Identifier,
        name: "Person".to_string(),
        fields: vec![
            Ast {
                kind: AstKind::Identifier,
                name: "name".to_string(),
                fields: vec![
                    Ast {
                        kind: AstKind::String,
                        name: "String".to_string(),
                        fields: vec![]
                    }
                ]
            },
            Ast {
                kind: AstKind::Identifier,
                name: "age".to_string(),
                fields: vec![
                    Ast {
                        kind: AstKind::Number,
                        name: "Int".to_string(),
                        fields: vec![]
                    },
                    Ast {
                        kind: AstKind::Identifier,
                        name: "U8".to_string(),
                        fields: vec![]
                    }
                ]
            }
        ]
    };
    assert_eq!(ast, expected_ast);

}

fn main() {
    test_ast(); 
    println!("Hello, world!");
}
