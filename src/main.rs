use crate::vm::{VM};

mod codegen;
mod lexer;
mod parser;
mod vm;

fn main() {
    let expr = "+ (* 2 3) (/ 8 2))";
    let tokens = parser::parse(expr).unwrap();
    println!("Tokens:");
    for i in &tokens {
        println!("{i:?}");
    }
    let ast = lexer::lex(tokens).unwrap();
    println!("AST: {:?}", &ast);
    let asm = codegen::gen(ast);
    println!("ASM:");
    for i in &asm {
        println!("{i:?}");
    }
    let mut vm = VM::default();
    vm.load(asm);
    vm.exec().unwrap();
    println!("Answer: {}", vm.stack()[0]);
}
