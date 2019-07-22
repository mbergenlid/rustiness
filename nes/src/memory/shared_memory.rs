use memory::{Address, BasicMemory, Memory};

use std::cell::RefCell;
use std::rc::Rc;
use Cycles;

pub struct SharedMemory(Rc<RefCell<dyn Memory>>);

impl SharedMemory {
    pub fn new() -> SharedMemory {
        SharedMemory(Rc::new(RefCell::new(BasicMemory::new())))
    }

    pub fn wrap<T: Memory + 'static>(memory: T) -> SharedMemory {
        SharedMemory(Rc::new(RefCell::new(memory)))
    }
}

impl Memory for SharedMemory {
    fn get(&self, address: Address, sub_cycle: Cycles) -> u8 {
        self.0.borrow().get(address, sub_cycle)
    }

    fn set(&mut self, address: Address, value: u8, sub_cycles: Cycles) {
        self.0.borrow_mut().set(address, value, sub_cycles);
    }
}
