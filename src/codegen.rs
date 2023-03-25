use crate::{
    lexer::Expr,
    parser::OpType,
    vm::{Op, Reg, Value},
};

pub fn gen(ast: Box<Expr>) -> Vec<Op> {
    match ast.as_ref() {
        Expr::Number(num) => vec![Op::Push(Value::Lit(*num))],
        Expr::Binary(op, lhs, rhs) => {
            let mut res = vec![];
            res.extend(gen(lhs.clone()));
            res.extend(gen(rhs.clone()));
            res.extend([
                Op::Pop(Reg::BX),
                Op::Pop(Reg::AX),
                match op {
                    OpType::Add => Op::Add(Reg::AX, Value::Reg(Reg::BX)),
                    OpType::Sub => Op::Sub(Reg::AX, Value::Reg(Reg::BX)),
                    OpType::Mul => Op::Mul(Reg::AX, Value::Reg(Reg::BX)),
                    OpType::Div => Op::Div(Reg::AX, Value::Reg(Reg::BX)),
                },
                Op::Push(Value::Reg(Reg::AX)),
            ]);
            res
        }
    }
}
