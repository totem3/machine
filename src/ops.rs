pub trait ByteContainer {
    type W;
    fn get(&self) -> Self::W;
    fn set(&mut self, v: Self::W);
}

pub trait Addressable<W> {
    fn get(&self, addr: u16) -> W;
    fn set(&mut self, addr: u16, W);
}
