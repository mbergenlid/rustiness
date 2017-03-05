
pub type Address = u16;

pub trait Memory {
    fn get(&self, address: Address) -> u8;
    fn set(&mut self, address: Address, value: u8);

    fn set_slice(&mut self, start: Address, data: &[u8]) {
        let mut address = start;
        for &d in data {
            self.set(address, d);
            address = address.wrapping_add(1);
        }
    }
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


use std::collections::HashMap;


use std::ops::{Range, Index};
impl Index<Range<usize>> for BasicMemory {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.data[index]
    }
}

pub trait MemoryMappedIO {
    fn read(&self, &BasicMemory) -> u8;
    fn write(&mut self, &mut BasicMemory, value: u8);
}

use borrow::MutableRef;
pub struct CPUMemory<'a> {
    memory: Box<BasicMemory>,
    io_registers: HashMap<u16, MutableRef<'a, MemoryMappedIO>>,
}

impl <'a> CPUMemory<'a> {
    pub fn new(memory: Box<BasicMemory>, io_registers: HashMap<u16, MutableRef<'a, MemoryMappedIO>>) -> CPUMemory<'a> {
        CPUMemory {
            memory: memory,
            io_registers: io_registers,
        }
    }


}
use std::fmt::{Error, Debug, Formatter};
pub struct CPUMemoryReference<'a, 'b>(pub u16, pub &'a CPUMemory<'b>) where 'b: 'a;

impl<'a, 'b> Debug for CPUMemoryReference<'a, 'b> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_fmt(
            format_args!("0x{:04x} -> 0x{:x}", self.0, self.1.memory.get(self.0))
        )
    }
}

use std::borrow::BorrowMut;
impl <'a> Memory for CPUMemory<'a> {
    fn get(&self, address: Address) -> u8 {
        self.io_registers.get(&address)
            .map(|io| io.read(self.memory.as_ref()))
            .unwrap_or_else(|| self.memory.get(address))
    }

    fn set(&mut self, address: Address, value: u8) {
        if let Some(io) = self.io_registers.get_mut(&address) {
            io.write(self.memory.borrow_mut(), value);
        } else {
            self.memory.set(address, value);
        }
    }
}

macro_rules! cpu_memory {
    ( $memory:expr, $( $x:expr => $y:expr ),* ) => {
        {
            use std::collections::HashMap;
            use $crate::memory::MemoryMappedIO;
            use $crate::borrow::MutableRef;
            let mut map: HashMap<u16, MutableRef<MemoryMappedIO>> = HashMap::new();

            $(
                map.insert($x, $y);
            )*
            let cpu_memory = $crate::memory::CPUMemory::new(
                $memory, map
            );
            cpu_memory
        }
    };
}

macro_rules! memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use memory;
            let mut temp_memory = memory::BasicMemory::new();
            $(
                temp_memory.set($x, $y);
            )*
            temp_memory
        }
    };
}

#[macro_export]
macro_rules! external_memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use nes::memory::Memory;
            let mut temp_memory = nes::memory::BasicMemory::new();
            $(
                temp_memory.set($x, $y);
            )*
            temp_memory
        }
    };
}

#[cfg(test)]
mod test {
    use super::{Memory, BasicMemory};
    use std::cell::{RefCell,Cell};
    use std::rc::Rc;

    struct TestRegister {
        reads: Cell<u32>,
        writes: Vec<u8>,
    }

    use super::MemoryMappedIO;
    impl MemoryMappedIO for Rc<RefCell<TestRegister>> {
        fn read(&self, _: &BasicMemory) -> u8 {
            let prev_value = self.borrow().reads.get();
            self.borrow().reads.set(prev_value + 1);
            return 0;
        }

        fn write(&mut self, _: &mut BasicMemory, value: u8) {
            self.borrow_mut().writes.push(value);
        }
    }

    #[test]
    fn test_io_registers() {
        let register1  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register2  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        {
            let mut io_registers = cpu_memory!(
                box BasicMemory::new(),
                0x2000 => MutableRef::Box(Box::new(register1.clone())),
                0x4016 => MutableRef::Box(Box::new(register2.clone()))
            );

            io_registers.get(0x2000);
            io_registers.get(0x4016);
            io_registers.get(0x4016);

            io_registers.set(0x2000, 4);
            io_registers.set(0x2000, 5);
            io_registers.set(0x4016, 6);

            io_registers.set(0x2001, 17);
            assert_eq!(17, io_registers.get(0x2001));
        }

        assert_eq!(1, register1.borrow().reads.get());
        assert_eq!(2, register2.borrow().reads.get());
        assert_eq!(vec!(4,5), register1.borrow().writes);
        assert_eq!(vec!(6), register2.borrow().writes);

    }
}
