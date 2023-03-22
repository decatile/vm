use vm::Op;

mod vm;

fn main() {
    let mut x = vm::VM::new();
    x.load(&[Op::Push(vm::Value::Lit(0.22))]);
}
