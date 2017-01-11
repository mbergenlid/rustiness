
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

    pub fn and_accumulator(&mut self, value: u8) {
        self.accumulator &= value;
        if self.accumulator == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }
        if self.accumulator & 0x80 == 0 {
            self.clear_flags(NEGATIVE_FLAG);
        } else {
            self.set_flags(NEGATIVE_FLAG);
        }
    }

    pub fn or_accumulator(&mut self, value: u8) {
        self.accumulator |= value;
    }

    pub fn asl_accumulator(&mut self) {
        let acc = self.accumulator;
        self.asl_into_accumulator(acc);
    }

    pub fn asl_into_accumulator(&mut self, value: u8) {
        if value & 0x80 == 0 {
            self.clear_flags(CARRY_FLAG);
        } else {
            self.set_flags(CARRY_FLAG);
        }
        self.accumulator = value << 1;
        if self.accumulator == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }
        if self.accumulator & 0x80 == 0 {
            self.clear_flags(NEGATIVE_FLAG);
        } else {
            self.set_flags(NEGATIVE_FLAG);
        }
    }

    pub fn bit_test(&mut self, mask: u8) {
        let value = self.accumulator & mask;
        if value == 0 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }

        self.processor_status = (mask & 0xD0) | (self.processor_status & 0xBF);
    }

    pub fn cmp_accumulator(&mut self, value: u8) {
        let left = self.accumulator;
        self.cmp(left, value);
    }

    pub fn cmp_register_x(&mut self, value: u8) {
        let left = self.register_x;
        self.cmp(left, value);
    }

    fn cmp(&mut self, left: u8, right: u8) {
        if left == right {
            self.set_flags(ZERO_FLAG | CARRY_FLAG);
            self.clear_flags(NEGATIVE_FLAG);
        } else if left > right {
            self.set_flags(CARRY_FLAG);
            self.clear_flags(ZERO_FLAG | NEGATIVE_FLAG);
        } else {
            self.set_flags(NEGATIVE_FLAG);
            self.clear_flags(CARRY_FLAG | ZERO_FLAG);
        }
    }

    pub fn decrement(&mut self, value: u8) -> u8 {
        if value == 1 {
            self.set_flags(ZERO_FLAG);
        } else {
            self.clear_flags(ZERO_FLAG);
        }
        let new_value = value.wrapping_sub(1);
        if new_value & 0x80 > 0 {
            self.set_flags(NEGATIVE_FLAG);
        }
        new_value
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

    pub fn flags(&mut self, value: u8) -> &mut CpuBuilder {
        self.cpu.processor_status = value;
        return self;
    }

    pub fn build(&self) -> CPU {
        return self.cpu;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_decrement() {
        let mut cpu = super::CpuBuilder::new()
            .build();

        let new_value = cpu.decrement(0x02);
        assert_eq!(new_value, 0x01);

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
            cpu.asl_into_accumulator(0x01);
            assert_eq!(cpu.accumulator, 0x02);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), false);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .build();
            cpu.asl_into_accumulator(0x80);
            assert_eq!(cpu.accumulator, 0x00);
            assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
            assert_eq!(cpu.is_flag_set(super::ZERO_FLAG), true);
        }
        {
            let mut cpu = super::CpuBuilder::new()
                .build();
            cpu.asl_into_accumulator(0x40);
            assert_eq!(cpu.accumulator, 0x80);
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

        //TODO: test N and V flags.
        cpu.bit_test(0x40);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), true);

        cpu.bit_test(0x80);
        assert_eq!(cpu.is_flag_set(super::CARRY_FLAG), true);
        assert_eq!(cpu.is_flag_set(super::OVERFLOW_FLAG), false);
        assert_eq!(cpu.is_flag_set(super::NEGATIVE_FLAG), true);
    }
}