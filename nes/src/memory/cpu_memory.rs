use memory::{BasicMemory, Memory, Address, MemoryMappedIO};
use borrow::MutableRef;
pub struct CPUMemory<'a> {
    memory: Box<BasicMemory>,
    io_registers: Vec<(u16, MutableRef<'a, MemoryMappedIO>)>,
}

impl <'a> CPUMemory<'a> {
    pub fn new(memory: Box<BasicMemory>, io_registers: Vec<(u16, MutableRef<'a, MemoryMappedIO>)>) -> CPUMemory<'a> {
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
        if address < 0x2000 {
            self.memory.get(address)
        } else {
            self.io_registers.iter()
                .find(|e| e.0 == address)
                .map(|e| e.1.read(self.memory.as_ref()))
                .unwrap_or_else(|| self.memory.get(address))
        }
    }

    fn set(&mut self, address: Address, value: u8) {
        if address < 0x2000 {
            self.memory.set(address, value);
        } else {
            if let Some(mut entry) = self.io_registers.iter_mut().find(|e| e.0 == address) {
                entry.1.write(self.memory.borrow_mut(), value);
            } else {
                self.memory.set(address, value);
            }
        }
    }
}

#[macro_export]
macro_rules! cpu_memory {
    ( $memory:expr, $( $x:expr => $y:expr ),* ) => {
        {
            use $crate::borrow::MutableRef;
            let cpu_memory = $crate::memory::CPUMemory::new(
                $memory, vec![ $(($x, $y), )* ]
            );
            cpu_memory
        }
    };
}


#[cfg(test)]
mod test {
    use memory::{Memory, BasicMemory};
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
