use register::Register;
use register::ByteContainer;
use register::RegisterClass;
use register::RegisterPair;
use register::RegisterRef;
use std::rc::Rc;
use std::cell::RefCell;

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
                RegisterClass::Pair(pair)
            }
            Register8::A => RegisterClass::Single(self.a.clone()),
        }
    }

    pub fn reg16(&self, which: Register16) -> RegisterClass {
        match which {
            Register16::BC => {
                let pair = RegisterPair::new(self.b.clone(), self.c.clone());
                RegisterClass::Pair(pair)
            }
            Register16::DE => {
                let pair = RegisterPair::new(self.d.clone(), self.e.clone());
                RegisterClass::Pair(pair)
            }
            Register16::HL => {
                let pair = RegisterPair::new(self.h.clone(), self.l.clone());
                RegisterClass::Pair(pair)
            }

            Register16::SP => RegisterClass::Double(self.sp.clone()),
            Register16::AF => {
                let pair = RegisterPair::new(self.a.clone(), self.f.clone());
                RegisterClass::Pair(pair)
            }

        }
    }


    pub fn set_pc(&mut self, counter: u16) {
        let mut pc = self.pc.borrow_mut();
        pc.set(counter);
    }

    pub fn incr(&mut self, bytes: usize) {
        let cur: u16 = self.pc.borrow().get();
        let mut pc = self.pc.borrow_mut();
        pc.set(cur + (bytes as u16));
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
        assert_eq!(b.borrow().get(), 0);
        {
            let mut bm = b.borrow_mut();
            bm.set(100);
        }
        assert_eq!(b.borrow().get(), 100);
    }

    #[test]
    fn test_16bit_register() {
        let cpu = Cpu::new();
        let mut b = match cpu.reg16(Register16::BC) {
            RegisterClass::Pair(r) => r,
            _ => unreachable!(),
        };
        assert_eq!(b.get(), 0);
        b.set(410);
        assert_eq!(b.get(), 410);
    }
}
