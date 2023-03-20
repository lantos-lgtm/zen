use crate::ast::{
    Accessor, Assignment, AssignmentBlock, Expr, FuncCall, Identifier, Literal, ParamBlock,
    StatementBlock, TypeDef, AnonymousType,
};
use crate::lexer::Lexer;
use crate::token::Token;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next();
        Parser {
            lexer,
            current_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next();
        &self.skip_formating();
    }
    // expect one or more tokens
    fn expect_token(&mut self, expected: Vec<Token>) {
        let mut found = false;
        match &self.current_token {
            None => panic!(
                "Was expecting one of {:?} but got {:?}",
                expected, self.current_token
            ),
            Some(token) => {
                for expected_token in &expected {
                    if std::mem::discriminant(token) == std::mem::discriminant(expected_token) {
                        found = true;
                    }
                }
                if !found {
                    panic!(
                        "Was expecting one of {:?} but got {:?}",
                        expected, self.current_token
                    );
                }
            }
        }
    }
    fn skip_formating(&mut self) {
        while let Some(token) = &self.current_token {
            match token {
                Token::Comment(_) | Token::Comma | Token::NewLine(_) | Token::WhiteSpace(_) => {
                    self.next_token();
                }
                _ => break,
            }
        }
    }
    fn parse_number_literal(&mut self) -> Expr {
        // starts with 0b, 0o, 0x -> binary, octal, hex
        // has a . -> float
        // has a e -> exponent
        // otherwise -> int
        if let Some(Token::NumberLiteral(value)) = &self.current_token {
            let number_expr = match value {
                // binary
                _ if value.starts_with("0b") => {
                    let val = value.trim_start_matches("0b");
                    let val = u32::from_str_radix(val, 2).unwrap();
                    Expr::Literal(Literal::BinaryLiteral(val))
                }
                // octal
                _ if value.starts_with("0o") => {
                    let val = value.trim_start_matches("0o");
                    let val = u32::from_str_radix(val, 8).unwrap();
                    Expr::Literal(Literal::OctalLiteral(val))
                }
                // hex
                _ if value.starts_with("0x") => {
                    let val = value.trim_start_matches("0x");
                    let val = u8::from_str_radix(val, 16).unwrap();
                    Expr::Literal(Literal::HexLiteral(val))
                }
                // float
                _ if value.contains(".") => {
                    todo!()
                }
                // int
                _ => {
                    let val = i64::from_str_radix(value, 10).unwrap();
                    Expr::Literal(Literal::IntLiteral(val))
                }
            };
            self.next_token();

            number_expr
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_string_literal(&mut self) -> Expr {
        if let Some(Token::StringLiteral(value)) = &self.current_token {
            let string_literal = Literal::StringLiteral(value.clone());
            self.next_token();
            Expr::Literal(string_literal)
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_char_literal(&mut self) -> Expr {
        if let Some(Token::CharLiteral(value)) = &self.current_token {
            let char_literal = Literal::CharLiteral(value.clone());
            self.next_token();
            Expr::Literal(char_literal)
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_literal(&mut self) -> Expr {
        // number literal
        // string literal
        // char literal
        match &self.current_token {
            Some(Token::NumberLiteral(_)) => self.parse_number_literal(),
            Some(Token::StringLiteral(_)) => self.parse_string_literal(),
            Some(Token::CharLiteral(_)) => self.parse_char_literal(),
            None => panic!("Unexpected EOF"),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_identifier(&mut self) -> Expr {
        if let Some(Token::Identifier(name)) = &self.current_token {
            let identifier = Identifier(name.clone());
            self.next_token();

            // ident : ident
            // ident . ident
            // ident { ... }
            // ident ( ... )
            match &self.current_token {
                // ident : ident
                Some(Token::Colon) => self.parse_assignment(Box::new(Expr::Identifier(identifier))),
                // ident . ident
                Some(Token::Dot) => self.parse_accessor(Box::new(Expr::Identifier(identifier))),
                // ident { ... }
                Some(Token::CurlyBraceOpen) => {
                    self.parse_block(Some(Box::new(Expr::Identifier(identifier))))
                }
                // ident ( ... )
                Some(Token::ParenOpen) => {
                    self.parse_paren_block(Some(Box::new(Expr::Identifier(identifier))))
                }
                _ => Expr::Identifier(identifier),
            }
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_assignment(&mut self, key: Box<Expr>) -> Expr {
        self.expect_token(vec![Token::Colon]);
        self.next_token();
        // key : value
        // key : Type ...
        // key : Func ...
        let value = Box::new(self.parse_expression());
        Expr::Assignment(Assignment { key, value })
    }

    fn parse_accessor(&mut self, object: Box<Expr>) -> Expr {
        self.expect_token(vec![Token::Dot]);
        self.next_token();
        let property = Box::new(self.parse_expression());
        Expr::Accessor(Accessor { object, property })
    }

    fn parse_spread_expression(&mut self) -> Expr {
        self.expect_token(vec![Token::Ellipse]);
        // skip this token(...)
        self.next_token();
        // this expr expects a Identifier
        self.expect_token(vec![Token::Identifier("".to_string())]);
        let expr = Box::new(self.parse_expression());
        // get the next expr
        Expr::SpreadExpr(expr)
    }

    fn parse_curly_block(&mut self, ident: Option<Box<Expr>>) -> Expr {
        // either a assignment block, a statement block
        self.expect_token(vec![Token::CurlyBraceOpen]);
        let mut is_statement_block = false;
        self.next_token();
        // this expects either
        // Spread       ...
        // Assignment   key : value
        // Ident        ident

        // Statement signatures
        // Accessor     ident . ident
        // FuncCall     ident paramBlock

        // if we encounter a curlybrace close we can finish parsing

        // if the result contains a statement signature return statement block
        // return assignment block

        let mut exprs: Vec<Expr> = Vec::new();
        loop {
            &self.skip_formating();
            match &self.current_token {
                Some(Token::CurlyBraceClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    exprs.push(self.parse_spread_expression());
                }
                Some(Token::Identifier(_)) => {
                    exprs.push(self.parse_identifier());
                }
                None | Some(Token::EndOfFile) => {
                    panic!("Unexpected EOF, expected: {:?}", Token::CurlyBraceClose);
                }
                // should add expected errors
                _ => {
                    panic!("Unexpected token: {:?}", self.current_token);
                }
            }
        }
        for expr in &exprs {
            match expr {
                Expr::Accessor(_) | Expr::FuncCall(_) => {
                    is_statement_block = true;
                    break;
                }
                _ => {}
            }
        }

        match &ident {
            Some(ident) => {
                if is_statement_block {
                    // maybe this can be the default constructor
                    unimplemented!("Statement block as type def, {:?}", self.current_token);
                } else {
                    Expr::TypeDef(TypeDef {
                        name: ident.to_owned(),
                        fields: Box::new(Expr::AssignmentBlock(AssignmentBlock(exprs))),
                    })
                }
            }
            None => {
                if is_statement_block {
                    Expr::StatementBlock(StatementBlock(exprs))
                } else {
                    // maybe this cn be anonymous Type?
                    Expr::AnonymousType(AnonymousType {
                        fields: Box::new(Expr::AssignmentBlock(AssignmentBlock(exprs))),
                    })
                    // unimplemented!("Assignment block as type def, ident:{:?} exprs:{:?} \n current_token:{:?}", ident, exprs, self.current_token);
                }
            }
        }
    }

    fn parse_paren_block(&mut self, ident: Option<Box<Expr>>) -> Expr {
        // WARNING: I MIGHT HAVE TO PUSH THIS TO AFTER TYPE CHECKING
        // because we need to check to see if the fields are valid
        // for the type
        // if not it might be a function call or statement block

        // a params block
        self.expect_token(vec![Token::ParenOpen]);
        self.next_token();

        // this expects either
        // Spread       ...ident || ...{}
        // Assignment   key : value
        // Ident        ident

        // if we encounter a paren close we can finish parsing
        let mut param_exprs: Vec<Expr> = Vec::new();
        let mut block_expr: Option<Expr> = None;
        loop {
            &self.skip_formating();
            match &self.current_token {
                Some(Token::ParenClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    param_exprs.push(self.parse_spread_expression());
                }
                Some(Token::Identifier(_)) => {
                    param_exprs.push(self.parse_identifier());
                }
                Some(Token::CharLiteral(_) | Token::StringLiteral(_) | Token::NumberLiteral(_)) => {
                    param_exprs.push(self.parse_literal());
                }

                None | Some(Token::EndOfFile) => {
                    // panic!("Unexpected EOF, expected: {:?}", &self.current_token);
                    // this is a valid case ) can be the end of a file
                    break;
                }

                // should add expected errors
                _ => {
                    unreachable!(
                        "lexerPos: {:?}, token: {:?}",
                        self.lexer.pos, self.current_token
                    );
                }
            }
        }

        // check to see if param has a assignment block or statement block
        loop {
            &self.skip_formating();
            match &self.current_token {
                Some(Token::CurlyBraceOpen) => {
                    block_expr = Some(self.parse_block(None));
                    break;
                }
                None | Some(Token::EndOfFile) => {
                    // this is a valid case ) can be the end of a file
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        // check to see if next token is a curly brace open
        if &self.current_token == &Some(Token::CurlyBraceOpen) {
            block_expr = Some(self.parse_block(None));
        }

        match &ident {
            Some(ident) => match block_expr {
                Some(expr) => {
                    if expr == Expr::StatementBlock(StatementBlock(vec![])) {
                        Expr::FuncCall(FuncCall {
                            name: ident.to_owned(),
                            args: Box::new(Expr::ParamBlock(ParamBlock(param_exprs))),
                            fields: None,
                            body: Some(Box::new(expr)),
                        })
                    } else {
                        Expr::FuncCall(FuncCall {
                            name: ident.to_owned(),
                            args: Box::new(Expr::ParamBlock(ParamBlock(param_exprs))),
                            fields: Some(Box::new(expr)),
                            body: None,
                        })
                    }
                }
                None => Expr::FuncCall(FuncCall {
                    name: ident.to_owned(),
                    args: Box::new(Expr::ParamBlock(ParamBlock(param_exprs))),
                    fields: None,
                    body: None,
                }),
            },
            None => {
                // anon block { }
                unimplemented!("Param block without a name")
            }
        }
    }

    fn parse_block(&mut self, ident: Option<Box<Expr>>) -> Expr {
        // either a assignment block, a statement block or a param block
        self.expect_token(vec![Token::CurlyBraceOpen, Token::ParenOpen]);
        let expr = match &self.current_token {
            Some(Token::CurlyBraceOpen) => self.parse_curly_block(ident),
            Some(Token::ParenOpen) => self.parse_paren_block(ident),
            _ => unreachable!(),
        };
        expr
    }

    fn parse_expression(&mut self) -> Expr {
        // Atoms
        //  identifier
        //  literals
        //  assignment

        // Unary
        //  spread

        // Binary
        //  Assignment
        //  Accessor

        //  Todo logical

        // Grouping
        //  AssignmentBlock
        //  StatementBlock
        //  ParamBlock

        &self.skip_formating();
        match &self.current_token {
            // Atoms
            Some(Token::Identifier(_)) => self.parse_identifier(),
            Some(Token::NumberLiteral(_)) => self.parse_number_literal(),
            Some(Token::StringLiteral(_)) => self.parse_string_literal(),
            // Binary should be handled by the caller

            // Unary
            Some(Token::Ellipse) => self.parse_spread_expression(),

            // Grouping
            Some(Token::CurlyBraceOpen) | Some(Token::ParenOpen) => self.parse_block(None),
            Some(Token::EndOfFile) => Expr::EndOfFile,
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse(&mut self) -> Expr {
        let mut expressions = Vec::new();

        while let Some(token) = &self.current_token {
            if token == &Token::EndOfFile {
                break;
            }
            expressions.push(self.parse_expression());
        }
        Expr::StatementBlock(StatementBlock(expressions))
    }
}

#[test]
fn test_parser() {
    let name_str = "Name: Type {
        fistName: String,
        lastName: String,
    }";
    let lexer = Lexer::new(name_str);
    let tokens = lexer.collect::<Vec<Token>>();
    println!("Parsing string: {:?}", name_str);
    println!("Parsing tokens: {:?}", tokens);
    let mut parser = Parser::new(name_str);
    let name_ast = parser.parse();

    let name_ast_expected =
        Expr::StatementBlock(StatementBlock(vec![Expr::Assignment(Assignment {
            key: Box::new(Expr::Identifier(Identifier("Name".to_string()))),
            value: Box::new(Expr::TypeDef(TypeDef {
                name: Box::new(Expr::Identifier(Identifier("Type".to_string()))),
                fields: Box::new(Expr::AssignmentBlock(AssignmentBlock(vec![
                    Expr::Assignment(Assignment {
                        key: Box::new(Expr::Identifier(Identifier("fistName".to_string()))),
                        value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
                    }),
                    Expr::Assignment(Assignment {
                        key: Box::new(Expr::Identifier(Identifier("lastName".to_string()))),
                        value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
                    }),
                ]))),
            })),
        })]));

    debug_assert!(name_ast == name_ast_expected);
}

#[test]
fn test_parser_1() {
    let name_str = "
        Name: Type {
            fistName: String,
            lastName: String,
        }
        Person: Type {
            ...Name,
            age: Int.i32(1),
        }

        greet: Function {
            args: {Person},
            body: {
                std.io.stdout.write(\"Hello, {person.firstName}\")
            }
        }

        myPerson: Person (
            firstName: String(\"John\"),
            lastName: String(\"Doe\"),
            age: Int.i32(32),
        )
    ";
    let lexer = Lexer::new(name_str);
    let tokens = lexer.collect::<Vec<Token>>();
    println!("Parsing string: {:?}", name_str);
    println!("Parsing tokens: {:?}", tokens);
    let mut parser = Parser::new(name_str);

    println!(
        "AST: {}",
        serde_json::to_string_pretty(&parser.parse()).unwrap()
    );
}
