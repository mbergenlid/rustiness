
use memory::Address;

pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
pub const DECIMAL_FLAG: u8 = 0b0000_1000;
pub const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const CARRY_FLAG: u8 = 0b0000_0001;

trait NesInteger {
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool {
        !self.is_negative()
    }
}

impl NesInteger for u8 {
    fn is_negative(&self) -> bool {
        self & 0x80 > 0
    }
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct CPU {
    program_counter: Address,
    pub stack_pointer: u8,
    accumulator: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
}

impl PartialEq for CPU {
    fn eq(&self, other: &CPU) -> bool {
        self.program_counter == other.program_counter &&
            self.accumulator == other.accumulator &&
            self.processor_status == other.processor_status &&
            self.register_x == other.register_x &&
            self.register_y == other.register_y &&
            self.stack_pointer == other.stack_pointer
    }
}

use std::fmt::{Formatter, Error, Display};
impl Display for CPU {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("CPU:\t\t\t\t\t|\n").unwrap();
        formatter.write_fmt(
            format_args!("\tProgram counter:  0x{:4X}\t|\n", self.program_counter)
        ).unwrap();

        formatter.write_str("\tProcessor status: N O D I Z C\t|\n").unwrap();
        formatter.write_fmt(format_args!("\t                  {} {} {} {} {} {}\t|\n",
                 self.is_flag_set(NEGATIVE_FLAG) as u8,
                 self.is_flag_set(OVERFLOW_FLAG) as u8,
                 self.is_flag_set(DECIMAL_FLAG) as u8,
                 self.is_flag_set(INTERRUPT_DISABLE_FLAG) as u8,
                 self.is_flag_set(ZERO_FLAG) as u8,
                 self.is_flag_set(CARRY_FLAG) as u8,
        )).unwrap();
        formatter.write_fmt(
            format_args!("\tAccumulator:      0x{:02X}\t\t|\n", self.accumulator())).unwrap();
        formatter.write_fmt(
            format_args!("\tRegister X:       0x{:02X}\t\t|\n", self.register_x())).unwrap();
        formatter.write_fmt(
            format_args!("\tRegister Y:       0x{:02X}\t\t|\n", self.register_y()))
    }
}

impl CPU {
    pub fn new(start_address: u16) -> CPU {
        return CPU {
            program_counter: start_address,
            stack_pointer: 0xFF,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0x04,
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
        let sum = self.accumulator as u16 + value as u16 + (self.processor_status & CARRY_FLAG) as u16;

        if sum > 0xFF {
            self.set_flags(CARRY_FLAG);
        } else {
            self.clear_flags(CARRY_FLAG);
        }
        if sum as u8 == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }
        if value & 0x80 == 0 && self.accumulator & 0x80 == 0 && sum >= 0x80 ||
            value & self.accumulator >= 0x80 && (sum as u8) < 0x80 {
            self.set_flags(OVERFLOW_FLAG);
        } else {
            self.clear_flags(OVERFLOW_FLAG);
        }
        if sum & 0x80 == 0 {
            self.clear_flags(NEGATIVE_FLAG);
        } else {
            self.set_flags(NEGATIVE_FLAG);
        }
        self.accumulator = sum as u8;
    }

    pub fn sub_accumulator(&mut self, value: u8) {
        let borrow = 1 - (self.processor_status & CARRY_FLAG);
        self.set_flags(CARRY_FLAG);
        let accumulator = self.accumulator;
        let accumulator = self.subtract_without_carry(accumulator, borrow);

        let new_value = self.subtract_without_carry(accumulator, value);

        let actual_sum = ((self.accumulator as i8) as i16) - ((value as i8) as i16) - (borrow as i16);
        if actual_sum < -128 || actual_sum > 127 {
            self.set_flags(OVERFLOW_FLAG);
        } else {
            self.clear_flags(OVERFLOW_FLAG);
        }
        self.update_z_and_n_flags(new_value);
        self.accumulator = new_value;
    }

    fn subtract_without_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        if rhs > lhs {
            self.clear_flags(CARRY_FLAG);
            ((lhs as u16 + 0x100) - rhs as u16) as u8
        } else {
            lhs - rhs
        }
    }

    pub fn and_accumulator(&mut self, value: u8) {
        let new_value = self.accumulator & value;
        self.update_z_and_n_flags(new_value);
        self.accumulator = new_value;
    }

    pub fn or_accumulator(&mut self, value: u8) {
        let new_value = self.accumulator | value;
        self.update_z_and_n_flags(new_value);
        self.accumulator = new_value;
    }

    pub fn asl_accumulator(&mut self) {
        let acc = self.accumulator;
        self.accumulator = self.arithmetic_shift_left(acc);
    }

    pub fn arithmetic_shift_left(&mut self, value: u8) -> u8 {
        if value & 0x80 == 0 {
            self.clear_flags(CARRY_FLAG);
        } else {
            self.set_flags(CARRY_FLAG);
        }
        let new_value = value << 1;
        self.update_z_and_n_flags(new_value);
        return new_value;
    }

    pub fn logical_shift_right_accumulator(&mut self) {
        let acc = self.accumulator;
        self.accumulator = self.logical_shift_right(acc);
    }

    pub fn logical_shift_right(&mut self, value: u8) -> u8 {
        if value & 0x01 > 0 {
            self.set_flags(CARRY_FLAG);
        } else {
            self.clear_flags(CARRY_FLAG);
        }
        let new_value = value >> 1;
        self.update_z_and_n_flags(new_value);
        return new_value;
    }

    pub fn rotate_accumulator_left(&mut self) {
        let acc = self.accumulator;
        self.accumulator = self.rotate_left(acc);
    }

    pub fn rotate_left(&mut self, value: u8) -> u8 {
        let carry = self.processor_status & CARRY_FLAG;
        if value & 0x80 == 0 {
            self.clear_flags(CARRY_FLAG);
        } else {
            self.set_flags(CARRY_FLAG);
        }
        let new_value = (value << 1) | carry;
        self.update_z_and_n_flags(new_value);
        return new_value;
    }

    pub fn rotate_accumulator_right(&mut self) {
        let acc = self.accumulator;
        self.accumulator = self.rotate_right(acc);
    }

    pub fn rotate_right(&mut self, value: u8) -> u8 {
        let carry = (self.processor_status & CARRY_FLAG) << 7;
        if value & 0x01 == 0 {
            self.clear_flags(CARRY_FLAG);
        } else {
            self.set_flags(CARRY_FLAG);
        }
        let new_value = (value >> 1) | carry;
        self.update_z_and_n_flags(new_value);
        return new_value;
    }

    pub fn bit_test(&mut self, mask: u8) {
        let value = self.accumulator & mask;
        if value == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }

        self.processor_status = (mask & 0xC0) | (self.processor_status & 0x3F);
    }

    pub fn cmp_accumulator(&mut self, value: u8) {
        let left = self.accumulator;
        self.cmp(left, value);
    }

    pub fn cmp_register_x(&mut self, value: u8) {
        let left = self.register_x;
        self.cmp(left, value);
    }

    pub fn cmp_register_y(&mut self, value: u8) {
        let left = self.register_y;
        self.cmp(left, value);
    }

    fn cmp(&mut self, left: u8, right: u8) {
        if left == right {
            self.set_flags(ZERO_FLAG | CARRY_FLAG);
        } else if left > right {
            self.set_flags(CARRY_FLAG);
            self.clear_flags(ZERO_FLAG);
        } else {
            self.clear_flags(CARRY_FLAG | ZERO_FLAG);
        }
        let result = left.wrapping_sub(right);
        if result.is_negative() {
            self.set_flags(NEGATIVE_FLAG);
        } else {
            self.clear_flags(NEGATIVE_FLAG);
        }
    }

    pub fn decrement(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.update_z_and_n_flags(new_value);
        new_value
    }

    pub fn decrement_x(&mut self) {
        let x = self.register_x;
        self.register_x = self.decrement(x)
    }

    pub fn decrement_y(&mut self) {
        let y = self.register_y;
        self.register_y = self.decrement(y)
    }

    pub fn increment(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.update_z_and_n_flags(new_value);
        new_value
    }
    pub fn increment_x(&mut self) {
        let x = self.register_x;
        self.register_x = self.increment(x);
    }

    pub fn increment_y(&mut self) {
        let y = self.register_y;
        self.register_y = self.increment(y);
    }

    pub fn xor_accumulator(&mut self, value: u8) {
        let new_value = self.accumulator ^ value;
        self.update_z_and_n_flags(new_value);
        self.accumulator = new_value;
    }

    pub fn load_accumulator(&mut self, value: u8) {
        self.update_z_and_n_flags(value);
        self.accumulator = value;
    }

    pub fn load_x(&mut self, value: u8) {
        self.update_z_and_n_flags(value);
        self.register_x = value;
    }

    pub fn load_y(&mut self, value: u8) {
        self.update_z_and_n_flags(value);
        self.register_y = value;
    }


    fn update_z_and_n_flags(&mut self, value: u8) {
        if value == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }
        if value & 0x80 > 0 {
            self.set_flags(NEGATIVE_FLAG);
        } else {
            self.clear_flags(NEGATIVE_FLAG);
        }
    }

    pub fn program_counter(&self) -> Address {
        self.program_counter
    }

    pub fn accumulator(&self) -> u8 {
        self.accumulator
    }

    pub fn register_x(&self) -> u8 {
        self.register_x
    }

    pub fn register_y(&self) -> u8 {
        self.register_y
    }

    pub fn processor_status(&self) -> u8 {
        self.processor_status
    }

    pub fn set_processor_status(&mut self, status: u8) {
        self.processor_status = status & 0xCF;
    }

    pub fn get_and_increment_pc(&mut self) -> Address {
        let old_value = self.program_counter;
        self.program_counter = self.program_counter.wrapping_add(1);
        old_value
    }

    pub fn add_program_counter(&mut self, value: u16) {
        self.program_counter = self.program_counter.wrapping_add(value);
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    pub fn push_stack(&mut self) -> u16 {
        let stack = self.stack_pointer as u16 + 0x100;
        if cfg!(feature = "sod") && stack as u8 > self.stack_pointer {
            panic!("STACK OVERFLOW");
        }
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        return stack;
    }

    pub fn pop_stack(&mut self) -> u16 {
        let stack = self.stack_pointer.wrapping_add(1) as u16 + 0x100;
        self.stack_pointer = stack as u8;
        return stack;
    }

}

pub struct CpuBuilder {
    cpu: CPU,
}

impl CpuBuilder {
    pub fn new() -> CpuBuilder {
        CpuBuilder {
            cpu: CPU::new(0x8000),
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

    pub fn flags(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.processor_status = value;
        return self;
    }

    pub fn stack_pointer(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.stack_pointer = value;
        return self;
    }

    pub fn build(&self) -> CPU {
        return self.cpu;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_sbc() {
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::NEGATIVE_FLAG | super::ZERO_FLAG | super::CARRY_FLAG | super::OVERFLOW_FLAG)
                .accumulator(0x02)
                .build();

            cpu.sub_accumulator(0x01);
            assert_eq!(cpu.accumulator, 1);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);

            cpu.sub_accumulator(0x03);
            assert_eq!(cpu.accumulator, 0xFE);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);

            //0xFE - 0x01 - 1 =
            cpu.sub_accumulator(0x01);
            assert_eq!(cpu.accumulator, 0xFC);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::CARRY_FLAG)
                .accumulator(0x02)
                .build();

            cpu.sub_accumulator(0x04);
            assert_eq!(cpu.accumulator, 0xFE);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);

            //0x1FE - 0x100 = 0xFE
            cpu.sub_accumulator(0xFF);
            assert_eq!(cpu.accumulator, 0xFE);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }

        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x00)
                .build();

            //0x00 - 1 - 0xFF = 0x100 - 1 - 0xFF = 0xFF - 0xFF
            cpu.sub_accumulator(0xFF);
            assert_eq!(cpu.accumulator, 0x00);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }
    }

    #[test]
    fn overflow_flag() {
        for s1 in (-128 as i8)..(127 as i8) {
            for s2 in (-128 as i8)..(127 as i8) {
                {
                    let mut cpu = super::CpuBuilder::new()
                        .flags(0)
                        .accumulator(s1 as u8)
                        .build();
                    cpu.add_accumulator(s2 as u8);

                    let actual_sum = (s1 as i16) + (s2 as i16);

                    if actual_sum >= -128 && actual_sum <= 127 {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
                    } else {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
                    }
                }
                {
                    let mut cpu = super::CpuBuilder::new()
                        .flags(0)
                        .accumulator(s1 as u8)
                        .build();
                    cpu.sub_accumulator(s2 as u8);

                    let actual_sum = (s1 as i16) - (s2 as i16) - 1;

                    assert_eq!(s1.wrapping_sub(s2).wrapping_sub(1) as u8, cpu.accumulator());
                    if actual_sum >= -128 && actual_sum <= 127 {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false, "Overflow should be clear: {} - {} - 1", s1, s2);
                    } else {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true, "Overflow should be set: {} - {} - 1", s1, s2);
                    }
                }
                {
                    let mut cpu = super::CpuBuilder::new()
                        .flags(super::CARRY_FLAG)
                        .accumulator(s1 as u8)
                        .build();
                    cpu.add_accumulator(s2 as u8);

                    let actual_sum = (s1 as i16) + (s2 as i16) + 1;

                    if actual_sum >= -128 && actual_sum <= 127 {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
                    } else {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
                    }
                }
                {
                    let mut cpu = super::CpuBuilder::new()
                        .flags(super::CARRY_FLAG)
                        .accumulator(s1 as u8)
                        .build();
                    cpu.sub_accumulator(s2 as u8);

                    let actual_sum = (s1 as i16) - (s2 as i16);

                    if actual_sum >= -128 && actual_sum <= 127 {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
                    } else {
                        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
                    }
                }
            }
        }
    }

    #[test]
    fn test_sub_overflow_flag() {
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::CARRY_FLAG)
                .accumulator(0)
                .build();
            cpu.sub_accumulator(1);
            assert_eq!(cpu.accumulator, 0xFF);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::CARRY_FLAG)
                .accumulator(0x80)
                .build();
            cpu.sub_accumulator(1);
            assert_eq!(cpu.accumulator, 0x7F);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::CARRY_FLAG)
                .accumulator(0x7F)
                .build();

            //127 - -1 = 0x7F - 0xFF = 0x17F - 0xFF = 128 (V = 1)
            cpu.sub_accumulator(0xFF);
            assert_eq!(cpu.accumulator, 0x80);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
        }
    }

    #[test]
    fn test_rotate_right() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::NEGATIVE_FLAG | super::ZERO_FLAG | super::CARRY_FLAG)
            .build();

        let new_value = cpu.rotate_right(0x04);
        assert_eq!(new_value, 0x82);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);

        let new_value = cpu.rotate_right(0x01);
        assert_eq!(new_value, 0x00);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);

        let new_value = cpu.rotate_right(0x01);
        assert_eq!(new_value, 0x80);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
    }

    #[test]
    fn test_rotate_left() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::NEGATIVE_FLAG | super::ZERO_FLAG | super::CARRY_FLAG)
            .build();

        let new_value = cpu.rotate_left(0x02);
        assert_eq!(new_value, 0x05);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);

        let new_value = cpu.rotate_left(0x80);
        assert_eq!(new_value, 0x00);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);

        let new_value = cpu.rotate_left(0x40);
        assert_eq!(new_value, 0x81);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
    }

    #[test]
    fn test_or() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x45)
            .flags(super::ZERO_FLAG)
            .flags(super::NEGATIVE_FLAG)
            .build();

        cpu.or_accumulator(0x08);
        assert_eq!(cpu.accumulator, 0x4D);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);

        cpu.or_accumulator(0x80);
        assert_eq!(cpu.accumulator, 0xCD);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);

        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x00)
                .build();
            cpu.or_accumulator(0x00);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        }
    }
    #[test]
    fn test_lsr() {
        let mut cpu = super::CpuBuilder::new().build();

        assert_eq!(cpu.logical_shift_right(0x02), 0x01);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);

        assert_eq!(cpu.logical_shift_right(0x01), 0x00);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);

        cpu.logical_shift_right(0xFF);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
    }

    #[test]
    fn test_load() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::NEGATIVE_FLAG)
            .build();

        cpu.load_accumulator(0x00);
        assert_eq!(cpu.accumulator, 0x00);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);

        cpu.load_accumulator(0x80);
        assert_eq!(cpu.accumulator, 0x80);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);

        cpu.load_x(0x01);
        assert_eq!(cpu.register_x, 0x01);

        cpu.load_y(0x01);
        assert_eq!(cpu.register_y, 0x01);
    }

    #[test]
    fn test_xor() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x25)
            .build();
        cpu.xor_accumulator(0x01);
        assert_eq!(cpu.accumulator, 0x25 ^ 0x01);

        cpu.xor_accumulator(0x80);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);

        let accumulator = cpu.accumulator;
        cpu.xor_accumulator(accumulator);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
    }

    #[test]
    fn test_increment() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::NEGATIVE_FLAG)
            .build();

        let new_value = cpu.increment(0x02);
        assert_eq!(new_value, 0x03);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);

        let new_value = cpu.increment(0xFF);
        assert_eq!(new_value, 0x00);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);

        let new_value = cpu.increment(0x7F);
        assert_eq!(new_value, 0x80);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
    }

    #[test]
    fn test_decrement() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::NEGATIVE_FLAG)
            .build();

        let new_value = cpu.decrement(0x02);
        assert_eq!(new_value, 0x01);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);


        let new_value = cpu.decrement(0x01);
        assert_eq!(new_value, 0x00);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);

        let new_value = cpu.decrement(0x00);
        assert_eq!(new_value, 0xFF);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
    }

    #[test]
    fn test_cmp_accumulator() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x0A)
            .flags(super::OVERFLOW_FLAG)
            .build();
        cpu.cmp_accumulator(0x01);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);

        cpu.cmp_accumulator(0x0A);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);

        cpu.cmp_accumulator(0x0B);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
    }

    #[test]
    fn test_cmp_80_and_0() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x80)
            .flags(super::OVERFLOW_FLAG)
            .build();
        cpu.cmp_accumulator(0x00);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
    }

    #[test]
    fn test_add_with_carry() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x05)
            .build();
        cpu.add_accumulator(0x02);
        assert_eq!(cpu.accumulator, 0x07);
    }

    #[test]
    fn add_with_carry_should_set_the_carry_flag_on_overflow() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0xFE)
            .build();
        cpu.add_accumulator(0x03);
        assert_eq!(cpu.accumulator, 0x01);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0xFF)
                .build();
            cpu.add_accumulator(1);
            assert_eq!(cpu.accumulator, 0x00);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x80)
                .build();
            cpu.add_accumulator(0xFF);
            assert_eq!(cpu.accumulator, 127);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        }
    }

    #[test]
    fn add_with_carry_should_not_set_the_carry_flag_on_underflow() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .build();
        cpu.add_accumulator(0b1111_1101);
        assert_eq!(cpu.accumulator, 0xFE);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
    }

    #[test]
    fn add_accumulator_should_clear_carry_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .flags(super::CARRY_FLAG)
            .build();
        cpu.add_accumulator(0x02);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false)
    }

    #[test]
    fn add_accumulator_should_use_the_carry_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .flags(super::CARRY_FLAG)
            .build();
        cpu.add_accumulator(0x02);
        assert_eq!(cpu.accumulator, 4);
    }

    #[test]
    fn add_with_carry_should_set_the_zero_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .build();
        cpu.add_accumulator(0b1111_1111);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
    }

    #[test]
    fn add_accumulator_should_clear_the_zero_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .flags(super::ZERO_FLAG)
            .build();
        cpu.add_accumulator(0x02);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
    }

    #[test]
    fn add_with_carry_should_set_the_overflow_flag() {
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(127)
                .build();
            cpu.add_accumulator(1);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(1)
                .build();
            cpu.add_accumulator(1);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(1)
                .build();
            cpu.add_accumulator(0xFF);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x80) //-128
                .build();
            cpu.add_accumulator(0xFF); //-1
            assert_eq!(cpu.accumulator, 0x7F);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
        }
    }

    #[test]
    fn add_accumulator_should_clear_the_overflow_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .flags(super::OVERFLOW_FLAG)
            .build();
        cpu.add_accumulator(0x01);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false)
    }

    #[test]
    fn add_accumulator_should_clear_set_and_clear_negative_flag() {
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x01)
                .build();
            cpu.add_accumulator(0xFE);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0xFF)
                .flags(super::NEGATIVE_FLAG)
                .build();
            cpu.add_accumulator(0x02);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
        }
    }

    #[test]
    fn test_multi_byte_adds() {
        let mut cpu = super::CpuBuilder::new()
            .flags(super::CARRY_FLAG)
            .accumulator(0xFF)
            .build();

        cpu.add_accumulator(0x01);
        assert_eq!(cpu.accumulator, 1);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);

        cpu.add_accumulator(0x01);
        assert_eq!(cpu.accumulator, 3);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
    }

    #[test]
    fn test_and_accumulator() {
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0x15)
                .build();
            cpu.and_accumulator(!0x15);
            assert_eq!(cpu.accumulator, 0x00);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0xFF)
                .build();
            cpu.and_accumulator(0x80);
            assert_eq!(cpu.accumulator, 0x80);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .accumulator(0xFF)
                .flags(super::NEGATIVE_FLAG | super::ZERO_FLAG)
                .build();
            cpu.and_accumulator(0x01);
            assert_eq!(cpu.accumulator, 0x01);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);
        }
    }

    #[test]
    fn test_asl() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .build();
        cpu.asl_accumulator();
        assert_eq!(cpu.accumulator, 0x02);
    }

    #[test]
    fn asl_should_set_carry_and_zero_flag() {
        {
            let mut cpu = super::CpuBuilder::new()
                .flags(super::CARRY_FLAG)
                .build();
            let new_value = cpu.arithmetic_shift_left(0x01);
            assert_eq!(new_value, 0x02);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .build();
            let new_value = cpu.arithmetic_shift_left(0x80);
            assert_eq!(new_value, 0x00);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .build();
            let new_value = cpu.arithmetic_shift_left(0x40);
            assert_eq!(new_value, 0x80);
            assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
        }
    }

    #[test]
    fn test_bit() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x01)
            .flags(super::CARRY_FLAG)
            .build();
        cpu.bit_test(0x02);
        assert_eq!(cpu.accumulator, 0x01);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);

        cpu.bit_test(0x01);
        assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), false);

        cpu.bit_test(0x40);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);

        cpu.bit_test(0x80);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
    }

    #[test]
    fn bit_test_should_clear_negative_flag() {
        let mut cpu = super::CpuBuilder::new()
            .accumulator(0x80)
            .flags(super::NEGATIVE_FLAG)
            .build();

       cpu.bit_test(0x00);
       assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), false);
    }

    extern crate rand;
    const UNUSED_BREAK_FLAG: u8 = 0b0001_0000;
    const UNUSED_FLAG: u8 = 0b0010_0000;
    #[test]
    #[allow(non_snake_case)]
    fn bit_test_should_only_affect_N_V_C_flags() {
        let non_affected_flags =
            UNUSED_BREAK_FLAG | super::DECIMAL_FLAG |
            super::INTERRUPT_DISABLE_FLAG | super::CARRY_FLAG |
            UNUSED_FLAG;
        for _ in 0..100 {
            let accumulator = rand::random::<u8>();
            let flags = rand::random::<u8>() | UNUSED_FLAG;
            let mut cpu = super::CpuBuilder::new()
                .accumulator(accumulator)
                .flags(flags)
                .build();

           let test_mask = rand::random::<u8>();
           cpu.bit_test(test_mask);
           assert_eq!(
            cpu.processor_status & non_affected_flags,
            flags & non_affected_flags,
            "\nFlags: 0b{:08b}\nAccumulator: 0b{:08b}\nMask: 0b{:08b}\nStatus: 0b{:08b}",
            flags, accumulator, test_mask, cpu.processor_status
          );
        }
    }
}
