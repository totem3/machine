use std::fmt;

const MEMORY_SIZE: usize = 131072;
pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory([");
        let mut nf = false;
        for v in self.data.iter().take(10) {
            if nf {
                write!(f, ", ");
            } else {
                nf = true;
            }
            write!(f, "{}", v);
        }
        write!(f, "])")
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; MEMORY_SIZE] }
    }

    pub fn store(&mut self, pointer: u16, values: &[u8]) {
        for (i, v) in values.iter().enumerate() {
            let index = (pointer as usize) + i;
            self.data[index] = *v;
        }
    }

    pub fn load(&self, pointer: u16, bytes: usize) -> Vec<u8> {
        let pointer: usize = pointer as usize;
        self.data[pointer..pointer + bytes].to_vec()
    }

    pub fn load_one(&self, pointer: u16) -> Vec<u8> {
        self.load(pointer, 1)
    }
}
