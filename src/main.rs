use vm::{Op, VM, Value};

mod vm;

fn main() {
    let mut x = VM::with_sized_stack(1024);
    x.load(&[
        Op::Write(Value::Lit(104.)),
        Op::Write(Value::Lit(101.)),
        Op::Write(Value::Lit(108.)),
        Op::Write(Value::Lit(108.)),
        Op::Write(Value::Lit(111.)),
    ]);
    x.exec().unwrap();
    println!("{}", x.io().1.into_iter().collect::<String>());
}
