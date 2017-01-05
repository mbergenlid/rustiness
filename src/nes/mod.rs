
mod cpu;
#[macro_use] mod memory;
mod opcodes;
mod addressing;

use nes::cpu::CPU;
use nes::memory::Memory;
use nes::addressing::AddressingMode;

pub struct NES {
    cpu: CPU,
    op_codes: OpCodes,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: CPU::new(),
            op_codes: OpCodes::new(),
        }
    }

    fn execute_instruction(&mut self, memory: &mut Memory) {
        let opcode = memory.get(self.cpu.get_and_increment_pc());

        let instr = { self.op_codes.get(opcode) };
        match instr {
            &Some(ref instruction) => instruction.execute(&mut self.cpu, memory),
            &None => println!("Unkown opcode"),
        }
    }
}

type OpCodeExecute = fn(AddressingMode, &mut CPU, &mut Memory);
type AddressingModeConstructor = fn(&mut CPU, &Memory) -> AddressingMode;

pub struct OpCodes {
    codes: Vec<Option<Box<Instruction>>>,
}

impl OpCodes {

    fn new() -> OpCodes {
        let all_codes: Vec<Box<Instruction>> = vec![
        Box::new(Branch { op_code: opcodes::BRANCH_PLUS, flag: cpu::NEGATIVE_FLAG, inverted: true }),
        Box::new(Branch { op_code: opcodes::BRANCH_MINUS, flag: cpu::NEGATIVE_FLAG, inverted: false }),
        Box::new(Branch { op_code: opcodes::BRANCH_OVERFLOW_SET, flag: cpu::OVERFLOW_FLAG, inverted: false }),
        Box::new(Branch { op_code: opcodes::BRANCH_OVERFLOW_CLEAR, flag: cpu::OVERFLOW_FLAG, inverted: true }),
        Box::new(Branch { op_code: opcodes::BRANCH_CARRY_SET, flag: cpu::CARRY_FLAG, inverted: false }),
        Box::new(Branch { op_code: opcodes::BRANCH_CARRY_CLEAR, flag: cpu::CARRY_FLAG, inverted: true }),
        Box::new(Branch { op_code: opcodes::BRANCH_NOT_EQUAL, flag: cpu::ZERO_FLAG, inverted: true }),
        Box::new(Branch { op_code: opcodes::BRANCH_EQUAL, flag: cpu::ZERO_FLAG, inverted: false }),

        Box::new(ADC {op_code: opcodes::ADC_IMMEDIATE, addressing_mode: AddressingMode::immediate, instruction: adc}),
        Box::new(ADC {op_code: opcodes::ADC_ZERO_PAGE, addressing_mode: AddressingMode::zero_paged, instruction: adc}),

        Box::new(ADC {op_code: opcodes::AND_IMMEDIATE, addressing_mode: AddressingMode::immediate, instruction: and}),
        ];

        let mut codes: Vec<Option<Box<Instruction>>> = vec![];
        for _ in 0..0xFF {
            codes.push(None);
        }

        for op_code in all_codes.into_iter() {
            let c = op_code.op_code();
            codes[c as usize] = Some(op_code);
        }

        return OpCodes {
            codes: codes,
        }
    }

    fn get(&self, code: u8) -> &Option<Box<Instruction>> {
        let instr: &Option<Box<Instruction>> = &self.codes[code as usize];
        return instr;
    }
}


trait Instruction {
    fn op_code(&self) -> u8;
    fn execute(&self, &mut CPU, &mut Memory);
}

struct Branch {
    op_code: u8,
    flag: u8,
    inverted: bool,
}

impl Instruction for Branch {
    fn op_code(&self) -> u8 {
        self.op_code
    }

    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let condition =
            if self.inverted {
                !cpu.is_flag_set(self.flag)
            } else {
                cpu.is_flag_set(self.flag)
            };

        //
        //0b0000_0000_0000_0100
        //0b1111_1111_1111_1100
        //------------
        //0b0000_0000
        let branch_distance: i8 = memory.get(cpu.get_and_increment_pc()) as i8;
        println!("Branch distance: {:#b}", branch_distance as u16);
        println!("Branch distance: {:#b}", cpu.program_counter());
        if condition {
            cpu.add_program_counter(branch_distance as u16);
        }
    }
}

struct ADC {
    op_code: u8,
    addressing_mode: AddressingModeConstructor,
    instruction: OpCodeExecute,
}

impl Instruction for ADC {
    fn op_code(&self) -> u8 {
        self.op_code
    }

    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        (self.instruction)((self.addressing_mode)(cpu, memory), cpu, memory);
    }
}

fn adc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.add_accumulator(memory.get(addressing_mode.operand_address));
}

fn and(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.and_accumulator(memory.get(addressing_mode.operand_address));
}

#[cfg(test)]
mod tests {
    use nes::cpu;
    use nes::memory::Memory;
    use nes::opcodes;

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
                0x8003 => 0x00
            ),
            &[
                cpu::CpuBuilder::new()
                    .program_counter(0x8002)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
                    .accumulator(0x00)
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