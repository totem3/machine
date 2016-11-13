use register::Register;
use register::RegisterClass;
use register::RegisterPair;
use register::RegisterRef;
use ops::ByteContainer;
use ops::Addressable;
use std::rc::Rc;
use std::cell::RefCell;

use memory::Memory;

#[derive(Debug)]
pub struct Cpu {
    pub a: RegisterRef<u8>,
    pub b: RegisterRef<u8>,
    pub c: RegisterRef<u8>,
    pub d: RegisterRef<u8>,
    pub e: RegisterRef<u8>,
    pub f: RegisterRef<u8>,
    pub h: RegisterRef<u8>,
    pub l: RegisterRef<u8>,
    pub pc: RegisterRef<u16>,
    pub sp: RegisterRef<u16>,
    pub ix: RegisterRef<u16>,
    pub iy: RegisterRef<u16>,
    pub r: RegisterRef<u8>,
    pub i: RegisterRef<u8>,
}

#[derive(Debug,PartialEq)]
pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

#[derive(Debug,PartialEq)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
    AF,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: Rc::new(RefCell::new(Register::default())),
            b: Rc::new(RefCell::new(Register::default())),
            c: Rc::new(RefCell::new(Register::default())),
            d: Rc::new(RefCell::new(Register::default())),
            e: Rc::new(RefCell::new(Register::default())),
            f: Rc::new(RefCell::new(Register::default())),
            h: Rc::new(RefCell::new(Register::default())),
            l: Rc::new(RefCell::new(Register::default())),
            pc: Rc::new(RefCell::new(Register::default())),
            sp: Rc::new(RefCell::new(Register::default())),
            ix: Rc::new(RefCell::new(Register::default())),
            iy: Rc::new(RefCell::new(Register::default())),
            r: Rc::new(RefCell::new(Register::default())),
            i: Rc::new(RefCell::new(Register::default())),
        }
    }

    pub fn reg8(&self, which: Register8) -> RegisterClass {
        match which {
            Register8::B => RegisterClass::Single(self.b.clone()),
            Register8::C => RegisterClass::Single(self.c.clone()),
            Register8::D => RegisterClass::Single(self.d.clone()),
            Register8::E => RegisterClass::Single(self.e.clone()),
            Register8::H => RegisterClass::Single(self.h.clone()),
            Register8::L => RegisterClass::Single(self.l.clone()),
            Register8::HL => {
                let pair = RegisterPair::new(self.h.clone(), self.l.clone());
                RegisterClass::Pair(Rc::new(RefCell::new(pair)))
            }
            Register8::A => RegisterClass::Single(self.a.clone()),
        }
    }

    pub fn reg16(&self, which: Register16) -> RegisterClass {
        match which {
            Register16::BC => {
                let pair = RegisterPair::new(self.b.clone(), self.c.clone());
                RegisterClass::Pair(Rc::new(RefCell::new(pair)))
            }
            Register16::DE => {
                let pair = RegisterPair::new(self.d.clone(), self.e.clone());
                RegisterClass::Pair(Rc::new(RefCell::new(pair)))
            }
            Register16::HL => {
                let pair = RegisterPair::new(self.h.clone(), self.l.clone());
                RegisterClass::Pair(Rc::new(RefCell::new(pair)))
            }

            Register16::SP => RegisterClass::Double(self.sp.clone()),
            Register16::AF => {
                let pair = RegisterPair::new(self.a.clone(), self.f.clone());
                RegisterClass::Pair(Rc::new(RefCell::new(pair)))
            }

        }
    }

    pub fn get_register_value<B>(reg: &Rc<RefCell<B>>) -> B::W
        where B: ByteContainer
    {
        reg.borrow().get()
    }

    pub fn set_register_value<B>(reg: Rc<RefCell<B>>, v: B::W)
        where B: ByteContainer
    {
        reg.borrow_mut().set(v)
    }

    pub fn set_pc(&mut self, counter: u16) {
        let mut pc = self.pc.borrow_mut();
        pc.set(counter);
    }

    pub fn increment_pc(&mut self, bytes: usize) {
        let cur: u16 = self.pc.borrow().get();
        let mut pc = self.pc.borrow_mut();
        pc.set(cur + (bytes as u16));
    }

    pub fn load(&mut self, memory: &Memory, src: u16, dst: RegisterClass) {
        match dst {
            RegisterClass::Single(reg) => {
                let value = memory.get(src);
                Cpu::set_register_value(reg, value);
            }
            RegisterClass::Double(reg) => {
                let value = memory.get(src);
                Cpu::set_register_value(reg, value);
            }
            RegisterClass::Pair(reg) => {
                let value = memory.get(src);
                Cpu::set_register_value(reg, value);
            }
        }
    }

    pub fn store(&self, memory: &mut Memory, src: RegisterClass, dst: u16) {
        match src {
            RegisterClass::Single(reg) => {
                let value = Cpu::get_register_value(&reg);
                memory.set(dst, value);
            }
            RegisterClass::Double(reg) => {
                let value = Cpu::get_register_value(&reg);
                memory.set(dst, value);
            }
            RegisterClass::Pair(reg) => {
                let value = Cpu::get_register_value(&reg);
                memory.set(dst, value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;
    use super::{Register8, Register16};
    use super::super::register::RegisterClass;
    use super::super::register::ByteContainer;

    #[test]
    fn test_8bit_register() {
        let cpu = Cpu::new();
        let b = match cpu.reg8(Register8::B) {
            RegisterClass::Single(r) => r,
            _ => unreachable!(),
        };
        assert_eq!(Cpu::get_register_value(&b), 0);
        Cpu::set_register_value(b.clone(), 100);
        assert_eq!(Cpu::get_register_value(&b), 100);
    }

    #[test]
    fn test_16bit_register() {
        let cpu = Cpu::new();
        let mut b = match cpu.reg16(Register16::SP) {
            RegisterClass::Double(r) => r,
            _ => unreachable!(),
        };
        assert_eq!(Cpu::get_register_value(&b), 0);
        Cpu::set_register_value(b.clone(), 65535);
        assert_eq!(Cpu::get_register_value(&b), 65535);
    }


    #[test]
    fn test_pair_register() {
        let cpu = Cpu::new();
        let mut b = match cpu.reg16(Register16::BC) {
            RegisterClass::Pair(r) => r,
            _ => unreachable!(),
        };
        assert_eq!(Cpu::get_register_value(&b), 0);
        Cpu::set_register_value(b.clone(), 410);
        assert_eq!(Cpu::get_register_value(&b), 410);
    }

}
