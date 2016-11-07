use std::cell::RefCell;
#[derive(Debug)]
pub struct Register {
    value: Vec<u8>,
}

#[derive(Debug)]
pub struct RegisterPair {
    high: RefCell<Register>,
    low: RefCell<Register>,
}

impl Default for Register {
    fn default() -> Self {
        Register { value: vec![] }
    }
}

pub trait Get<T> {
    fn get(&self) -> T;
}

impl Get<u8> for Register {
    fn get(&self) -> u8 {
        if self.value.is_empty() {
            0
        } else {
            self.value[0]
        }
    }
}

impl Get<u16> for Register {
    fn get(&self) -> u16 {
        if self.value.len() < 2 {
            0
        } else {
            ((self.value[1] as u16) << 8) | self.value[0] as u16
        }
    }
}


impl Register {
    pub fn set8(&mut self, v: u8) {
        self.value = vec![v];
    }

    pub fn set16(&mut self, v: u16) {
        let v1: u8 = (v & 0xff) as u8;
        let v2: u8 = ((v >> 8) as u8) & 0xff;
        self.value = vec![v1, v2];
    }
}

#[cfg(test)]
mod test {
    use super::Register;
    use super::Get;
    fn test_default() {
        let r = Register::default();
        let _u8: u8 = r.get();
        let _u16: u16 = r.get();
        assert_eq!(_u8, 0u8);
        assert_eq!(_u16, 0u16);
    }

    fn test_get() {
        let mut r = Register::default();
        r.set16(1989);
        let _u8: u8 = r.get();
        let _u16: u16 = r.get();
        assert_eq!(_u8, 197u8);
        assert_eq!(_u16, 1989u16);
    }

    fn test_get_set() {
        let mut r = Register::default();
        r.set16(2000u16);
        let _u16: u16 = r.get();
        assert_eq!(_u16, 2000u16);
    }
}
