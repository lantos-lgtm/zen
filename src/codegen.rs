use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::Target;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{
    BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, FunctionValue, PointerValue,
};
use inkwell::{FloatPredicate, OptimizationLevel};
use std::convert::Into;

use crate::ast::{Atom, Binary, BinaryOp, Expr, Group, Literal};
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
    fn new(
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
            Expr::Atom(Atom::Literal(Literal::IntLiteral(int))) => {
                Ok(self.context.i64_type().const_int(*int as u64, false).into())
            }
            Expr::Atom(Atom::Literal(Literal::FloatLiteral(float))) => {
                Ok(self.context.f64_type().const_float(*float as f64).into())
            }
            Expr::Atom(Atom::Literal(Literal::StringLiteral(string))) => {
                let string_const = self.context.const_string(string.as_bytes(), true);
                let global_string =
                    self.module
                        .add_global(string_const.get_type(), None, "global_string");
                global_string.set_initializer(&string_const);
                Ok(global_string.as_pointer_value().into())
            }
            _ => Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }

    fn gen_assignment(&mut self, assignment: &Binary) -> Result<(), CodeGenError> {
        let left = &assignment.left;
        let right = &assignment.right;

        let alloca = match left.as_ref() {
            Expr::Atom(Atom::Identifier(identifier)) => {
                // Check if the variable already exists in the symbol table
                match self.symbol_table.get(identifier) {
                    Some(alloca) => *alloca,
                    None => {
                        todo!("Identifier symbol table")

                        // // If the variable doesn't exist, create it
                        // let ty = self.context.i64_type();
                        // let name = identifier.as_str();
                        // let new_alloca = self.builder.build_alloca(ty, name);

                        // // Add the variable to the symbol table mapping
                        // self.symbol_table.insert(identifier.clone(), new_alloca);
                        // new_alloca
                    }
                }
            }
            _ => return Err(CodeGenError::UnexpectedExpr(*left.clone())),
        };

        let value = self.gen_basic_value(right)?;
        self.builder.build_store(alloca, value);
        Ok(())
    }

    fn gen_statement_block(&mut self, expr: &Expr) -> Result<(), CodeGenError> {

        match expr {
            Expr::Group(Group { op, exprs }) => {
                for statement in exprs {
                    match statement {
                        // here we should expect, asignments and function calls
                        Expr::Binary(binary) => match binary {
                            Binary {
                                op,
                                left: _,
                                right: _,
                            } => match op {
                                BinaryOp::Assignment => self.gen_assignment(binary)?,
                                BinaryOp::Accessor => todo!("Accessor"),
                            },
                        },
                        Expr::FuncCall(_) => todo!("FuncCall"),
                        _ => return Err(CodeGenError::UnexpectedExpr(statement.clone())),
                    }
                }
            }
            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
        Ok(())
    }

    fn gen_atom(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::Atom(atom) => match atom {
                Atom::Identifier(identifier) => {
                    match self.symbol_table.get(identifier) {
                        Some(alloca) => {
                            todo!("Identifier symbol table");
                            Ok(())
                        }
                        None => Err(CodeGenError::UnexpectedExpr(expr.clone())),
                    }
                }
                Atom::Literal(_) => todo!(),
                _ => Err(CodeGenError::UnexpectedExpr(expr.clone())),
            },
            _ => Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }
    fn gen_binary(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::Binary(binary) => {
                match binary.op {
                    BinaryOp::Assignment => self.gen_assignment(binary)?,
                    BinaryOp::Accessor => todo!("Accessor not implemented"),
                }
            },
            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
        Ok(())
    }


    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::Atom(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Binary(_) => self.gen_binary(expr),
            Expr::Group(_) => todo!(),
            Expr::TypeDef(_) => todo!(),
            Expr::FuncCall(_) => todo!(),
            _ => return Err(CodeGenError::UnexpectedExpr(expr.clone())),
        }
    }

    pub fn gen(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
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
    let mut symbol_table: HashMap<String, PointerValue> = HashMap::new();
    let mut codegen = CodeGen {
        context: &context,
        builder: &builder,
        module: &module,
        symbol_table: &mut symbol_table,
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
