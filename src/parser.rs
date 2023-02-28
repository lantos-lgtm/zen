use clap::Id;
use serde::{Serialize};
use crate::ast::{Expr, Literal, Identifier, Assignment, Key, Body, SpreadOperator, TypeDef, Fields, Accessor};
use crate::tokenizer::{Token, Tokenizer};


#[derive(Debug, PartialEq, Serialize)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

pub struct Parser<'a> {
    tokens: std::iter::Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.tokens.next().ok_or(ParseError::UnexpectedEOF)
    }

    fn parse_string_literal(&mut self) -> Result<Expr, ParseError> {
        if let Token::StringLiteral(value) = self.next_token()? {
            Ok(Expr::Literal(Literal::StringLiteral(value)))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_number_literal(&mut self) -> Result<Expr, ParseError> {
        if let Token::IntLiteral(value) = self.next_token()? {
            Ok(Expr::Literal(Literal::IntLiteral(value)))
        } else if let Token::FloatLiteral(value) = self.tokens.next().unwrap() {
            Ok(Expr::Literal(Literal::FloatLiteral(value)))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_identifier(&mut self) -> Result<Expr, ParseError> {
        // move this complexity to the parse_primary and other edge cases
        // identifier(...) = L Token::ParenOpen         R
        // identifier{...} = L Token::CurlyBraceOpen    R
        // identifier:...  = L Token::Colon             R
        // identifier)     = L Token::Identifier        R
        // identifier,     = L Token::Identifier        R
        // identifier.     = L Token::Identifier        R
        // identifier)      = L Token::Identifier       R

        if let Token::Identifier(name) = self.next_token()? {
            Ok(Expr::Identifier(Identifier(name)))
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }

    }

    fn parse_ellipse(&mut self) -> Result<Expr, ParseError> {
        // ...identifier
        if let Token::Ellipse = self.next_token()? {
            if let Some(token) = self.tokens.peek() {
                match token {
                    Token::Identifier(_) => {
                        Ok( self.parse_identifier()? )
                    },
                    // Token::CurlyBraceOpen => {
                    //     Ok( self.parse_type_def()? )
                    // },
                    _ => Err(ParseError::UnexpectedToken(self.tokens.next().unwrap())),
                    
                }
            } else {
                Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
            }
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_dot(&mut self, lhs: Expr) -> Result<Expr, ParseError> {
        // ident.ident[()|{}|:,.]
        // ident.ident
        // L , Token::Dot, R
        // make sure lhs is an identifier
        if let Expr::Identifier(object) = lhs {
            let expr = Expr::Accessor(Accessor {
                object: object,
                property: Box::new(self.parse_expr()?),
            });
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
        // todo!()
    }

    fn parse_key(&mut self, key: Expr) -> Result<Key, ParseError> {
        // identifier
        // { ... }
        // { key: Type }

        if let Token::Identifier(name) = self.next_token()? {
            Ok(Key::Key(Identifier(name)))
        } else if let Token::CurlyBraceOpen = self.next_token()? {
            todo!()
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_assignment(&mut self, lhs: Expr) -> Result<Expr, ParseError> {
        // identifier: ...  = L Token::Colon, R
        // {...}: ...       = L Token::Colon, R
        let key = self.parse_key(lhs)?;
        if let asisgnment = self.next_token()? {
            let expr = Expr::Assignment(Assignment {
                key:  key,
                value: Box::new(self.parse_expr()?),
            });
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))   
        }
    }


    fn parse_body(&mut self) -> Result<Expr, ParseError> {
        // { ... }
        // while token != Token::CurlyBraceClose { keep parsing }
        todo!()
     
    }  
    fn parse_fields(&mut self) -> Result<Fields, ParseError> {
        // { ... }
        // while token != Token::CurlyBraceClose { keep parsing }
        todo!()
    }

    fn parse_type_def(&mut self, lhs: Expr) ->  Result<Expr, ParseError> {
        // Type { ... }
        // Type { key: Type, ... }
        if let Token::Identifier(name) = self.next_token()? {
            let expr = Expr::TypeDef(TypeDef {
                name: Identifier(name),
                fields: self.parse_fields()?,
            });
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedToken(self.tokens.next().unwrap()))
        }
    }

    fn parse_call(&mut self, lhs: Expr) -> Result<Expr, ParseError> {
        // identifier(...) = L Token::ParenOpen, R
        // {...}(...)      = L Token::ParenOpen, R // call a function instantly
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        // the main parse route
        // Should this be looping
        let mut lhs = self.parse_primary()?;
        // this should be our lhs
        let mut expr : Option<Expr> = None;
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::Newline(_) | Token::Comma=> {
                    self.tokens.next();
                }
                Token::Colon => {
                    expr = Some(self.parse_assignment(lhs)?);
                },
                Token::Dot => {
                    expr = Some(self.parse_dot(lhs)?);
                },
                Token::Ellipse => {
                    expr = Some(self.parse_ellipse()?);
                },
                Token::CurlyBraceOpen => {
                    // probably a typedef
                    expr = Some(self.parse_type_def(lhs)?);
                },
                Token::ParenOpen => {
                    // probably a function call
                    expr = Some(self.parse_call(lhs)?);
                },
                // The following tokens are handled in the respected open token
                // Token::CurlyBraceClose 
                // Token::ParenClose
                _ => {
                    expr = None;
                }
            }
        } 

        if let Some(expr) = expr {
            Ok(expr)
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        // parse simplest types such as literals or variables
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::CurlyBraceOpen => self.parse_body(),
                Token::Identifier(_) => self.parse_identifier(),
                _ => Err(ParseError::UnexpectedToken(self.tokens.next().unwrap())),
            }
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_program(&mut self) -> Result<Expr, ParseError> {
        // parses the entry of the program. ssuch as statments, declarations and functions
        // what can we expect to see on at the root of the file or body
        // identifier
        // {}
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::CurlyBraceOpen => self.parse_body(),
                Token::Identifier(_) => self.parse_identifier(),
                _ => Err(ParseError::UnexpectedToken(self.tokens.next().unwrap())),
            }
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr()
    }
}

#[test]
fn test_ast() {

    let name_str = "Name: Type {
        fistName: String,
        lastName: String,
    }";
    let tokenizer = Tokenizer::new(name_str);
    let tokens = tokenizer.collect::<Vec<Token>>();
    println!("Parsing: {:?}", tokens);
    let mut parser = Parser::new(name_str);
    let name_ast = match(parser.parse()) {
        Ok(ast) => ast,
        Err(e) => panic!("Error: {:?}", e)
    };

    println!("{:?}", name_ast);

    let name_ast_expected = Expr::Assignment(Assignment { 
        key: Key::Key(Identifier("Name".to_string())),
        value: Box::new(Expr::TypeDef(TypeDef{
            name: Identifier("Type".to_string()),
            fields: Fields(vec![
                Assignment {
                    key: Key::Key(Identifier("fistName".to_string())),
                    value: Box::new(Expr::TypeDef(TypeDef{
                        name: Identifier("String".to_string()),
                        fields: Fields(vec![]),
                    })),
                },
                Assignment {
                    key: Key::Key(Identifier("lastName".to_string())),
                    value: Box::new(Expr::TypeDef(TypeDef{
                        name: Identifier("String".to_string()),
                        fields: Fields(vec![]),
                    })),
                },
            ]),
        }))
    });


    assert!(name_ast == name_ast_expected);
}