use cpu::Cpu;
use cpu::{Register8, Register16};
use memory::Memory;
use ops::Addressable;
use register::RegisterClass;
use std::rc::Rc;

#[derive(Debug,PartialEq,Clone)]
pub enum Operand {
    Reg(Register8),
    Reg16(Register16),
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
    pub op: Operation,
}

fn read16(v: &Vec<u8>) -> u16 {
    (v[1] as u16) << 8 | v[0] as u16
}

impl Instruction {
    pub fn exec(&self, mut cpu: &mut Cpu, dm: &mut Memory, args: Vec<u8>) {
        match self.op {
            Operation::Nop => {}
            Operation::Ld(ref dst, ref src) => {
                match (dst, src) {
                    (&Operand::Reg(ref dst_reg), &Operand::Reg(ref src_reg)) => {
                        let src = match cpu.reg8(&src_reg) {
                            RegisterClass::Single(r) => r,
                            _ => unreachable!("src register must be 8bit register"),
                        };
                        let dst = match cpu.reg8(&dst_reg) {
                            RegisterClass::Single(r) => r,
                            _ => unreachable!("dst register must be 8bit register"),
                        };
                        let val = Cpu::get_register_value(&src);
                        Cpu::set_register_value(dst, val);
                    }
                    (&Operand::Reg(ref dst_reg), &Operand::Immediate8) => {
                        let val = args[0];
                        match cpu.reg8(&dst_reg) {
                            RegisterClass::Single(dst) => {
                                Cpu::set_register_value(dst, val);
                            }
                            _ => unreachable!("dst register must bi 8bit register"),
                        };
                    }
                    (&Operand::Reg(ref dst_reg), &Operand::Mem(ref src)) => {
                        let dst = match cpu.reg8(&dst_reg) {
                            RegisterClass::Single(r) => r,
                            _ => unreachable!("dst register must bi 8bit register"),
                        };
                        match **src {
                            Operand::Reg16(ref addr_reg) => {
                                match addr_reg {
                                    // IX, IY
                                    &Register16::IX | &Register16::IY => {
                                        let reg = match cpu.reg16(&addr_reg) {
                                            RegisterClass::Double(reg) => reg,
                                            _ => unreachable!("register must be IX or IY"),
                                        };
                                        let addr = Cpu::get_register_value(&reg);
                                        let d = args[0];
                                        let val = dm.get(addr + (d as u16));
                                        Cpu::set_register_value(dst, val);
                                    }
                                    _ => {
                                        let reg = match cpu.reg16(&addr_reg) {
                                            RegisterClass::Pair(hl) => hl,
                                            _ => unreachable!("register must be Pair"),
                                        };
                                        let addr = Cpu::get_register_value(&reg);
                                        let val = dm.get(addr);
                                        Cpu::set_register_value(dst, val);
                                    }
                                }
                            }
                            Operand::Immediate16 => {
                                let addr = read16(&args);
                                let val = dm.get(addr);
                                Cpu::set_register_value(dst, val);
                            }
                            // Immediate16
                            _ => unreachable!("Only Register16 or Immedeate16 is possible"),
                        }
                        let val = args[0];
                        match cpu.reg8(&dst_reg) {
                            RegisterClass::Single(dst) => {
                                Cpu::set_register_value(dst, val);
                            }
                            _ => unreachable!(),
                        };
                    }
                    (&Operand::Mem(ref addr), &Operand::Reg(ref src_reg)) => {
                        let addr = match **addr {
                            Operand::Reg(ref reg) => {
                                let reg = match cpu.reg8(reg) {
                                    RegisterClass::Pair(r) => r,
                                    _ => unreachable!("register must be pair(HL)"),
                                };
                                Cpu::get_register_value(&reg)
                            }
                            Operand::Reg16(ref reg) => {
                                match reg {
                                    &Register16::IX | &Register16::IY => {
                                        let reg = match cpu.reg16(reg) {
                                            RegisterClass::Double(r) => r,
                                            _ => unreachable!("register IX or IY must be 16bit"),
                                        };
                                        let addr = Cpu::get_register_value(&reg);
                                        let d = args[0] as u16;
                                        addr + d
                                    }
                                    _ => {
                                        match cpu.reg16(reg) {
                                            RegisterClass::Double(reg) => {
                                                Cpu::get_register_value(&reg)
                                            }
                                            RegisterClass::Pair(reg) => {
                                                Cpu::get_register_value(&reg)
                                            }
                                            _ => unreachable!("register must be 16bit"),
                                        }
                                    }
                                }
                            }
                            Operand::Immediate16 => read16(args),
                            _ => {
                                unreachable!("memory address must be preserved in register or \
                                              16bit immediate")
                            }
                        };
                        let src = match cpu.reg8(src_reg) {
                            RegisterClass::Single(r) => r,
                            _ => unreachable!("src register must be 8bit"),
                        };
                        let val = Cpu::get_register_value(&src);
                        dm.set(addr, val);
                    }
                    (&Operand::Reg16(ref dst_reg), &Operand::Immediate16) => {
                        let val = read16(&args);
                        match cpu.reg16(&dst_reg) {
                            RegisterClass::Double(dst) => {
                                Cpu::set_register_value(dst, val);
                            }
                            RegisterClass::Pair(dst) => {
                                Cpu::set_register_value(dst, val);
                            }
                            _ => unreachable!(),
                        };
                    }
                    // &Operand::Mem(ref addr) => {}
                    _ => unreachable!(),
                }
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
