use std::vec;

use crate::ast::{Atom, Binary, BinaryOp, Expr, FuncCall, Group, GroupOp, Literal, TypeDef, Unary};

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
        let _ = &self.skip_formating();
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
    fn parse_number_literal(&mut self) -> Result<Expr, ParseError> {
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
                    Expr::Atom(Atom::Literal(Literal::BinaryLiteral(val)))
                }
                // octal
                _ if value.starts_with("0o") => {
                    let val = value.trim_start_matches("0o");
                    let val = u32::from_str_radix(val, 8).unwrap();
                    Expr::Atom(Atom::Literal(Literal::OctalLiteral(val)))
                }
                // hex
                _ if value.starts_with("0x") => {
                    let val = value.trim_start_matches("0x");
                    let val = u8::from_str_radix(val, 16).unwrap();
                    Expr::Atom(Atom::Literal(Literal::HexLiteral(val)))
                }
                // float
                _ if value.contains(".") => {
                    // if contains e -> exponent
                    // lhs = float
                    // rhs = exponent
                    if value.contains("e") {
                        let mut split = value.split("e");
                        let lhs = split.next().unwrap().parse::<f64>().unwrap();

                        let rhs = split.next().unwrap();
                        let rhs = i32::from_str_radix(rhs, 10).unwrap();

                        // calculate the exponent
                        let float = lhs * 10_f64.powi(rhs);
                        Expr::Atom(Atom::Literal(Literal::FloatLiteral(float)))
                    } else {
                        let val = value.parse::<f64>().unwrap();
                        Expr::Atom(Atom::Literal(Literal::FloatLiteral(val)))
                    }
                }
                // int
                _ => {
                    if value.contains("e") {
                        let mut split = value.split("e");
                        let lhs = i64::from_str_radix(split.next().unwrap(), 10).unwrap();

                        let rhs = split.next().unwrap();
                        let rhs = i64::from_str_radix(rhs, 10).unwrap();

                        // calculate the exponent
                        let int = lhs * 10_i64.pow(rhs as u32);

                        Expr::Atom(Atom::Literal(Literal::IntLiteral(int)))
                    } else {
                        let val = value.parse::<i64>().unwrap();
                        Expr::Atom(Atom::Literal(Literal::IntLiteral(val)))
                    }
                }
            };
            self.next_token();

            Ok(number_expr)
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_string_literal(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::StringLiteral(value)) = &self.current_token {
            let string_literal = Literal::StringLiteral(value.clone());
            self.next_token();
            Ok(Expr::Atom(Atom::Literal(string_literal)))
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_bool_literal(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::BoolLiteral(value)) = &self.current_token {
            let bool_literal = Literal::BoolLiteral(value.clone());
            self.next_token();
            Ok(Expr::Atom(Atom::Literal(bool_literal)))
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_char_literal(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::CharLiteral(value)) = &self.current_token {
            let char_literal = Literal::CharLiteral(value.clone());
            self.next_token();
            Ok(Expr::Atom(Atom::Literal(char_literal)))
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_literal(&mut self) -> Result<Expr, ParseError> {
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

    fn parse_identifier(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::Identifier(identifier)) = &self.current_token {
            self.next_token();

            // ident : ident
            // ident . ident
            // ident { ... }
            // ident ( ... )
            match &self.current_token {
                // ident : ident
                Some(Token::Colon) => self
                    .parse_assignment(Box::new(Expr::Atom(Atom::Identifier(identifier.clone())))),
                // ident . ident
                Some(Token::Dot) => {
                    self.parse_accessor(Box::new(Expr::Atom(Atom::Identifier(identifier.clone()))))
                }
                // ident { ... }
                Some(Token::CurlyBraceOpen) => self.parse_block(Some(Box::new(Expr::Atom(
                    Atom::Identifier(identifier.clone()),
                )))),
                // ident ( ... )
                Some(Token::ParenOpen) => self.parse_paren_block(Some(Box::new(Expr::Atom(
                    Atom::Identifier(identifier.clone()),
                )))),
                _ => Ok(Expr::Atom(Atom::Identifier(identifier.clone()))),
            }
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    fn parse_assignment(&mut self, key: Box<Expr>) -> Result<Expr, ParseError> {
        self.expect_token(vec![Token::Colon]);
        self.next_token();
        // key : value
        // key : Type ...
        // key : Func ...
        let value = Box::new(self.parse_expression().unwrap());
        Ok(Expr::Binary(Binary {
            op: BinaryOp::Assignment,
            left: key,
            right: value,
        }))
    }

    fn parse_accessor(&mut self, object: Box<Expr>) -> Result<Expr, ParseError> {
        self.expect_token(vec![Token::Dot]);
        self.next_token();
        let property = Box::new(self.parse_expression().unwrap());
        Ok(Expr::Binary(Binary {
            op: BinaryOp::Accessor,
            left: object,
            right: property,
        }))
    }

    fn parse_spread_expression(&mut self) -> Result<Expr, ParseError> {
        self.expect_token(vec![Token::Ellipse]);
        // skip this token(...)
        self.next_token();
        // this expr expects a Identifier
        self.expect_token(vec![Token::Identifier("".to_string())]);
        let expr = Box::new(self.parse_expression().unwrap());
        // get the next expr
        Ok(Expr::Unary(Unary::SpreadExpr(expr)))
    }

    fn parse_curly_block(&mut self, ident: Option<Box<Expr>>) -> Result<Expr, ParseError> {
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
            let _ = &self.skip_formating();
            match &self.current_token {
                Some(Token::CurlyBraceClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    exprs.push(self.parse_spread_expression().unwrap());
                }
                Some(Token::Identifier(_)) => {
                    exprs.push(self.parse_identifier().unwrap());
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
            // if any of the exprs is not an assignment then we need to treat it as a statement block
            match expr {
                Expr::Binary(Binary {
                    op: BinaryOp::Assignment,
                    ..
                }) => {}
                _ => {
                    is_statement_block = true;
                    break;
                }
            }
        }

        match &ident {
            Some(ident) => {
                if is_statement_block {
                    // maybe this can be the default constructor
                    unimplemented!("Statement block as type def, {:?}", self.current_token);
                } else {
                    // this is a type def
                    Ok(Expr::TypeDef(TypeDef {
                        name: ident.to_owned(),
                        fields: Box::new(Expr::Group(Group {
                            exprs,
                            op: GroupOp::AnonymousType,
                        })),
                    }))
                }
            }
            None => {
                if is_statement_block {
                    // this is a statement block
                    Ok(Expr::Group(Group {
                        exprs,
                        op: GroupOp::StatementBlock,
                    }))
                } else {
                    // this is a anonymous type
                    Ok(Expr::Group(Group {
                        exprs,
                        op: GroupOp::AnonymousType,
                    }))
                }
            }
        }
    }

    fn parse_paren_block(&mut self, ident: Option<Box<Expr>>) -> Result<Expr, ParseError> {
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
            let _ = &self.skip_formating();
            match &self.current_token {
                Some(Token::ParenClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    param_exprs.push(self.parse_spread_expression().unwrap());
                }
                Some(Token::Identifier(_)) => {
                    param_exprs.push(self.parse_identifier().unwrap());
                }
                Some(Token::CharLiteral(_) | Token::StringLiteral(_) | Token::NumberLiteral(_)) => {
                    param_exprs.push(self.parse_literal().unwrap());
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
            let _ = &self.skip_formating();
            match &self.current_token {
                Some(Token::CurlyBraceOpen) => {
                    block_expr = Some(self.parse_block(None).unwrap());
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
            block_expr = Some(self.parse_block(None).unwrap());
        }

        match &ident {
            Some(ident) => match block_expr {
                Some(expr) => {
                    if expr
                        == Expr::Group(Group {
                            exprs: vec![],
                            op: GroupOp::StatementBlock,
                        })
                    {
                        Ok(Expr::FuncCall(FuncCall {
                            name: ident.to_owned(),
                            args: Box::new(Expr::Group(Group {
                                exprs: param_exprs,
                                op: GroupOp::ParamBlock,
                            })),
                            fields: None,
                            body: Some(Box::new(expr)),
                        }))
                    } else {
                        Ok(Expr::FuncCall(FuncCall {
                            name: ident.to_owned(),
                            args: Box::new(Expr::Group(Group {
                                exprs: param_exprs,
                                op: GroupOp::ParamBlock,
                            })),
                            fields: Some(Box::new(expr)),
                            body: None,
                        }))
                    }
                }
                None => Ok(Expr::FuncCall(FuncCall {
                    name: ident.to_owned(),
                    args: Box::new(Expr::Group(Group {
                        exprs: param_exprs,
                        op: GroupOp::ParamBlock,
                    })),
                    fields: None,
                    body: None,
                })),
            },
            None => {
                // anon block { }
                unimplemented!("Param block without a name")
            }
        }
    }

    fn parse_block(&mut self, ident: Option<Box<Expr>>) -> Result<Expr, ParseError> {
        // either a assignment block, a statement block or a param block
        self.expect_token(vec![Token::CurlyBraceOpen, Token::ParenOpen]);
        let expr = match &self.current_token {
            Some(Token::CurlyBraceOpen) => self.parse_curly_block(ident),
            Some(Token::ParenOpen) => self.parse_paren_block(ident),
            _ => unreachable!(),
        };
        expr
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
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

        let _ = &self.skip_formating();
        match &self.current_token {
            // Atoms
            Some(Token::Identifier(_)) => self.parse_identifier(),
            Some(Token::NumberLiteral(_)) => self.parse_number_literal(),
            Some(Token::StringLiteral(_)) => self.parse_string_literal(),
            Some(Token::BoolLiteral(_)) => self.parse_bool_literal(),
            // Binary should be handled by the caller

            // Unary
            Some(Token::Ellipse) => self.parse_spread_expression(),

            // Grouping
            Some(Token::CurlyBraceOpen) | Some(Token::ParenOpen) => self.parse_block(None),
            Some(Token::EndOfFile) => Ok(Expr::EndOfFile),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let mut expressions = Vec::new();

        while let Some(token) = &self.current_token {
            if token == &Token::EndOfFile {
                break;
            }
            expressions.push(self.parse_expression().unwrap());
        }
        Ok(Expr::Group(Group {
            exprs: expressions,
            op: GroupOp::StatementBlock,
        }))
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
    let name_ast = parser.parse().unwrap();

    // let name_ast_expected =
    //     Expr::StatementBlock(StatementBlock(vec![Expr::Assignment(Assignment {
    //         key: Box::new(Expr::Identifier(Identifier("Name".to_string()))),
    //         value: Box::new(Expr::TypeDef(TypeDef {
    //             name: Box::new(Expr::Identifier(Identifier("Type".to_string()))),
    //             fields: Box::new(Expr::AssignmentBlock(AssignmentBlock(vec![
    //                 Expr::Assignment(Assignment {
    //                     key: Box::new(Expr::Identifier(Identifier("fistName".to_string()))),
    //                     value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
    //                 }),
    //                 Expr::Assignment(Assignment {
    //                     key: Box::new(Expr::Identifier(Identifier("lastName".to_string()))),
    //                     value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
    //                 }),
    //             ]))),
    //         })),
    //     })]));

    let name_ast_expected = Expr::Group(Group {
        op: GroupOp::StatementBlock,
        exprs: vec![Expr::Binary(Binary {
            op: BinaryOp::Assignment,
            left: Box::new(Expr::Atom(Atom::Identifier("Name".to_string()))),
            right: Box::new(Expr::TypeDef(TypeDef {
                name: Box::new(Expr::Atom(Atom::Identifier("Type".to_string()))),
                fields: Box::new(Expr::Group(Group {
                    op: GroupOp::AnonymousType,
                    exprs: vec![
                        Expr::Binary(Binary {
                            op: BinaryOp::Assignment,
                            left: Box::new(Expr::Atom(Atom::Identifier("fistName".to_string()))),
                            right: Box::new(Expr::Atom(Atom::Identifier("String".to_string()))),
                        }),
                        Expr::Binary(Binary {
                            op: BinaryOp::Assignment,
                            left: Box::new(Expr::Atom(Atom::Identifier("LastName".to_string()))),
                            right: Box::new(Expr::Atom(Atom::Identifier("String".to_string()))),
                        }),
                    ],
                })),
            })),
        })],
    });
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
