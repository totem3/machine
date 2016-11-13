use std::rc::Rc;
use std::cell::RefCell;
use ops::ByteContainer;

pub enum RegisterClass {
    Single(RegisterRef<u8>),
    Double(RegisterRef<u16>),
    Pair(RegisterPairRef),
}

pub type RegisterRef<T> = Rc<RefCell<Register<T>>>;
pub type RegisterPairRef = Rc<RefCell<RegisterPair>>;

pub struct RegisterPair {
    high: RegisterRef<u8>,
    low: RegisterRef<u8>,
}

impl RegisterPair {
    pub fn new(high: RegisterRef<u8>, low: RegisterRef<u8>) -> Self {
        RegisterPair {
            high: high,
            low: low,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Register<T> {
    value: T,
}

impl<T: Default> Default for Register<T> {
    fn default() -> Self {
        Register { value: T::default() }
    }
}

impl<T> Register<T> {
    pub fn new(v: T) -> Self {
        Register { value: v }
    }
}

impl ByteContainer for Register<u8> {
    type W = u8;
    fn get(&self) -> Self::W {
        self.value
    }
    fn set(&mut self, v: Self::W) {
        self.value = v;
    }
}

impl ByteContainer for Register<u16> {
    type W = u16;
    fn get(&self) -> Self::W {
        self.value
    }
    fn set(&mut self, v: Self::W) {
        self.value = v;
    }
}


impl ByteContainer for RegisterPair {
    type W = u16;
    fn get(&self) -> Self::W {
        self.high.borrow().get() as u16 | (self.low.borrow().get() as u16) << 8
    }

    fn set(&mut self, v: Self::W) {
        let hb = (v & 0xff) as u8;
        let lb = ((v & 0xff00) >> 8) as u8;
        let mut high = self.high.borrow_mut();
        high.value = hb;
        let mut low = self.low.borrow_mut();
        low.value = lb;
    }
}

#[cfg(test)]
mod test {
    use super::super::ops::ByteContainer;
    use super::Register;
    use super::RegisterPair;
    use super::RegisterRef;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_register() {
        let mut r: Register<u8> = Register::default();
        assert_eq!(r.get(), 0);
        r.set(42);
        assert_eq!(r.get(), 42);

        let mut r16: Register<u16> = Register::new(1989);
        assert_eq!(r16.get(), 1989);
        r16.set(65535);
        assert_eq!(r16.get(), 65535);
    }

    #[test]
    fn test_register_pair() {
        let r1: RegisterRef<u8> = Rc::new(RefCell::new(Register::new(197)));
        let r2: RegisterRef<u8> = Rc::new(RefCell::new(Register::new(7)));
        let mut rp = RegisterPair::new(r1.clone(), r2.clone());
        assert_eq!(rp.get(), 1989);
        rp.set(15623);
        assert_eq!(rp.get(), 15623);

        assert_eq!(r1.borrow().get(), 0x07);
        assert_eq!(r2.borrow().get(), 0x3d);
    }
}
