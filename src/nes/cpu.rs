
use nes::memory::Address;

pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const CARRY_FLAG: u8 = 0b0000_0001;

#[derive(Eq, Debug)]
pub struct CPU {
    pub program_counter: Address,
    //    stack_pointers: u8,
    pub accumulator: u8,
    //    register_x: u8,
    //    register_y: u8,
    pub processor_status: u8,
}

impl PartialEq for CPU {
    fn eq(&self, other: &CPU) -> bool {
        self.program_counter == other.program_counter &&
            self.accumulator == other.accumulator &&
            self.processor_status == other.processor_status
    }
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            program_counter: 0x8000,
            //        stack_pointer: 0xFF,
            accumulator: 0,
            //        register_x: 0,
            //        register_y: 0,
            processor_status: 0
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.processor_status |= flags;
    }

    pub fn clear_flags(&mut self, flags: u8) {
        self.processor_status &= !flags;
    }

    pub fn is_flag_set(&self, flags: u8) -> bool {
        self.processor_status & flags > 0
    }

    pub fn add_accumulator(&mut self, value: u8) {
        self.accumulator += value;
    }
}