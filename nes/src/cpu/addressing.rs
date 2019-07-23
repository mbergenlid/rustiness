use cpu::CPU;
use memory::Address;
use memory::Memory;

pub const NO_ADDRESSING: AddressingMode = AddressingMode {
    cycles: 0,
    operand_address: 0,
};
pub struct AddressingMode {
    pub operand_address: Address,
    pub cycles: u8,
}

impl AddressingMode {
    pub fn immediate(cpu: &mut CPU) -> AddressingMode {
        let operand_address = cpu.get_and_increment_pc();
        return AddressingMode {
            cycles: 1,
            operand_address: operand_address,
        };
    }

    pub fn zero_paged(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let operand_address = memory.get(cpu.get_and_increment_pc(), 1) as u16;
        return AddressingMode {
            cycles: 2,
            operand_address: operand_address,
        };
    }

    pub fn zero_paged_x(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let operand_address: Address =
            memory.get(cpu.get_and_increment_pc(), 1) as Address + cpu.register_x() as Address;
        return AddressingMode {
            cycles: 3,
            operand_address: if operand_address > 0xFF {
                operand_address & 0xFF
            } else {
                operand_address
            },
        };
    }

    pub fn zero_paged_y(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let operand_address: Address =
            memory.get(cpu.get_and_increment_pc(), 1) as Address + cpu.register_y() as Address;
        return AddressingMode {
            cycles: 3,
            operand_address: operand_address & 0xFF,
        };
    }

    pub fn absolute(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let lsbs: u8 = memory.get(cpu.get_and_increment_pc(), 1);
        let msbs: u8 = memory.get(cpu.get_and_increment_pc(), 2);
        return AddressingMode {
            cycles: 3,
            operand_address: (msbs as Address) << 8 | lsbs as Address,
        };
    }

    pub fn absolute_x(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let x = cpu.register_x();
        AddressingMode::absolute_indexed(cpu, memory, x)
    }

    pub fn absolute_y(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let y = cpu.register_y();
        AddressingMode::absolute_indexed(cpu, memory, y)
    }

    fn absolute_indexed(cpu: &mut CPU, memory: &dyn Memory, index: u8) -> AddressingMode {
        let lsb: u16 = memory.get(cpu.get_and_increment_pc(), 1) as u16;
        let msb: u16 = memory.get(cpu.get_and_increment_pc(), 2) as u16;
        let base_address = (msb << 8) | lsb;
        let operand_address = (base_address as u32) + (index as u32);
        let cycles = if (operand_address as u16 >> 8) > msb {
            memory.get((msb << 8) | (lsb as u8).wrapping_add(index) as u16, 3); //dummy read
            4
        } else {
            3
        };
        return AddressingMode {
            cycles: cycles,
            operand_address: operand_address as u16,
        };
    }

    pub fn indirect(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let ial = memory.get(cpu.get_and_increment_pc(), 1);
        let iah = memory.get(cpu.get_and_increment_pc(), 2);

        let adl = memory.get(((iah as u16) << 8) | ial as u16, 3) as u16;
        let adh = memory.get(((iah as u16) << 8) | ial.wrapping_add(1) as u16, 4) as u16;
        let operand_address = (adh << 8) | adl;

        return AddressingMode {
            cycles: 5,
            operand_address: operand_address,
        };
    }

    pub fn indirect_x(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let index = memory.get(cpu.get_and_increment_pc(), 1);
        let base_address = index.wrapping_add(cpu.register_x());

        let operand_address = {
            let lsb: u16 = memory.get(base_address as u16, 3) as u16;
            let msb: u16 = memory.get(base_address.wrapping_add(1) as u16, 4) as u16;
            (msb << 8) | lsb
        };
        return AddressingMode {
            cycles: 5,
            operand_address: operand_address,
        };
    }

    pub fn indirect_y(cpu: &mut CPU, memory: &dyn Memory) -> AddressingMode {
        let ial = memory.get(cpu.get_and_increment_pc(), 1);
        let bal = memory.get(ial as u16, 2);
        let bah = memory.get(ial.wrapping_add(1) as u16, 3);

        let base_address = ((bah as u16) << 8) | bal as u16;

        let operand_address = base_address.wrapping_add(cpu.register_y() as u16);
        let cycles = if (operand_address >> 8) > (base_address >> 8) {
            memory.get(
                (base_address & 0xFF00) | (bal as u8).wrapping_add(cpu.register_y()) as u16,
                3,
            ); //dummy read
            5
        } else {
            4
        };
        return AddressingMode {
            cycles: cycles,
            operand_address: operand_address,
        };
    }
}

#[cfg(test)]
mod test {
    use super::AddressingMode;
    use cpu;

    #[test]
    fn test_zero_paged_addressing() {
        let memory = memory!(
            0x8000 => 0xAC,
            0x00AC => 0x0A
        );

        let mut cpu = cpu::CPU::new(0x8000);

        let addressing = AddressingMode::zero_paged(&mut cpu, &memory);
        assert_eq!(0x00AC, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn test_immediate_addressing() {
        let mut cpu = cpu::CPU::new(0x8000);

        let addressing = AddressingMode::immediate(&mut cpu);
        assert_eq!(0x8000, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn test_zero_paged_indexed_addressing() {
        {
            let memory = memory!(
                0x8000 => 5
            );
            let mut cpu = cpu::CpuBuilder::new().register_x(10).build();
            let addressing = AddressingMode::zero_paged_x(&mut cpu, &memory);
            assert_eq!(15, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
        {
            let memory = memory!(
                0x8000 => 5
            );
            let mut cpu = cpu::CpuBuilder::new().register_y(10).build();
            let addressing = AddressingMode::zero_paged_y(&mut cpu, &memory);
            assert_eq!(15, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
    }

    #[test]
    fn zero_paged_indexed_addressing_should_wrap_around() {
        {
            let memory = memory!(
                0x8000 => 0xF0
            );
            let mut cpu = cpu::CpuBuilder::new().register_x(0x12).build();
            let addressing = AddressingMode::zero_paged_x(&mut cpu, &memory);
            assert_eq!(2, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
        {
            let memory = memory!(
                0x8000 => 0xF0
            );
            let mut cpu = cpu::CpuBuilder::new().register_y(0x12).build();
            let addressing = AddressingMode::zero_paged_y(&mut cpu, &memory);
            assert_eq!(2, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
    }

    #[test]
    fn absolute_addressing() {
        let memory = memory!(
            0x8000 => 0x05,
            0x8001 => 0xA0
        );
        let mut cpu = cpu::CpuBuilder::new().build();
        let addressing = AddressingMode::absolute(&mut cpu, &memory);
        assert_eq!(0xA005, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_absolute_x_addressing() {
        let memory = memory!(
            0x8000 => 0x05,
            0x8001 => 0xA0
        );
        let mut cpu = cpu::CpuBuilder::new().register_x(2).build();
        let addressing = AddressingMode::absolute_x(&mut cpu, &memory);
        assert_eq!(0xA007, addressing.operand_address);
        assert_eq!(3, addressing.cycles);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_absolute_x_addressing_with_page_crossing() {
        let memory = memory!(
            0x8000 => 0xF0,
            0x8001 => 0xA0
        );
        let mut cpu = cpu::CpuBuilder::new().register_x(0x10).build();
        let addressing = AddressingMode::absolute_x(&mut cpu, &memory);
        assert_eq!(0xA100, addressing.operand_address);
        assert_eq!(4, addressing.cycles);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_absolute_x_addressing_should_wrap_around() {
        let memory = memory!(
            0x8000 => 0xF0,
            0x8001 => 0xFF
        );
        let mut cpu = cpu::CpuBuilder::new().register_x(0x12).build();
        let addressing = AddressingMode::absolute_x(&mut cpu, &memory);
        assert_eq!(0x0002, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indirect_addressing() {
        let memory = memory!(
            0x8000 => 0xF0,
            0x8001 => 0xFF,

            0xFFF0 => 0x05,
            0xFFF1 => 0xA0

        );
        let mut cpu = cpu::CpuBuilder::new().build();
        let addressing = AddressingMode::indirect(&mut cpu, &memory);
        assert_eq!(0xA005, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_indirect_addressing() {
        let memory = memory!(
            0x8000 => 0xA0,

            0x00A5 => 0x34,
            0x00A6 => 0x12

        );
        let mut cpu = cpu::CpuBuilder::new().register_x(0x5).build();
        let addressing = AddressingMode::indirect_x(&mut cpu, &memory);
        assert_eq!(0x1234, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indexed_indirect_addressing_should_wrap_around_on_index() {
        let memory = memory!(
            0x8000 => 0xFE,

            0x0000 => 0x34,
            0x0001 => 0x12
        );
        let mut cpu = cpu::CpuBuilder::new().register_x(0x2).build();
        let addressing = AddressingMode::indirect_x(&mut cpu, &memory);
        assert_eq!(0x1234, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indexed_indirect_addressing_should_wrap_around() {
        let memory = memory!(
            0x8000 => 0xFF,

            0x0000 => 0x12,
            0x00FF => 0x34,

            0x00A5 => 0x34,
            0x00A6 => 0x12

        );
        let mut cpu = cpu::CpuBuilder::new().register_x(0x0).build();
        let addressing = AddressingMode::indirect_x(&mut cpu, &memory);
        assert_eq!(0x1234, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indirect_indexed_addressing() {
        let memory = memory!(
            0x8000 => 0xA0,

            0x00A0 => 0x30,
            0x00A1 => 0x12

        );
        let mut cpu = cpu::CpuBuilder::new().register_y(0x04).build();
        let addressing = AddressingMode::indirect_y(&mut cpu, &memory);
        assert_eq!(0x1234, addressing.operand_address);
        assert_eq!(4, addressing.cycles);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indirect_indexed_addressing_with_page_crossing() {
        let memory = memory!(
            0x8000 => 0xA0,

            0x00A0 => 0xF0,
            0x00A1 => 0xA0

        );
        let mut cpu = cpu::CpuBuilder::new().register_y(0x10).build();
        let addressing = AddressingMode::indirect_y(&mut cpu, &memory);
        assert_eq!(0xA100, addressing.operand_address);
        assert_eq!(5, addressing.cycles);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indirect_indexed_addressing_should_wrap_around() {
        let memory = memory!(
            0x8000 => 0xA0,

            0x00A0 => 0xF0,
            0x00A1 => 0xFF

        );
        let mut cpu = cpu::CpuBuilder::new().register_y(0x12).build();
        let addressing = AddressingMode::indirect_y(&mut cpu, &memory);
        assert_eq!(0x0002, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }
}
