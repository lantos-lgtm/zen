

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::FloatPredicate;


pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    // customs


}


pub enum CodeGenError {
    UnexpectedExpr(Expr),
    UnexpectedEOF,
}

impl <'a, 'ctx>CodeGen<'a, 'ctx> {

    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::SpreadExpr(_) => todo!(),
            Expr::Assignment(_) => todo!(),
            Expr::Accessor(_) => todo!(),
            Expr::AssignmentBlock(_) => todo!(),
            Expr::StatementBlock(_) => todo!(),
            Expr::ParamBlock(_) => todo!(),
            Expr::AnonymousType(_) => todo!(),
            Expr::TypeDef(_) => todo!(),
            Expr::FuncCall(_) => todo!(),
            _ => todo!(),
        }
        todo!()
    }

    fn gen(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        todo!()
    }
    
}


#[test]
pub fn test_compiler() {
    let file = std::fs::read_to_string("./test/test.zen").expect("Failed to read file");
    let mut parser = parser::Parser::new(&file);
    let ast = parser.parse();
    println!("{:#?}", ast);
    
    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();
    let mut codegen = CodeGen {
        context: &context,
        builder: &builder,
        module: &module,
    };
    codegen.gen(&ast).unwrap();
    module.print_to_stderr();
}