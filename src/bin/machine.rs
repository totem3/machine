#![feature(rustc_private)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate vm;

use vm::cpu::Cpu;
use vm::memory::Memory;
use vm::instruction::*;

use std::collections::HashMap;

struct Machine {
    cpu: Cpu,
    im: Memory,
    dm: Memory,
    state: State,
    op: Vec<u8>,
    inst: Instruction,
    table: HashMap<u8, HashMap<u8, HashMap<u8, Instruction>>>,
}

#[derive(Debug)]
enum State {
    Fetch,
    Decode,
    Exec,
    Writeback,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl Machine {
    pub fn init_table() -> HashMap<u8, HashMap<u8, HashMap<u8, Instruction>>> {
        let table = map!{
            0 => map!{
                0 => map!{
                    0 => nop()
                },
                1 => map!{
                    0 => ld16(Operand::Reg16(REG_BC)),
                    2 => ld16(Operand::Reg16(REG_DE)),
                    4 => ld16(Operand::Reg16(REG_HL)),
                    6 => ld16(Operand::Reg16(REG_SP)),
                    1 => add16(Operand::Reg16(REG_HL), Operand::Reg16(REG_BC)),
                    3 => add16(Operand::Reg16(REG_HL), Operand::Reg16(REG_DE)),
                    5 => add16(Operand::Reg16(REG_HL), Operand::Reg16(REG_HL)),
                    7 => add16(Operand::Reg16(REG_HL), Operand::Reg16(REG_SP))
                },
                2 => map!{
                    0 => ld_a2m(Operand::Reg16(REG_BC))
                    2 => ld_a2m(Operand::Reg16(REG_DE))
                    4 => ld_r2mi(Operand::Reg16(REG_HL))
                    6 => ld_r2mi(Operand::Reg(REG_A))
                },
                6 => map!{
                    0 => ld8(Operand::Reg(REG_B)),
                    1 => ld8(Operand::Reg(REG_C)),
                    2 => ld8(Operand::Reg(REG_D)),
                    3 => ld8(Operand::Reg(REG_E)),
                    4 => ld8(Operand::Reg(REG_H)),
                    5 => ld8(Operand::Reg(REG_L)),
                    6 => ld8(Operand::Reg(REG_HLM)),
                    7 => ld8(Operand::Reg(REG_A))
                }
            }
        };
        table
    }

    pub fn new(cpu: Cpu,
               im: Memory,
               dm: Memory,
               table: HashMap<u8, HashMap<u8, HashMap<u8, Instruction>>>)
               -> Self {
        Machine {
            cpu: cpu,
            im: im,
            dm: dm,
            state: State::Fetch,
            op: vec![],
            inst: nop(),
            table: table,
        }
    }

    pub fn tick(&mut self) {
        match self.state {
            State::Fetch => self.fetch(),
            State::Decode => self.decode(),
            State::Exec => self.exec(),
            State::Writeback => self.writeback(),
        }
        self.next();
    }

    fn load(&mut self, bytes: usize) -> Vec<u8> {
        let ip = self.cpu.pc.expect("pc is empty");
        let res = self.im.load(ip, bytes);
        self.cpu.incr(bytes);
        res
    }

    fn load_one(&mut self) -> Vec<u8> {
        self.load(1)
    }

    fn load_two(&mut self) -> Vec<u8> {
        self.load(2)
    }

    fn load_three(&mut self) -> Vec<u8> {
        self.load(3)
    }

    pub fn next(&mut self) {
        self.state = match self.state {
            State::Fetch => State::Decode,
            State::Decode => State::Exec,
            State::Exec => State::Writeback,
            State::Writeback => State::Fetch,
        }
    }

    pub fn fetch(&mut self) {
        info!("fetch");
        self.op = self.load_one();
        debug!("inst: {:?}", self.op);
    }

    pub fn decode(&mut self) {
        info!("decode");
        if self.op.len() < 1 {
            panic!("instruction is empty");
        }
        let inst = self.op[0];
        let x = (inst >> 6) & 0x03;
        let y = (inst >> 3) & 0x07;
        let z = inst & 0x07;
        println!("x: {}, y: {}, z: {}", x, y, z);
        // let inst = self.lookup(x, y, z);
        match self.table.get(&x).and_then(|t| t.get(&z)).and_then(|t| t.get(&y)) {
            Some(i) => self.inst = i.clone(),
            None => {}
        }
        debug!("inst: {:?}", self.inst);
    }

    fn lookup(&self, x: u8, y: u8, z: u8) -> Option<&Instruction> {
        self.table.get(&x).and_then(|t| t.get(&z)).and_then(|t| t.get(&y))
    }

    pub fn exec(&mut self) {
        info!("exec");
        let args: Vec<u8> = match self.inst.kind {
            InstKind::NoOperand => vec![],
            InstKind::N => self.load_one(),
            InstKind::NN => self.load_two(),
            InstKind::Displacement => self.load_one(),
        };
        self.inst.exec(&mut self.cpu, args);
    }

    pub fn writeback(&mut self) {
        info!("writeback");
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
            if self.cpu.pc.unwrap() == 10 || self.cpu.pc.unwrap() == ::std::u16::MAX {
                self.dump_cpu();
                break;
            }
        }
    }

    pub fn dump_cpu(&self) {
        println!("{:#?}", self.cpu);
    }
}

fn main() {
    env_logger::init().unwrap();

    let mut cpu = Cpu::new();
    cpu.set_pc(0);
    let mut im = Memory::new();
    im.store(0, &[0b00000001, 0b11000101, 0b00000111]);
    im.store(3, &[0b00010001, 0b11000101, 0b00000111]);
    println!("Instruction Memory: {:?}", im);
    let dm = Memory::new();
    let table = Machine::init_table();
    println!("table {:#?}", table);
    let mut machine = Machine::new(cpu, im, dm, table);

    machine.run()
}
