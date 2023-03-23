use std::collections::{VecDeque, HashMap};

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
    Mark(&'static str),
    Goto(&'static str),
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
    pub opptr: usize,
}

impl Regs {
    fn resolve(&self, reg: Reg) -> &f64 {
        match reg {
            Reg::A => &self.a,
            Reg::B => &self.a,
            Reg::C => &self.a,
            Reg::OpPtr => panic!("ты шо дурак"),
        }
    }

    fn resolve_mut(&mut self, reg: Reg) -> &mut f64 {
        match reg {
            Reg::A => &mut self.a,
            Reg::B => &mut self.a,
            Reg::C => &mut self.a,
            Reg::OpPtr => panic!("ты шо дурак"),
        }
    }
}

#[derive(Default, Debug)]
pub struct VM {
    code: VecDeque<Op>,
    stack: VecDeque<f64>,
    stack_size: Option<usize>,
    marks: HashMap<&'static str, usize>,
    regs: Regs,
}

impl VM {
    pub fn with_sized_stack(stack_size: usize) -> Self {
        Self { stack_size: Some(stack_size), ..Default::default() }
    }

    pub fn load(&mut self, code: &[Op]) {
        self.code.extend(code);
    }

    pub fn exec(&mut self) -> VMResult {
        loop {
            let op = if let Some(val) = self.code.get(self.regs.opptr).cloned() {
                val
            } else {
                return Ok(());
            };
            match op {
                Op::Push(val) => {
                    if let Some(stack_size) = self.stack_size {
                        if self.stack.len() == stack_size {
                            Err(ExecutionError::StackOverflow)?;
                        }
                    }
                    self.stack.push_back(self.retrieve_value(val));
                },
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
                Op::Mark(id) => drop(self.marks.insert(id, self.regs.opptr)),
                Op::Goto(id) => if let Some(index) = self.marks.get(id).cloned() {
                    self.regs.opptr = index;
                } else {
                    Err(ExecutionError::NoSuchMark)?
                }
            }
            self.regs.opptr += 1;
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
    StackOverflow,
    ZeroDivisionError,
    NoSuchMark,
}
