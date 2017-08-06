use memory::{Memory, Address, MemoryMappedIO};
use borrow::MutableRef;

pub struct CPUMemory<'a> {
    memory: Box<Memory>,
    io_registers: Vec<(u16, MutableRef<'a, MemoryMappedIO>)>,
}

impl <'a> CPUMemory<'a> {
    pub fn new(memory: Box<Memory>, io_registers: Vec<(u16, MutableRef<'a, MemoryMappedIO>)>) -> CPUMemory<'a> {
        CPUMemory {
            memory: memory,
            io_registers: io_registers,
        }
    }

    fn translate(&self, address: Address) -> Address {
        if address >= 0x2008 && address < 0x4000 {
            0x2000 + (address & 0x7)
        } else {
            address
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
        let address = self.translate(address);
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
        let address = self.translate(address);
        if address < 0x2000 {
            self.memory.set(address, value);
        } else {
            if let Some(mut entry) = self.io_registers.iter_mut().find(|e| e.0 == address) {
                let memory: &mut Memory = self.memory.borrow_mut();
                entry.1.write(memory, value);
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
        fn read(&self, _: &Memory) -> u8 {
            let prev_value = self.borrow().reads.get();
            self.borrow().reads.set(prev_value + 1);
            return 0;
        }

        fn write(&mut self, _: &mut Memory, value: u8) {
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

    #[test]
    #[allow(non_snake_case)]
    fn ppu_registers_should_be_mirrored_at_2008_to_3FFF() {
        let register0  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register1  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register2  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register3  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register4  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register5  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register6  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));
        let register7  = Rc::new(RefCell::new(TestRegister { reads: Cell::new(0), writes: vec!() }));

        let mut memory = cpu_memory!(
            box BasicMemory::new(),
            0x2000 => MutableRef::Box(Box::new(register0.clone())),
            0x2001 => MutableRef::Box(Box::new(register1.clone())),
            0x2002 => MutableRef::Box(Box::new(register2.clone())),
            0x2003 => MutableRef::Box(Box::new(register3.clone())),
            0x2004 => MutableRef::Box(Box::new(register4.clone())),
            0x2005 => MutableRef::Box(Box::new(register5.clone())),
            0x2006 => MutableRef::Box(Box::new(register6.clone())),
            0x2007 => MutableRef::Box(Box::new(register7.clone()))
        );

        let mut address = 0x2008;
        while address < 0x4000 {
            memory.get(address);
            memory.get(address + 1);
            memory.get(address + 2);
            memory.get(address + 3);
            memory.get(address + 4);
            memory.get(address + 5);
            memory.get(address + 6);
            memory.get(address + 7);

            memory.set(address, 42);
            memory.set(address + 1, 42);
            memory.set(address + 2, 42);
            memory.set(address + 3, 42);
            memory.set(address + 4, 42);
            memory.set(address + 5, 42);
            memory.set(address + 6, 42);
            memory.set(address + 7, 42);
            address += 8;
        }

        assert_eq!(0x3FF, register0.borrow().reads.get());
        assert_eq!(0x3FF, register1.borrow().reads.get());
        assert_eq!(0x3FF, register2.borrow().reads.get());
        assert_eq!(0x3FF, register3.borrow().reads.get());
        assert_eq!(0x3FF, register4.borrow().reads.get());
        assert_eq!(0x3FF, register5.borrow().reads.get());
        assert_eq!(0x3FF, register6.borrow().reads.get());
        assert_eq!(0x3FF, register7.borrow().reads.get());

        let writes = vec!(42; 0x3FF);
        assert_eq!(&writes, &register0.borrow().writes);
        assert_eq!(&writes, &register1.borrow().writes);
        assert_eq!(&writes, &register2.borrow().writes);
        assert_eq!(&writes, &register3.borrow().writes);
        assert_eq!(&writes, &register4.borrow().writes);
        assert_eq!(&writes, &register5.borrow().writes);
        assert_eq!(&writes, &register6.borrow().writes);
        assert_eq!(&writes, &register7.borrow().writes);
    }
}
