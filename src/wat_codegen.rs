use std::collections::HashMap;
use crate::ast::*;

use crate::parser;

pub enum ValueType {
    Int,
    Float,
    Bool,
    Char,
    Octal,
    Hex,
    Binary,
    String,
}


pub struct FunctionDefinition {
    name: String,
    params: Vec<(String, ValueType)>,
    return_type: ValueType,
    body: Vec<Expr>,
}

pub struct watCodeGen {
    indentation_level: usize,
    symbol_table: HashMap<String, ValueType>,
    function_table: HashMap<String, FunctionDefinition>,
    current_function: Option<String>,
    label_counter: usize,
}

impl watCodeGen {
    pub fn new() -> Self {
        Self {
            indentation_level: 0,
            symbol_table: HashMap::new(),
            function_table: HashMap::new(),
            current_function: None,
            label_counter: 0,
        }
    }
    pub fn generate(&self, expr: &Expr) -> String {
        match expr {
            Expr::Atom(atom) => self.atom_to_wat(atom),
            Expr::Unary(unary) => self.unary_to_wat(unary),
            Expr::Binary(binary) => self.binary_to_wat(binary),
            Expr::Ternary(ternary) => self.ternary_to_wat(ternary),
            Expr::Group(group) => self.group_to_wat(group),
        }
    }

    fn literal_to_wat(&self, literal: &Literal) -> String {
        match literal {
            Literal::IntLiteral(i) => format!("(int {})", i),
            Literal::FloatLiteral(f) => format!("(float {})", f),
            Literal::BoolLiteral(b) => format!("(bool {})", b),
            Literal::CharLiteral(c) => format!("(char {})", c),
            Literal::StringLiteral(s) => format!("(string {})", s),
            Literal::OctalLiteral(o) => format!("(octal {})", o),
            Literal::HexLiteral(h) => format!("(hex {})", h),
            Literal::BinaryLiteral(b) => format!("(binary {})", b),
        }
    }

    fn atom_to_wat(&self, atom: &Atom) -> String {
        match atom {
            Atom::Identifier(id) => format!("(identifier {})", id),
            Atom::Literal(literal) => self.literal_to_wat(literal),
            Atom::EndOfFile => "(end)".to_string(),
        }
    }
    fn unary_to_wat(&self, unary: &Unary) -> String {
        let expr = self.generate(&*unary.expr);

        match unary.op {
            UnaryOp::SpreadExpr => format!("(spread {})", expr),
        }
    }


    fn binary_to_wat(&self, binary: &Binary) -> String {
        let left = self.generate(&*binary.left);
        let right = self.generate(&*binary.right);

        // if Ident: Fn { } we are funcDef
        // if Ident: Fn() { } we are funcCall

        "".to_string()

    }
    fn ternary_to_wat(&self, ternary: &Ternary) -> String {
        let left = self.generate(&*ternary.left);
        let middle = self.generate(&*ternary.middle);
        let right = self.generate(&*ternary.right);

        match ternary.op {
            TernaryOp::FuncDefInvoke => format!("(invoke {} {} {})", left, middle, right),
        }
    }

    fn group_to_wat(&self, group: &Group) -> String {
        let exprs = group.exprs.iter().map(|expr| self.generate(expr)).collect::<Vec<_>>().join(" ");

        match group.op {
            GroupOp::AssignmentBlock => format!("(block {})", exprs),
            GroupOp::StatementBlock => format!("(block {})", exprs),
            GroupOp::ParamBlock => format!("(block {})", exprs),
            GroupOp::AnonymousType => format!("(type {})", exprs),
        }
    }

}



#[test]
fn test_generate_binary() {
    let codegen = watCodeGen::new();
    // let file = std::fs::read_to_string(path).unwrap();
    let file = "main: Fn {
    a: void,
    r: int,
    fn: {
        a: 1
        return(a + 3)
    }
   }".to_string();
   
    let mut parser = parser::Parser::new(&file);
    let ast = parser.parse().unwrap();
    println!("{:#?}", ast);
    let wat = codegen.generate(&ast);
    println!("{}", wat);
    
}