use crate::ast::{Expr, Program, Identifier, TypeDef, Assignment, AssignmentBlock, StatementBlock, ParamBlock , Literal, Accessor, FuncCall};
use crate::lexer::{Lexer};
use crate::token::{Token};

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

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
    }
    // expect one or more tokens
    fn expect_token(&mut self, expected: Vec<Token>) {
        if let Some(token) = &self.current_token {
            if !expected.contains(token) {
                panic!("Unexpected token: {:?}, expected: {:?}", token, expected);
            }
        } else {
            panic!("Unexpected EOF, expected: {:?}", expected);
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
                Some(Token::Colon) => {
                    self.parse_assignment(Box::new(Expr::Identifier(identifier)))
                }
                // ident . ident
                Some(Token::Dot) => {
                    self.parse_accessor(Box::new(Expr::Identifier(identifier)))
                }
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
                // skip comments, newlines, commas, whitespace, 
                Some(Token::Comment(_)) | Some(Token::NewLine(_)) | Some(Token::Comma) | Some(Token::WhiteSpace(_)) => {
                    self.next_token();
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
                    print!("found! Statement block signature, expr: {:?}", expr);
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
                    unimplemented!("Assignment block as type def, ident:{:?} exprs:{:?} \n current_token:{:?}", ident, exprs, self.current_token);
                }
            }
        }

       
    }

    fn parse_paren_block(&mut self, ident: Option<Box<Expr>>) -> Expr {
        // a params block
        self.expect_token(vec![Token::ParenOpen]);
        self.next_token();

        // this expects either
        // Spread       ...ident || ...{}
        // Assignment   key : value
        // Ident        ident

        // if we encounter a paren close we can finish parsing
        let mut paramExprs: Vec<Expr> = Vec::new();
        let mut blockExpr: Option<Expr> = None;
        loop {
            match &self.current_token {
                Some(Token::ParenClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    paramExprs.push(self.parse_spread_expression());
                }
                Some(Token::Identifier(_)) => {
                    paramExprs.push(self.parse_identifier());
                }
                None | Some(Token::EndOfFile) => {
                    panic!("Unexpected EOF, expected: {:?}", Token::ParenClose);
                }
                // should add expected errors
                _ => unreachable!()
            }
        }
        loop {
            match &self.current_token {
                Some(Token::ParenClose) => {
                    self.next_token();
                    break;
                }
                Some(Token::Ellipse) => {
                    paramExprs.push(self.parse_spread_expression());
                }
                Some(Token::Identifier(_)) => {
                    paramExprs.push(self.parse_identifier());
                }
                None | Some(Token::EndOfFile) => {
                    panic!("Unexpected EOF, expected: {:?}", Token::ParenClose);
                }
                // should add expected errors
                _ => unreachable!()
            }
        }

        // check to see if next token is a curly brace open
        if &self.current_token == &Some(Token::CurlyBraceOpen) {
            blockExpr = Some(self.parse_block(None));
        }

        

        match &ident {
            Some(ident) => {
                match blockExpr {
                    Some(expr) => {
                        if expr == Expr::StatementBlock(StatementBlock(vec![])) {
                            Expr::FuncCall(
                                FuncCall {
                                    name: ident.to_owned(),
                                    args: Box::new(Expr::ParamBlock(ParamBlock(paramExprs))),
                                    fields: None,
                                    body: Some(Box::new(expr)),
                                }
                            )
                        } else {
                            Expr::FuncCall(
                                FuncCall {
                                    name: ident.to_owned(),
                                    args: Box::new(Expr::ParamBlock(ParamBlock(paramExprs))),
                                    fields: Some(Box::new(expr)),
                                    body: None,
                                }
                            )
                        }
                    },
                    None => {
                        Expr::FuncCall(
                            FuncCall {
                                name: ident.to_owned(),
                                args: Box::new(Expr::ParamBlock(ParamBlock(paramExprs))),
                                fields: None,
                                body: None,
                            }
                        )
                    }
                }
            }
            None => {
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
            _ => unreachable!()
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
            
            
            // skip newlines, whitespace, and colons
            Some(Token::Comment(_)) | Some(Token::NewLine(_)) | Some(Token::WhiteSpace(_)) | Some(Token::Colon) => {
                self.next_token();
                self.parse_expression()
            }

            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut expressions = Vec::new();

        while let Some(token) = &self.current_token {
            match token {
                Token::Comment(_) | Token::NewLine(_) | Token::WhiteSpace(_) | Token::Colon => {
                    self.next_token();
                }
                _ => {
                    expressions.push(self.parse_expression());
                }
            }
        }

        Program(expressions)
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
    let name_ast = match parser.parse() {
        Program(expressions) => expressions[0].clone()
    };

    let name_ast_expected = Expr::Assignment(Assignment { 
        key: Box::new(Expr::Identifier(Identifier("Name".to_string()))),
        value: Box::new(Expr::TypeDef(TypeDef {
            name: Box::new(Expr::Identifier(Identifier("Type".to_string()))),
            fields: Box::new(Expr::AssignmentBlock(AssignmentBlock(vec![
                Expr::Assignment(
                    Assignment {
                        key: Box::new(Expr::Identifier(Identifier("fistName".to_string()))),
                        value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
                    }
                ),
                Expr::Assignment(
                    Assignment {
                        key: Box::new(Expr::Identifier(Identifier("lastName".to_string()))),
                        value: Box::new(Expr::Identifier(Identifier("String".to_string()))),
                    }
                ),
            ]))),
        })),
    });
    assert!(name_ast == name_ast_expected);
}
