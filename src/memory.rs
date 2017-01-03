pub type Address = u16;

pub trait Memory {
    fn get(&self, address: Address) -> u8;
    fn set(&mut self, address: Address, value: u8);
}

pub struct BasicMemory {
    data: [u8; 65_536]
}

impl BasicMemory {
    pub fn new() -> BasicMemory {
        return BasicMemory { data: [0; 65_536]};
    }
}

impl Memory for BasicMemory {
    fn get(&self, address: Address) -> u8 {
        return self.data[address as usize];
    }

    fn set(&mut self, address: Address, value: u8) {
        self.data[address as usize] = value;
    }
}