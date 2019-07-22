pub type Address = u16;
use std::iter::Iterator;
use std::ops::{Index, Range};

use Cycles;

pub trait Memory {
    fn get(&self, address: Address, sub_cycle: Cycles) -> u8;
    fn set(&mut self, address: Address, value: u8, sub_cycle: Cycles);

    fn dma(&self, range: Range<Address>, destination: &mut [u8]) {
        for (i, address) in range.enumerate() {
            destination[i] = self.get(address, 0);
        }
    }

    fn set_slice(&mut self, start: Address, data: &[u8]) {
        let mut address = start;
        for &d in data {
            self.set(address, d, 0);
            address = address.wrapping_add(1);
        }
    }
}

pub struct BasicMemory {
    data: Vec<u8>,
}

impl BasicMemory {
    pub fn new() -> BasicMemory {
        return BasicMemory {
            data: vec![0; 65_536],
        };
    }
}

impl Memory for BasicMemory {
    fn get(&self, address: Address, _: Cycles) -> u8 {
        return self.data[address as usize];
    }

    fn set(&mut self, address: Address, value: u8, _: Cycles) {
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
    fn read(&self, &dyn Memory) -> u8;
    fn write(&mut self, &mut dyn Memory, value: u8);

    fn read_at_cycle(&self, memory: &dyn Memory, _: Cycles) -> u8 {
        self.read(memory)
    }

    fn write_at_cycle(&mut self, memory: &mut dyn Memory, value: u8, _: Cycles) {
        self.write(memory, value)
    }
}

#[macro_export]
macro_rules! memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use $crate::memory::{Memory, BasicMemory};
            let mut temp_memory = BasicMemory::new();
            $(
                temp_memory.set($x, $y, 0);
            )*
            temp_memory
        }
    };
}
