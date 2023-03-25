use std::{io::{self, Write}, iter};

use crate::vm::VM;

mod codegen;
mod lexer;
mod parser;
mod vm;

fn main() {
    let mut vm = VM::default();
    loop {
        let mut buf = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        let _expr = io::stdin().read_line(&mut buf).unwrap();
        let tokens = match parser::parse(buf.trim()) {
            Ok(tokens) => tokens,
            Err(err) => {
                let offset = iter::repeat(' ').take(err.index + 2).collect::<String>();
                println!("{}↑ {:?}", offset, err.value);
                continue;
            }
        };
        let ast = match lexer::lex(tokens) {
            Ok(ast) => ast,
            Err(err) => {
                let offset = iter::repeat(' ').take(err.token.index + 2).collect::<String>();
                println!("{}↑ {:?}", offset, err.value);
                continue;
            }
        };
        let asm = codegen::gen(ast);
        vm.load(asm);
        vm.exec().unwrap();
        println!("{}", vm.stack().back().unwrap());
    }
}
