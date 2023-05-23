use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::{Module, Linkage};
use inkwell::passes::PassManager;
use inkwell::targets::Target;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, FunctionValue, PointerValue, AggregateValueEnum, AnyValueEnum,
};
use inkwell::{FloatPredicate, OptimizationLevel, AddressSpace};
use std::convert::Into;

use crate::ast::{Atom, Binary, BinaryOp, Expr, Group, Literal};
use crate::parser;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    symbol_table: HashMap<String, PointerValue<'a>>,
}

#[derive(Debug)]
pub enum CodeGenError {
    UnexpectedExpr(Expr),
    UnexpectedEOF,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
    ) -> Self {
        let symbol_table = HashMap::new();

        Self {
            context,
            builder,
            module,
            symbol_table,
        }
    }

    fn gen_literal(&mut self, literal: &Literal) -> Result<(), CodeGenError> {

        // create and store the values in symbol table
        // get the ptr value
        let ptr:PointerValue = match literal {
            // Other Literal variants ...
            Literal::IntLiteral(value) => self.context.i64_type().const_int(*value as u64, false).into(),
            Literal::FloatLiteral(value) => self.context.f64_type().const_float(*value as f64).into(),
            Literal::BoolLiteral(value) => self.context.bool_type().const_int(*value as u64, false).into(),
            Literal::CharLiteral(value) => self.context.i8_type().const_int(*value as u64, false).into(),
            Literal::OctalLiteral(value) => self.context.i32_type().const_int(*value as u64, false).into(),
            Literal::HexLiteral(value) => self.context.i8_type().const_int(*value as u64, false).into(),
            Literal::BinaryLiteral(value) => self.context.i32_type().const_int(*value as u64, false).into(),
            Literal::StringLiteral(value) => {
                let string = self.context.const_string(value.as_bytes(), false);
                let ptr = self.builder.build_alloca(string.get_type(), "string");
                self.builder.build_store(ptr, string);
                ptr
            },
        };
        Ok(())
    }

    fn gen_identifier(&mut self, identifier: &str) -> Result<(), CodeGenError> {
        let pointer = self.symbol_table.get(identifier).unwrap();
        let value  = self.builder.build_load(pointer.get_type(), *pointer, identifier);
        Ok(())
    }

    fn gen_atom(&mut self, atom: &Atom) -> Result<(), CodeGenError> {   
        let _llvm_value = match atom {
            Atom::Literal(literal) => self.gen_literal(literal),
            Atom::Identifier(identifier) => self.gen_identifier(identifier),
            Atom::EndOfFile => return Err(CodeGenError::UnexpectedEOF),
        };
        Ok(())
    } 

    fn gen_assignment(&mut self, expr: &Binary) -> Result<(), CodeGenError> {

        todo!()
    }

    fn gen_binary(&mut self, expr: &Binary) -> Result<(), CodeGenError> {
        match expr {
            Binary {
                op,
                left,
                right,
            } => match op {
                BinaryOp::Assignment => self.gen_assignment(expr),
                BinaryOp::Accessor => todo!("Accessor"),
                BinaryOp::FieldDef => todo!("FieldDef"),
                BinaryOp::TypeDef => todo!("TypeDef"),
                BinaryOp::Invoke => todo!("Invoke"),
            },
        }
    }


    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::Atom(atom) => self.gen_atom(atom),
            Expr::Unary(_) => todo!(),
            Expr::Binary(binary) => self.gen_binary(binary),
            Expr::Ternary(ternary) => todo!(),
            Expr::Group(_) => todo!(),
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

#[test]
pub fn test() {
    let context = Context::create();
    let module = context.create_module("sum");
    let builder = context.create_builder();

    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    let function = module.add_function("sum", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();
    let z = function.get_nth_param(2).unwrap().into_int_value();

    let sum = builder.build_int_add(x, y, "sum");
    let sum = builder.build_int_add(sum, z, "sum");

    let w_ptr = builder.build_alloca(i64_type, "w");
    let i64_zero = i64_type.const_int(10, false);

    builder.build_store(w_ptr, i64_zero);

    // retrieve the value of w
    let w = builder.build_load(i64_type, w_ptr, "w").into_int_value();

    let sum = builder.build_int_add(sum, w, "sum");
    builder.build_return(Some(&sum));

    type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

    let sum = unsafe { execution_engine.get_function("sum").ok() }.unwrap() as JitFunction<SumFunc>;

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z + 10);
    }
}
