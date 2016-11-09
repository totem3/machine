use cpu::Cpu;
use std::rc::Rc;

pub const REG_B: u8 = 0;
pub const REG_C: u8 = 1;
pub const REG_D: u8 = 2;
pub const REG_E: u8 = 3;
pub const REG_H: u8 = 4;
pub const REG_L: u8 = 5;
pub const REG_HLM: u8 = 6;
pub const REG_A: u8 = 7;

pub const REG_BC: u8 = 0;
pub const REG_DE: u8 = 1;
pub const REG_HL: u8 = 2;
pub const REG_SP: u8 = 3;
pub const REG_AF: u8 = 4;

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

pub fn nop() -> Instruction {
    Instruction {
        kind: InstKind::NoOperand,
        op: Operation::Nop,
    }
}
pub fn ld8(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NN,
        op: Operation::Ld(reg, Operand::Immediate8),
    }
}

pub fn ld16(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NN,
        op: Operation::Ld(reg, Operand::Immediate16),
    }
}

pub fn ld_a2m(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NoOperand,
        op: Operation::Ld(Operand::Mem(Rc::new(reg)), Operand::Reg(REG_A)),
    }
}

pub fn ld_r2mi(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NN,
        op: Operation::Ld(Operand::Mem(Rc::new(Operand::Immediate16)), reg),
    }
}

pub fn ld_m2a(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NoOperand,
        op: Operation::Ld(Operand::Reg(REG_A), Operand::Mem(Rc::new(reg))),
    }
}

pub fn ld_mi2r(reg: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NN,
        op: Operation::Ld(reg, Operand::Mem(Rc::new(Operand::Immediate16))),
    }
}

pub fn add16(reg1: Operand, reg2: Operand) -> Instruction {
    Instruction {
        kind: InstKind::NoOperand,
        op: Operation::Add(reg1, reg2),
    }
}
