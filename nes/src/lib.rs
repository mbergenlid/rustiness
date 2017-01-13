
pub mod cpu;
#[macro_use] pub mod memory;
mod opcodes;
mod instructions;
pub mod addressing;
pub mod ppu;

use cpu::CPU;
use memory::Memory;

pub struct NES {
    cpu: CPU,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: CPU::new(),
        }
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        self.execute_instruction(memory);
    }

    fn execute_instruction(&mut self, memory: &mut Memory) {
        opcodes::execute_instruction(&mut self.cpu, memory);
    }
}

#[cfg(test)]
mod tests {
    use cpu;
    use memory::Memory;
    use opcodes;

    macro_rules! instruction_test {
        ( $name:expr, $memory:expr, $expected_cpu:expr ) => {
            {
                #[test]
                fn test_$name() {
                    test_instruction(memory, expected_cpu);
                }
            }
        };
    }

    fn test_program(memory: &mut Memory, expected_cpu_states: &[cpu::CPU]) {
        let mut nes = super::NES::new();

        for &expected_cpu in expected_cpu_states.iter() {
            nes.execute_instruction(memory);
            assert_eq!(expected_cpu, nes.cpu);
        }
    }

    #[test]
    fn test() {
        test_program(
            &mut memory!(
                //ADC $05
                0x8000 => 0x69,
                0x8001 => 0x05,

                //AND $00
                0x8002 => 0x29,
                0x8003 => 0x00,
                //ORA $05
                0x8004 => opcodes::ORA_IMMEDIATE,
                0x8005 => 0x05,

                0x8006 => opcodes::ASL_ACCUMULATOR
            ),
            &[
                cpu::CpuBuilder::new()
                    .program_counter(0x8002)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
                    .flags(cpu::ZERO_FLAG)
                    .accumulator(0x00)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8006)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8007)
                    .accumulator(0x0A) //1010
                    .build()
            ]
        )
    }

    #[test]
    fn test_add_with_carry_immediate() {
        test_instruction(
            &mut memory!(
                0x8000 => 0x69,
                0x8001 => 0x05
            ),
            cpu::CpuBuilder::new()
                .program_counter(0x8002)
                .accumulator(0x05)
                .build()
        );
    }

    #[test]
    fn test_add_with_carry_zero_page() {
        test_instruction(
            &mut memory!(
                0x8000 => 0x65,
                0x8001 => 0xAC,
                0x00AC => 0x0A
            ),
            cpu::CpuBuilder::new()
                .program_counter(0x8002)
                .accumulator(10)
                .build()
        )
    }

    fn test_instruction(memory: &mut Memory, expected_cpu: cpu::CPU) {
        let mut nes = super::NES::new();
        nes.execute_instruction(memory);

        assert_eq!(expected_cpu, nes.cpu);
    }

    #[test]
    fn test_branch_equal() {
        test_branch(cpu::ZERO_FLAG, opcodes::BRANCH_EQUAL, false);
        test_branch(cpu::ZERO_FLAG, opcodes::BRANCH_NOT_EQUAL, true);
        test_branch(cpu::NEGATIVE_FLAG, opcodes::BRANCH_MINUS, false);
        test_branch(cpu::NEGATIVE_FLAG, opcodes::BRANCH_PLUS, true);
        test_branch(cpu::CARRY_FLAG, opcodes::BRANCH_CARRY_SET, false);
        test_branch(cpu::CARRY_FLAG, opcodes::BRANCH_CARRY_CLEAR, true);
        test_branch(cpu::OVERFLOW_FLAG, opcodes::BRANCH_OVERFLOW_SET, false);
        test_branch(cpu::OVERFLOW_FLAG, opcodes::BRANCH_OVERFLOW_CLEAR, true);
    }

    fn test_branch(flag: u8, op_code: u8, negative: bool) {
        {
            {
                let mut memory = memory!(
                    0x8000 => op_code,
                    0x8001 => 0x06
                );

                let mut nes = super::NES::new();
                nes.cpu.set_flags(flag);
                nes.execute_instruction(&mut memory);
                let cpu = &nes.cpu;
                if negative {
                    assert_eq!(0x8002, cpu.program_counter());
                } else {
                    assert_eq!(0x8008, cpu.program_counter());
                }
            }

            {
                let mut memory = memory!(
                    0x8000 => op_code,
                    0x8001 => 0b1111_1010 // -6
                );

                let mut nes = super::NES::new();
                nes.cpu.set_flags(flag);
                nes.execute_instruction(&mut memory);
                let cpu = &nes.cpu;
                if negative {
                    assert_eq!(0x8002, cpu.program_counter());
                } else {
                    assert_eq!(0x7FFC, cpu.program_counter());
                }
            }
        }

        {
            {
                let mut memory = memory!(
                    0x8000 => op_code,
                    0x8001 => 0x06
                );

                let mut nes = super::NES::new();
                nes.cpu.clear_flags(flag);
                nes.execute_instruction(&mut memory);
                let cpu = &nes.cpu;
                if negative {
                    assert_eq!(0x8008, cpu.program_counter());
                } else {
                    assert_eq!(0x8002, cpu.program_counter());
                }
            }

            {
                let mut memory = memory!(
                    0x8000 => op_code,
                    0x8001 => 0b1111_1010 // -6
                );

                let mut nes = super::NES::new();
                nes.cpu.clear_flags(flag);
                nes.execute_instruction(&mut memory);
                let cpu = &nes.cpu;
                if negative {
                    assert_eq!(0x7FFC, cpu.program_counter());
                } else {
                    assert_eq!(0x8002, cpu.program_counter());
                }
            }
        }
    }
}