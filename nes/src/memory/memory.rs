
pub type Address = u16;
use std::iter::Iterator;
use std::ops::{Range, Index};

pub trait Memory {
    fn get(&self, address: Address) -> u8;
    fn set(&mut self, address: Address, value: u8);

    fn dma(&self, range: Range<Address>, destination: &mut [u8]) {
        for (i, address) in range.enumerate() {
            destination[i] = self.get(address);
        }
    }

    fn set_slice(&mut self, start: Address, data: &[u8]) {
        let mut address = start;
        for &d in data {
            self.set(address, d);
            address = address.wrapping_add(1);
        }
    }
}

pub struct BasicMemory {
    data: Vec<u8>
}

impl BasicMemory {
    pub fn new() -> BasicMemory {
        return BasicMemory { data: vec![0; 65_536]};
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


impl Index<Range<usize>> for BasicMemory {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.data[index]
    }
}

pub trait MemoryMappedIO {
    fn read(&self, &Memory) -> u8;
    fn write(&mut self, &mut Memory, value: u8);
}


#[macro_export]
macro_rules! memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use $crate::memory::{Memory, BasicMemory};
            let mut temp_memory = BasicMemory::new();
            $(
                temp_memory.set($x, $y);
            )*
            temp_memory
        }
    };
}