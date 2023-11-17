use inkwell::builder::{Builder, BuilderError};
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
    FieldDef,
}

#[derive(Debug, Clone)]
pub struct Symbol<'ctx> {
    name: String,
    ptr: PointerValue<'ctx>,
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
    BuilderError(BuilderError),
}
impl From<BuilderError> for CodeGenError {
    fn from(error: BuilderError) -> Self {
        CodeGenError::BuilderError(error)
    }
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
                let i8_type = self.context.i8_type();
                let str_type = i8_type.array_type(value.len() as u32);

                let string = self.context.const_string(value.as_bytes(), false);

                let global = self.module.add_global(str_type, None, "str");
                global.set_initializer(&string);
                global.as_pointer_value().into()
            }
        })
    }

    fn gen_identifier(&mut self, identifier: &str) -> Result<BasicValueEnum, CodeGenError> {
        match self.symbol_table.get(identifier) {
            Some(symbol) => Ok(symbol.ptr.into()),
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
            Binary { op, left, right } => match *op {


                BinaryOp::Assignment => {
                    match &**left {
                        Expr::Atom(Atom::Identifier(ident)) => {
                            Ok(())
                        }
                        Expr::Group(Group {
                            op: GroupOp::AssignmentBlock,
                            exprs,
                        }) => {
                            // Spread
                            // { ident, ident... } : expr
                            todo!("Handle spread")
                        }
                        Expr::Binary(Binary {
                            op: BinaryOp::Accessor,
                            left,
                            right,
                        }) => {
                            // Field accessor
                            // ident.ident : expr
                            todo!("Handle field accessor")
                        }
                        _ => Err(CodeGenError::UnexpectedExpr(*left.clone()))?,
                    }
                }
                _ => Err(CodeGenError::UnexpectedExpr(Expr::Binary(expr.clone()))),
            },
        }
    }
    fn gen_unary(&mut self, expr: &Unary) -> Result<BasicValueEnum, CodeGenError> {
        match expr {
            Unary { op, expr } => match op {
                UnaryOp::SpreadExpr => {
                    // Generate your BasicValueEnum here
                    // For example:
                    let value = self.context.i32_type().const_int(0, false).into();
                    Ok(value)
                }
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
                            Expr::Binary(binary) => {
                                println!("About to call gen_assignment");
                                self.gen_assignment(binary)?
                            }
                            _ => {
                                println!("About to return UnexpectedExpr error");
                                return Err(CodeGenError::UnexpectedExpr(expr.clone()));
                            }
                        }
                    }
                    Ok(())
                }
                GroupOp::StatementBlock => {
                    for expr in exprs {
                        println!("About to call gen_expr");
                        self.gen_expr(expr)?;
                    }
                    Ok(())
                }
                _ => {
                    println!("About to hit todo! macro");
                    todo!(" other group ops ")
                }
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
                TernaryOp::FuncDefInvoke => todo!("InvokeDefine"),
            },
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum, CodeGenError> {
        match expr {
            Expr::Atom(atom) => self.gen_atom(atom),
            Expr::Unary(unary) => self.gen_unary(unary),
            Expr::Binary(binary) => {
                self.gen_binary(binary)?;
                Ok(self.context.i32_type().const_int(0, false).into()) // replace with actual value
            }
            Expr::Ternary(ternary) => {
                self.gen_ternary(ternary)?;
                Ok(self.context.i32_type().const_int(0, false).into()) // replace with actual value
            }
            Expr::Group(group) => {
                self.gen_group(group)?;
                Ok(self.context.i32_type().const_int(0, false).into()) // replace with actual value
            }
            _ => Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }

    pub fn compile(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        self.gen_expr(expr)?;
        Ok(())
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
    println!("{:#?}", module.print_to_string().to_string());
}
