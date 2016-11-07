use register::{Register, Get};

#[derive(Debug)]
pub struct Cpu {
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: Register,
    pub h: Register,
    pub l: Register,
    pub pc: Register,
    pub sp: Register,
    pub ix: Register,
    pub iy: Register,
    pub r: Register,
    pub i: Register,
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
            a: Register::default(),
            b: Register::default(),
            c: Register::default(),
            d: Register::default(),
            e: Register::default(),
            f: Register::default(),
            h: Register::default(),
            l: Register::default(),
            pc: Register::default(),
            sp: Register::default(),
            ix: Register::default(),
            iy: Register::default(),
            r: Register::default(),
            i: Register::default(),
        }
    }

    pub fn reg8(&self, which: Register8) -> Vec<&Register> {
        match which {
            Register8::B => vec![&self.b],
            Register8::C => vec![&self.c],
            Register8::D => vec![&self.d],
            Register8::E => vec![&self.e],
            Register8::H => vec![&self.h],
            Register8::L => vec![&self.l],
            Register8::HL => vec![&self.h, &self.l],
            Register8::A => vec![&self.a],
        }
    }

    pub fn reg8_mut(&mut self, which: Register8) -> Vec<&mut Register> {
        match which {
            Register8::B => vec![&mut self.b],
            Register8::C => vec![&mut self.c],
            Register8::D => vec![&mut self.d],
            Register8::E => vec![&mut self.e],
            Register8::H => vec![&mut self.h],
            Register8::L => vec![&mut self.l],
            Register8::HL => vec![&mut self.h, &mut self.l],
            Register8::A => vec![&mut self.a],
        }
    }

    pub fn reg16(&self, which: Register16) -> Vec<&Register> {
        match which {
            Register16::BC => vec![&self.b, &self.c],
            Register16::DE => vec![&self.d, &self.e],
            Register16::HL => vec![&self.h, &self.l],
            Register16::SP => vec![&self.sp],
            Register16::AF => vec![&self.a, &self.f],
        }
    }


    pub fn set_pc(&mut self, counter: u16) {
        self.pc.set16(counter);
    }

    pub fn incr(&mut self, bytes: usize) {
        let cur: u16 = self.pc.get();
        self.pc.set16(cur + (bytes as u16));
    }

    pub fn set16(&mut self, n: u8, val: u16) {
        // 0 BC
        // 1 DE
        // 2 HL
        // 3 SP
        match n {
            0 => {
                self.b.set16(val);
                self.c.set16(val);
            }
            1 => {
                self.d.set16(val);
                self.e.set16(val);
            }
            2 => {
                self.h.set16(val);
                self.l.set16(val);
            }
            3 => {
                self.sp.set16(val);
            }
            _ => unreachable!(),
        }
    }
}
