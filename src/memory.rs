use std::fmt;
use ops::Addressable;

const MEMORY_SIZE: usize = 131072;
pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "Memory([");
        let mut nf = false;
        for v in self.data.iter().take(10) {
            if nf {
                let _ = write!(f, ", ");
            } else {
                nf = true;
            }
            let _ = write!(f, "{}", v);
        }
        write!(f, "])")
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; MEMORY_SIZE] }
    }
}

impl Addressable<u8> for Memory {
    fn get(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        self.data[addr]
    }

    fn set(&mut self, addr: u16, v: u8) {
        let addr = addr as usize;
        self.data[addr] = v;
    }
}

impl Addressable<u16> for Memory {
    fn get(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        (self.data[addr] as u16) << 8 | self.data[addr + 1] as u16
    }

    fn set(&mut self, addr: u16, v: u16) {
        let addr = addr as usize;
        self.data[addr] = (v >> 8) as u8;
        self.data[addr + 1] = v as u8;
    }
}

#[cfg(test)]
mod test {
    use super::Memory;
    use super::super::ops::Addressable;
    #[test]
    fn test_get_set8() {
        let mut m = Memory::new();
        let r: u8 = m.get(20);
        assert_eq!(r, 0u8);
        m.set(20, 120u8);
        let r: u8 = m.get(20);
        assert_eq!(r, 120u8);
    }

    #[test]
    fn test_get_set16() {
        let mut m = Memory::new();
        let addr: u16 = 40;
        let r: u16 = m.get(addr);
        assert_eq!(r, 0);
        m.set(addr, 320u16);
        let r: u16 = m.get(addr);
        assert_eq!(r, 320)
    }
}
