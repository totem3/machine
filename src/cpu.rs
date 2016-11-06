use register::Register;

// D, S
// 111 A
// 000 B
// 001 C
// 010 D
// 011 E
// 100 H
// 101 L

// RP
// 00 BC
// 01 DE
// 10 HL
// 11 SP

#[derive(Debug)]
pub struct Cpu {
    pub a: Register<u8>,
    pub b: Register<u8>,
    pub c: Register<u8>,
    pub d: Register<u8>,
    pub e: Register<u8>,
    pub f: Register<u8>,
    pub h: Register<u8>,
    pub l: Register<u8>,
    pub pc: Register<u16>,
    pub sp: Register<u16>,
    pub ix: Register<u16>,
    pub iy: Register<u8>,
    pub r: Register<u8>,
    pub i: Register<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: None,
            b: None,
            c: None,
            d: None,
            e: None,
            f: None,
            h: None,
            l: None,
            pc: None,
            sp: None,
            ix: None,
            iy: None,
            r: None,
            i: None,
        }
    }

    pub fn set_pc(&mut self, counter: u16) {
        self.pc = Some(counter);
    }

    pub fn incr(&mut self, bytes: usize) {
        self.pc = self.pc.map(|v| {
            println!("v = {}", v);
            v + (bytes as u16)
        });
    }

    pub fn set16(&mut self, n: u8, val: u16) {
        // 0 BC
        // 1 DE
        // 2 HL
        // 3 SP
        match n {
            0 => {
                self.b = Some((val & 0xff) as u8);
                self.c = Some((val >> 8) as u8);
            }
            1 => {
                self.d = Some((val & 0xff) as u8);
                self.e = Some((val >> 8) as u8);
            }
            2 => {
                self.h = Some((val & 0xff) as u8);
                self.l = Some((val >> 8) as u8);
            }
            3 => {
                self.sp = Some(val);
            }
            _ => unreachable!(),
        }
    }
}
