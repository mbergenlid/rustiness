
use opcodes;
use memory::Address;
use memory::Memory;

const NEGATIVE_FLAG: u8 = 0b1000_0000;
const OVERFLOW_FLAG: u8 = 0b0100_0000;
const ZERO_FLAG: u8 = 0b0000_0010;
const CARRY_FLAG: u8 = 0b0000_0001;

struct NES {
    cpu: CPU,
    op_codes: OpCodes,
}

impl NES {
    fn new() -> NES {
        NES {
            cpu: CPU::new(),
            op_codes: OpCodes::new(),
        }
    }

    fn execute_instruction(&mut self, memory: &mut Memory) {
        let opcode = memory.get(self.cpu.program_counter);
        self.cpu.program_counter += 1;

        let instr = { self.op_codes.get(opcode) };
        match instr {
            &Some(ref instruction) => instruction.execute(&mut self.cpu, memory),
            &None => println!("Unkown opcode"),
        }
    }
}

#[derive(Eq, Debug)]
pub struct CPU {
    pub program_counter: Address,
//    stack_pointers: u8,
    pub accumulator: u8,
//    register_x: u8,
//    register_y: u8,
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
        //        register_x: 0,
        //        register_y: 0,
            processor_status: 0
        }
    }

    fn set_flags(&mut self, flags: u8) {
        self.processor_status |= flags;
    }

    fn clear_flags(&mut self, flags: u8) {
        self.processor_status &= !flags;
    }

    fn is_flag_set(&self, flags: u8) -> bool {
        self.processor_status & flags > 0
    }

    pub fn add_accumulator(&mut self, value: u8) {
        self.accumulator += value;
    }
}

pub struct AddressingMode {
    pub operand_address: Address
}

impl AddressingMode {
    pub fn zero_paged(cpu: &mut CPU, memory: &Memory) -> AddressingMode {
        let operand_address = memory.get(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        return AddressingMode {
            operand_address: operand_address,
        }
    }

    #[allow(unused_variables)]
    pub fn immediate(cpu: &mut CPU, ignored: &Memory) -> AddressingMode {
        let operand_address = cpu.program_counter;
        cpu.program_counter += 1;
        return AddressingMode {
            operand_address: operand_address,
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
            Box::new(Branch { op_code: opcodes::BRANCH_PLUS, flag: NEGATIVE_FLAG, inverted: true }),
            Box::new(Branch { op_code: opcodes::BRANCH_MINUS, flag: NEGATIVE_FLAG, inverted: false }),
            Box::new(Branch { op_code: opcodes::BRANCH_OVERFLOW_SET, flag: OVERFLOW_FLAG, inverted: false }),
            Box::new(Branch { op_code: opcodes::BRANCH_OVERFLOW_CLEAR, flag: OVERFLOW_FLAG, inverted: true }),
            Box::new(Branch { op_code: opcodes::BRANCH_CARRY_SET, flag: CARRY_FLAG, inverted: false }),
            Box::new(Branch { op_code: opcodes::BRANCH_CARRY_CLEAR, flag: CARRY_FLAG, inverted: true }),
            Box::new(Branch { op_code: opcodes::BRANCH_NOT_EQUAL, flag: ZERO_FLAG, inverted: true }),
            Box::new(Branch { op_code: opcodes::BRANCH_EQUAL, flag: ZERO_FLAG, inverted: false }),

            Box::new(ADC {op_code: opcodes::ADC_IMMEDIATE, addressing_mode: AddressingMode::immediate, instruction: adc}),
            Box::new(ADC {op_code: opcodes::ADC_ZERO_PAGE, addressing_mode: AddressingMode::zero_paged, instruction: adc}),
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
        let branch_distance = memory.get(cpu.program_counter) as Address;
        cpu.program_counter += 1;
        if condition {
            cpu.program_counter += branch_distance;
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

#[cfg(test)]
mod tests {
    use memory::BasicMemory;
    use memory::Memory;
    use super::AddressingMode;
    use opcodes;


    macro_rules! memory {
        ( $( $x:expr => $y:expr ),* ) => {
            {
                use memory;
                let mut temp_memory = memory::BasicMemory::new();
                $(
                    temp_memory.set($x, $y);
                )*
                temp_memory
            }
        };
    }

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

    #[test]
    fn test_zero_paged_addressing() {
        let mut memory = BasicMemory::new();
        memory.set(0x8001, 0xAC);
        memory.set(0x00AC, 0x0A);

        let mut cpu = super::CPU::new();
        cpu.program_counter = 0x8001;

        let addressing = AddressingMode::zero_paged(&mut cpu, &memory);
        assert_eq!(0x00AC, addressing.operand_address);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    fn test_immediate_addressing() {
        let mut memory = BasicMemory::new();
        memory.set(0x8001, 0x05);

        let mut cpu = super::CPU::new();
        cpu.program_counter = 0x8001;

        let addressing = AddressingMode::immediate(&mut cpu, &memory);
        assert_eq!(0x8001, addressing.operand_address);
        assert_eq!(cpu.program_counter, 0x8002);
    }

    #[test]
    fn test_add_with_carry_immediate() {
        test_instruction(
            &mut memory!(
                0x8000 => 0x69,
                0x8001 => 0x05
            ),
            super::CPU {
                program_counter: 0x8002,
                accumulator: 0x05,
                processor_status: super::CPU::new().processor_status,
            }
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
            super::CPU {
                program_counter: 0x8002,
                accumulator: 10,
                processor_status: super::CPU::new().processor_status,
            }
        )
    }

    fn test_instruction(memory: &mut Memory, expected_cpu: super::CPU) {
        let mut nes = super::NES::new();
        nes.execute_instruction(memory);

        assert_eq!(expected_cpu, nes.cpu);
    }
    #[test]
    fn test_add_with_carry() {
        let mut memory = BasicMemory::new();
        memory.set(0x8000, 0x05);

        let mut cpu = super::CPU::new();

        super::adc(AddressingMode::immediate(&mut cpu, &memory), &mut cpu, &mut memory);
        assert_eq!(0x05, cpu.accumulator);
    }

    #[test]
    fn test_branch_equal() {
        test_branch(super::ZERO_FLAG, opcodes::BRANCH_EQUAL, false);
//        test_branch(super::ZERO_FLAG, opcodes::BRANCH_NOT_EQUAL, true);
//        test_branch(super::NEGATIVE_FLAG, opcodes::BRANCH_MINUS, false);
//        test_branch(super::NEGATIVE_FLAG, opcodes::BRANCH_PLUS, true);
//        test_branch(super::CARRY_FLAG, opcodes::BRANCH_CARRY_SET, false);
//        test_branch(super::CARRY_FLAG, opcodes::BRANCH_CARRY_CLEAR, true);
//        test_branch(super::OVERFLOW_FLAG, opcodes::BRANCH_OVERFLOW_SET, false);
//        test_branch(super::OVERFLOW_FLAG, opcodes::BRANCH_OVERFLOW_CLEAR, true);
    }

    fn test_branch(flag: u8, op_code: u8, negative: bool) {
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
                assert_eq!(0x8002, cpu.program_counter);
            } else {
                assert_eq!(0x8008, cpu.program_counter);
            }
        }

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
                assert_eq!(0x8008, cpu.program_counter);
            } else {
                assert_eq!(0x8002, cpu.program_counter);
            }
        }
    }
}