use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::{Linkage, Module};
use inkwell::passes::PassManager;
use inkwell::targets::Target;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{
    AggregateValueEnum, AnyValueEnum, BasicMetadataValueEnum, BasicValue, BasicValueEnum,
    FloatValue, FunctionValue, PointerValue,
};
use inkwell::{AddressSpace, FloatPredicate, OptimizationLevel};
use std::convert::Into;

use crate::ast::{
    Atom, Binary, BinaryOp, Expr, Group, GroupOp, Literal, Ternary, TernaryOp, Unary, UnaryOp,
};
use crate::parser;

use std::collections::HashMap;

#[derive(Debug, Clone)] 
pub enum SymbolKind {
    Function,
    Variable,
    Type,
    FieldDef
}

#[derive(Debug, Clone)] 
pub struct Symbol <'ctx> {
    name: String,
    ptr : PointerValue<'ctx>,
    kind: SymbolKind,
}


#[derive(Debug)]
struct SymbolTable<'ctx> {
    parent: Option<&'ctx SymbolTable<'ctx>>,
    symbols: HashMap<String, Symbol<'ctx>>,
}

impl<'ctx> SymbolTable<'ctx> {
    fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    fn with_parent(parent: &'ctx SymbolTable<'ctx>) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, value: Symbol<'ctx>) {
        self.symbols.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<Symbol> {
        self.symbols
            .get(name)
            .cloned()
            .or_else(|| self.parent?.get(name))
    }
}

#[derive(Debug)]
pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    symbol_table: SymbolTable<'ctx>,
}

#[derive(Debug)]
pub enum CodeGenError {
    UnexpectedExpr(Expr),
    UnexpectedEOF,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn new(context: &'ctx Context, builder: &'a Builder<'ctx>, module: &'a Module<'ctx>) -> Self {
        let symbol_table = SymbolTable::new();
        Self {
            context,
            builder,
            module,
            symbol_table,
        }
    }

    fn gen_literal(&mut self, literal: &Literal) -> Result<BasicValueEnum, CodeGenError> {
        // create and store the values in symbol table
        // get the ptr value
        Ok(match literal {
            // Other Literal variants ...
            Literal::IntLiteral(value) => self
                .context
                .i64_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::FloatLiteral(value) => {
                self.context.f64_type().const_float(*value as f64).into()
            }
            Literal::BoolLiteral(value) => self
                .context
                .bool_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::CharLiteral(value) => self
                .context
                .i8_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::OctalLiteral(value) => self
                .context
                .i32_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::HexLiteral(value) => self
                .context
                .i8_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::BinaryLiteral(value) => self
                .context
                .i32_type()
                .const_int(*value as u64, false)
                .into(),
            Literal::StringLiteral(value) => {
                let string = self.context.const_string(value.as_bytes(), false);
                let ptr = self.builder.build_alloca(string.get_type(), "string");
                todo!("store the string in ptr")
            }
        })
    }

    fn gen_identifier(&mut self, identifier: &str) -> Result<BasicValueEnum, CodeGenError> {
        match self.symbol_table.get(identifier) {
            Some(value) => Ok(value.into()),
            None => Err(CodeGenError::UnexpectedExpr(Expr::Atom(Atom::Identifier(
                identifier.to_string(),
            )))),
        }
    }

    fn gen_atom(&mut self, atom: &Atom) -> Result<BasicValueEnum, CodeGenError> {
        match atom {
            Atom::Literal(literal) => self.gen_literal(literal),
            Atom::Identifier(identifier) => self.gen_identifier(identifier),
            Atom::EndOfFile => todo!(),
        }
    }

    fn gen_assignment(&mut self, expr: &Binary) -> Result<(), CodeGenError> {
        match expr {
            Binary { op, left, right } => match op {
                BinaryOp::Assignment => {
                    // check the to see if the left is identifier alreadt exists in symbol table
                    // if not then create a new one
                    todo!("check the symbol table")
                }
                _ => todo!(),
            },
        }
    }

    fn gen_unary(&mut self, expr: &Unary) -> Result<(), CodeGenError> {
        match expr {
            Unary { op, expr } => match op {
                UnaryOp::SpreadExpr => todo!("SpreadExpr"),
            },
        }
    }

    fn gen_binary(&mut self, expr: &Binary) -> Result<(), CodeGenError> {
        match expr {
            Binary { op, left, right } => match op {
                BinaryOp::Assignment => self.gen_assignment(expr),
                BinaryOp::Accessor => todo!("Accessor"),
                BinaryOp::FieldDef => todo!("FieldDef"),
                BinaryOp::TypeDef => todo!("TypeDef"),
                BinaryOp::Invoke => todo!("Invoke"),
            },
        }
    }

    fn gen_group(&mut self, group: &Group) -> Result<(), CodeGenError> {
        match group {
            Group { op, exprs } => match op {
                GroupOp::AssignmentBlock => {
                    for expr in exprs {
                        match expr {
                            Expr::Binary(binary) => self.gen_assignment(binary)?,
                            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
                        }
                    }
                    Ok(())
                }
                GroupOp::StatementBlock => {
                    for expr in exprs {
                        self.gen_expr(expr)?;
                    }
                    Ok(())
                }
                _ => todo!(" other group ops "),
            },
        }
    }

    fn gen_ternary(&mut self, ternary: &Ternary) -> Result<(), CodeGenError> {
        match ternary {
            Ternary {
                op,
                left,
                middle,
                right,
            } => match op {
                TernaryOp::InvokeDefine => todo!("InvokeDefine"),
            },
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::Atom(atom) => {
                self.gen_atom(atom);
                Ok(())
            }
            Expr::Unary(_) => todo!(),
            Expr::Binary(binary) => self.gen_binary(binary),
            Expr::Ternary(ternary) => self.gen_ternary(ternary),
            Expr::Group(group) => self.gen_group(group),
            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }

    pub fn compile(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        self.gen_expr(expr)
    }
}

#[test]
pub fn test_codeGen() {
    // let path = "src/tests/test.zen";
    // let file = std::fs::read_to_string(path).unwrap();
    let file = "a: 10".to_string();
    let mut parser = parser::Parser::new(&file);
    let ast = parser.parse().unwrap();
    println!("{:#?}", ast);

    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let mut codegen = CodeGen::new(&context, &builder, &module);
    codegen.compile(&ast);
    module.print_to_stderr();
}
