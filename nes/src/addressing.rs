use cpu::CPU;
use memory::Memory;
use memory::Address;

pub struct AddressingMode {
    pub operand_address: Address
}

impl AddressingMode {
    pub fn zero_paged(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address = memory.get(cpu.get_and_increment_pc()) as u16;
        return AddressingMode {
            operand_address: operand_address,
        }
    }

    #[allow(unused_variables)]
    pub fn immediate(cpu: &mut CPU, ignored: &Memory) -> AddressingMode {
        let operand_address = cpu.get_and_increment_pc();
        return AddressingMode {
            operand_address: operand_address,
        }
    }

    pub fn zero_paged_index_x(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address: Address = memory.get(cpu.get_and_increment_pc()) as Address + cpu.register_x() as Address;
        return AddressingMode {
            operand_address: if operand_address > 0xFF { operand_address & 0xFF } else { operand_address }
        }
    }

    pub fn zero_paged_index_y(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address: Address = memory.get(cpu.get_and_increment_pc()) as Address + cpu.register_y() as Address;
        return AddressingMode {
            operand_address: operand_address & 0xFF
        }
    }

    pub fn absolute_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let lsbs: u8 = memory.get(cpu.get_and_increment_pc());
        let msbs: u8 = memory.get(cpu.get_and_increment_pc());
        return AddressingMode {
            operand_address: (msbs as Address) << 8 | lsbs as Address
        }
    }

    pub fn absolute_x_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let lsb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
        let msb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
        let base_address = (msb << 8) | lsb;
        let operand_address = (base_address as u32) + (cpu.register_x() as u32);
        return AddressingMode {
            operand_address: operand_address as u16
        }
    }

    pub fn absolute_y_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let lsb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
        let msb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
        let base_address = (msb << 8) | lsb;
        let operand_address = (base_address as u32) + (cpu.register_y() as u32);
        return AddressingMode {
            operand_address: operand_address as u16
        }
    }

    pub fn indirect_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let base_address = {
            let lsb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
            let msb: u16 = memory.get(cpu.get_and_increment_pc()) as u16;
            (msb << 8) | lsb
        };
        let operand_address = {
            let lsb: u16 = memory.get(base_address) as u16;
            let msb: u16 = memory.get(base_address+1) as u16;
            (msb << 8) | lsb
        };
        return AddressingMode {
            operand_address: operand_address
        }
    }

    pub fn indirect_x_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let index1 = memory.get(cpu.get_and_increment_pc()) as u16;
        let base_address = index1 + cpu.register_x() as u16;

        let operand_address = {
            let lsb: u16 = memory.get(base_address) as u16;
            let msb: u16 = memory.get(base_address+1) as u16;
            (msb << 8) | lsb
        };
        return AddressingMode {
            operand_address: operand_address
        }
    }

    pub fn indirect_y_addressing(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let base_address = memory.get(cpu.get_and_increment_pc()) as u16;
        let operand_address: u32 = {
            let lsb: u16 = memory.get(base_address) as u16;
            let msb: u16 = memory.get(base_address+1) as u16;
            ((msb << 8) | lsb) as u32
        };
        return AddressingMode {
            operand_address: (operand_address + cpu.register_y() as u32) as u16
        }
    }
}

#[cfg(test)]
mod test {
    use cpu;
    use memory::Memory;
    use super::AddressingMode;

    #[test]
    fn test_zero_paged_addressing() {
        let memory = memory!(
            0x8000 => 0xAC,
            0x00AC => 0x0A
        );

        let mut cpu = cpu::CPU::new();

        let addressing = AddressingMode::zero_paged(&mut cpu, &memory);
        assert_eq!(0x00AC, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn test_immediate_addressing() {
        let memory = memory!(
            0x8000 => 0x05
        );
        let mut cpu = cpu::CPU::new();

        let addressing = AddressingMode::immediate(&mut cpu, &memory);
        assert_eq!(0x8000, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn test_zero_paged_indexed_addressing() {
        {
            let memory = memory!(
                0x8000 => 5
            );
            let mut cpu = cpu::CpuBuilder::new()
                .register_x(10)
                .build();
            let addressing = AddressingMode::zero_paged_index_x(&mut cpu, &memory);
            assert_eq!(15, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
        {
            let memory = memory!(
                0x8000 => 5
            );
            let mut cpu = cpu::CpuBuilder::new()
                .register_y(10)
                .build();
            let addressing = AddressingMode::zero_paged_index_y(&mut cpu, &memory);
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
            let mut cpu = cpu::CpuBuilder::new()
                .register_x(0x12)
                .build();
            let addressing = AddressingMode::zero_paged_index_x(&mut cpu, &memory);
            assert_eq!(2, addressing.operand_address);
            assert_eq!(cpu.program_counter(), 0x8001);
        }
        {
            let memory = memory!(
                0x8000 => 0xF0
            );
            let mut cpu = cpu::CpuBuilder::new()
                .register_y(0x12)
                .build();
            let addressing = AddressingMode::zero_paged_index_y(&mut cpu, &memory);
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
        let addressing = AddressingMode::absolute_addressing(&mut cpu, &memory);
        assert_eq!(0xA005, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_absolute_x_addressing() {
        let memory = memory!(
            0x8000 => 0x05,
            0x8001 => 0xA0
        );
        let mut cpu = cpu::CpuBuilder::new()
            .register_x(2)
            .build();
        let addressing = AddressingMode::absolute_x_addressing(&mut cpu, &memory);
        assert_eq!(0xA007, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8002);
    }

    #[test]
    fn indexed_absolute_x_addressing_should_wrap_around() {
        let memory = memory!(
            0x8000 => 0xF0,
            0x8001 => 0xFF
        );
        let mut cpu = cpu::CpuBuilder::new()
            .register_x(0x12)
            .build();
        let addressing = AddressingMode::absolute_x_addressing(&mut cpu, &memory);
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
        let addressing = AddressingMode::indirect_addressing(&mut cpu, &memory);
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
        let mut cpu = cpu::CpuBuilder::new()
            .register_x(0x5)
            .build();
        let addressing = AddressingMode::indirect_x_addressing(&mut cpu, &memory);
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
        let mut cpu = cpu::CpuBuilder::new()
            .register_y(0x04)
            .build();
        let addressing = AddressingMode::indirect_y_addressing(&mut cpu, &memory);
        assert_eq!(0x1234, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }

    #[test]
    fn indirect_indexed_addressing_should_wrap_around() {
        let memory = memory!(
            0x8000 => 0xA0,

            0x00A0 => 0xF0,
            0x00A1 => 0xFF

        );
        let mut cpu = cpu::CpuBuilder::new()
            .register_y(0x12)
            .build();
        let addressing = AddressingMode::indirect_y_addressing(&mut cpu, &memory);
        assert_eq!(0x0002, addressing.operand_address);
        assert_eq!(cpu.program_counter(), 0x8001);
    }
}