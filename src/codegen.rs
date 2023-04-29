use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::Target;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, PointerValue, BasicValue, BasicValueEnum};
use inkwell::{FloatPredicate, OptimizationLevel};
use std::convert::Into;

use crate::ast::{Expr, StatementBlock, Identifier, Assignment, Literal};
use crate::parser;
use std::collections::HashMap;


#[derive(Debug)]
pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub symbol_table: &'a mut HashMap<String, PointerValue<'ctx>>,
}

#[derive(Debug)]
pub enum CodeGenError {
    UnexpectedExpr(Expr),
    UnexpectedEOF,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn new (
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        symbol_table: &'a mut HashMap<String, PointerValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
            symbol_table,
        }
    }
    fn gen_basic_value(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match expr {
            Expr::Literal(Literal::IntLiteral(int)) => {
                Ok(self.context.i32_type().const_int(*int as u64, false).into())
            },
            Expr::Literal(Literal::FloatLiteral(float)) => {
                Ok(self.context.f32_type().const_float(*float as f64).into())
            },
            Expr::Literal(Literal::StringLiteral(string)) => {
                let string_const = self.context.const_string(string.as_bytes(), true);
                let global_string = self
                    .module
                    .add_global(string_const.get_type(), None, "global_string");
                global_string.set_initializer(&string_const);
                Ok(global_string.as_pointer_value().into())
            }
            _ => Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }

    fn gen_assignment(&mut self, assignment: &Assignment) -> Result<(), CodeGenError> {
        let key = &assignment.key;
        let value = &assignment.value;
    
        let alloca = match key.as_ref() {
            Expr::Identifier(Identifier(identifier)) => {
                // Check if the variable already exists in the symbol table
                match self.symbol_table.get(identifier) {
                    Some(alloca) => *alloca,
                    None => {
                        // If the variable doesn't exist, create it
                        let new_alloca = self.builder.build_alloca(self.context.i32_type(), &identifier);
                        self.symbol_table.insert(identifier.clone(), new_alloca);
                        new_alloca
                    }

                }
            }
            Expr::AnonymousType(_) => todo!(),
            _ => return Err(CodeGenError::UnexpectedExpr(Expr::Assignment(assignment.clone()))),
        };

        let value = self.gen_basic_value(value)?;
        self.builder.build_store(alloca, value);
        Ok(())
    }
    

    fn gen_statement_block(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::StatementBlock(StatementBlock(statements)) => {
                for statement in statements {
                    self.gen_expr(statement).unwrap();
                }
            }
            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
        Ok(())
    }


    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::SpreadExpr(_) => todo!(),
            Expr::Assignment(assignment) => self.gen_assignment(assignment),
            Expr::Accessor(_) => todo!(),
            Expr::AssignmentBlock(_) => todo!(),
            Expr::StatementBlock(_) => self.gen_statement_block(expr),
            Expr::ParamBlock(_) => todo!(),
            Expr::AnonymousType(_) => todo!(),
            Expr::TypeDef(_) => todo!(),
            Expr::FuncCall(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn gen(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        self.gen_expr(expr)
    }
}

#[test]
pub fn test_codeGen() {
    let path = "src/tests/test.zen";
    let file = std::fs::read_to_string(path).unwrap();
    let mut parser = parser::Parser::new(&file);
    let ast = parser.parse().unwrap();
    println!("{:#?}", ast);

    let context     = Context::create();
    let module       = context.create_module("test");
    let builder     = context.create_builder();
    let mut symbol_table: HashMap<String, PointerValue> = HashMap::new();
    let mut codegen = CodeGen {
        context:        &context,
        builder:        &builder,
        module:         &module,
        symbol_table:   &mut symbol_table,
        
    };
    codegen.gen(&ast).unwrap();
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

    builder.build_return(Some(&sum));

    type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

    let sum = unsafe { execution_engine.get_function("sum").ok() }.unwrap() as JitFunction<SumFunc>;

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }
}
