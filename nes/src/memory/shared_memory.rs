use memory::{BasicMemory, Memory, Address};

use std::rc::Rc;
use std::cell::RefCell;

pub struct SharedMemory(Rc<RefCell<Memory>>);

impl SharedMemory {
    pub fn new() -> SharedMemory {
        SharedMemory(Rc::new(RefCell::new(BasicMemory::new())))
    }

    pub fn wrap<T: Memory + 'static>(memory: T) -> SharedMemory {
        SharedMemory(Rc::new(RefCell::new(memory)))
    }
}

impl Memory for SharedMemory {
    fn get(&self, address: Address) -> u8 {
        self.0.borrow().get(address)
    }

    fn set(&mut self, address: Address, value: u8) {
        self.0.borrow_mut().set(address, value);
    }
}
