
use memory::Address;

pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const CARRY_FLAG: u8 = 0b0000_0001;

#[derive(Eq, Debug, Clone, Copy)]
pub struct CPU {
    program_counter: Address,
    //    stack_pointers: u8,
    accumulator: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
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
            register_x: 0,
            register_y: 0,
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

    pub fn and_accumulator(&mut self, value: u8) {
        self.accumulator &= value;
    }

    pub fn program_counter(&self) -> Address {
        self.program_counter
    }

    pub fn register_x(&self) -> u8 {
        self.register_x
    }

    pub fn register_y(&self) -> u8 {
        self.register_y
    }

    pub fn get_and_increment_pc(&mut self) -> Address {
        let old_value = self.program_counter;
        self.program_counter += 1;
        old_value
    }

    pub fn add_program_counter(&mut self, value: u16) {
        self.program_counter = self.program_counter.wrapping_add(value);
    }

}

pub struct CpuBuilder {
    cpu: CPU,
}

impl CpuBuilder {
    pub fn new() -> CpuBuilder {
        CpuBuilder {
            cpu: CPU::new(),
        }
    }

    pub fn program_counter(&mut self, value: Address) -> &mut CpuBuilder {
        self.cpu.program_counter = value;
        return self;
    }

    pub fn accumulator(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.accumulator = value;
        return self;
    }

    pub fn register_x(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.register_x = value;
        return self;
    }

    pub fn register_y(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.register_y = value;
        return self;
    }

    pub fn build(&self) -> CPU {
        return self.cpu;
    }
}