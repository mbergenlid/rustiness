use nes::cpu::CPU;
use nes::memory::Memory;
use nes::memory::Address;

pub struct AddressingMode {
    pub operand_address: Address
}

impl AddressingMode {
    pub fn zero_paged(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address = memory.get(cpu.get_and_increase_pc(1)) as u16;
        return AddressingMode {
            operand_address: operand_address,
        }
    }

    #[allow(unused_variables)]
    pub fn immediate(cpu: &mut CPU, ignored: &Memory) -> AddressingMode {
        let operand_address = cpu.get_and_increase_pc(1);
        return AddressingMode {
            operand_address: operand_address,
        }
    }

    pub fn zero_paged_index_x(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address: Address = memory.get(cpu.get_and_increase_pc(1)) as Address + cpu.register_x() as Address;
        return AddressingMode {
            operand_address: if operand_address > 0xFF { operand_address & 0xFF } else { operand_address }
        }
    }

    pub fn zero_paged_index_y(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address: Address = memory.get(cpu.get_and_increase_pc(1)) as Address + cpu.register_y() as Address;
        return AddressingMode {
            operand_address: if operand_address > 0xFF { operand_address & 0xFF } else { operand_address }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu;
    use nes::memory::Memory;
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
}