use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Lit(f64),
    Reg(Reg),
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Push(Value),
    Pop(Reg),
    Add(Reg, Value),
    Sub(Reg, Value),
    Mul(Reg, Value),
    Div(Reg, Value),
    Nop,
}

#[derive(Clone, Copy, Debug)]
pub enum Reg {
    A,
    B,
    C,
    OpPtr,
}

#[derive(Default, Debug)]
pub struct Regs {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub optr: usize,
}

impl Regs {
    fn resolve(&self, reg: Reg) -> &f64 {
        match reg {
            Reg::A => &self.a,
            Reg::B => &self.a,
            Reg::C => &self.a,
            Reg::OpPtr => panic!(),
        }
    }

    fn resolve_mut(&mut self, reg: Reg) -> &mut f64 {
        match reg {
            Reg::A => &mut self.a,
            Reg::B => &mut self.a,
            Reg::C => &mut self.a,
            Reg::OpPtr => panic!(),
        }
    }
}

pub struct VM {
    code: VecDeque<Op>,
    stack: VecDeque<f64>,
    regs: Regs,
}

impl VM {
    pub fn new() -> Self {
        Self {
            code: VecDeque::new(),
            stack: VecDeque::new(),
            regs: Regs::default(),
        }
    }

    pub fn load(&mut self, code: &[Op]) {
        self.code.extend(code);
    }

    pub fn exec(&mut self) -> VMResult {
        loop {
            let op = if let Some(val) = self.code.get(self.regs.optr).cloned() {
                val
            } else {
                return Ok(());
            };
            match op {
                Op::Push(val) => self.stack.push_back(self.retrieve_value(val)),
                Op::Pop(reg) => {
                    if let Some(val) = self.stack.pop_back() {
                        *self.regs.resolve_mut(reg) = val;
                    } else {
                        Err(ExecutionError::EmptyStack)?
                    }
                }
                Op::Add(reg, val) => *self.regs.resolve_mut(reg) += self.retrieve_value(val),
                Op::Sub(reg, val) => *self.regs.resolve_mut(reg) -= self.retrieve_value(val),
                Op::Mul(reg, val) => *self.regs.resolve_mut(reg) *= self.retrieve_value(val),
                Op::Div(reg, val) => {
                    let x = self.retrieve_value(val);
                    if x == 0. {
                        Err(ExecutionError::ZeroDivisionError)?;
                    }
                    *self.regs.resolve_mut(reg) /= x;
                }
                Op::Nop => {}
            }
        }
    }

    pub fn regs(&self) -> &Regs {
        &self.regs
    }

    pub fn stack(&self) -> &VecDeque<f64> {
        &self.stack
    }

    fn retrieve_value(&self, val: Value) -> f64 {
        match val {
            Value::Lit(lit) => lit,
            Value::Reg(reg) => self.regs.resolve(reg).clone(),
        }
    }
}

pub type VMResult = Result<(), ExecutionError>;

#[derive(Clone, Debug)]
pub enum ExecutionError {
    EmptyStack,
    ZeroDivisionError,
    BadPtr,
}
