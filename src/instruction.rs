use cpu::Cpu;
use std::rc::Rc;

#[derive(Debug,PartialEq,Clone)]
pub enum InstKind {
    NoOperand,
    N,
    NN,
    Displacement,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Operand {
    Reg(u8),
    Reg16(u8),
    Mem(Rc<Operand>),
    Immediate8,
    Immediate16,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Operation {
    Nop,
    Ld(Operand, Operand),
    Add(Operand, Operand),
}

#[derive(Debug,PartialEq,Clone)]
pub struct Instruction {
    pub kind: InstKind,
    pub op: Operation,
}

impl Instruction {
    pub fn exec(&self, mut cpu: &mut Cpu, args: Vec<u8>) {
        match self.op {
            Operation::Nop => {}
            Operation::Ld(ref lop, ref rop) => {
                self.load(&mut cpu, args);
            }
            Operation::Add(ref lop, ref rop) => {}
        }
    }

    fn load(&self, cpu: &mut Cpu, args: Vec<u8>) {
        let (left, right) = match self.op {
            Operation::Ld(ref lop, ref rop) => (lop.clone(), rop.clone()),
            _ => unreachable!(),
        };
        match (left, right) {
            (Operand::Reg16(ref num), ref Immediate16) => {
                let val: u16 = ((args[1] as u16) << 8) | (args[0] as u16);
                // cpu.set16(*num, val);
            }
            _ => unreachable!(),
        }
    }
}
