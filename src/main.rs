use vm::{Op, VM};

mod vm;

fn main() {
    let mut x = VM::with_sized_stack(1024);
    x.load(&[Op::Push(vm::Value::Lit(0.22))]);
    x.exec().unwrap();
    println!("{:?} {:?}", x.regs(), x.stack());
}
