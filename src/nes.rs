
use opcodes;
use memory::Address;
use memory::Memory;

pub struct CPU {
    pub program_counter: Address,
//    stack_pointer: u8,
    pub accumulator: u8,
//    register_x: u8,
//    register_y: u8,
//    processor_status: u8
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
        program_counter: 0x8000,
        //        stack_pointer: 0xFF,
        accumulator: 0,
        //        register_x: 0,
        //        register_y: 0,
        //        processor_status: 0,
        }
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

fn execute_one_instruction(cpu: &mut CPU, memory: &mut Memory) {
    let opcode = memory.get(cpu.program_counter);
    cpu.program_counter += 1;

    match opcode {
        opcodes::ADC_IMMEDIATE => adc(AddressingMode::immediate(cpu, memory), cpu, memory),
        opcodes::ADC_ZERO_PAGE => adc(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        _ => println!("Unkown opcode")
    }
}

type Opcode = fn(&mut CPU, &mut Memory);

//
//fn adc_zero_page_indexed(cpu: &mut CPU, memory: &mut Memory) {
//    let base_address = memory.get(cpu.program_counter) as u16;
//    cpu.accumulator = cpu.accumulator + memory.get(base_address + cpu.register_x as u16);
//    cpu.program_counter += 1;
//}
//
//fn adc_absolute(cpu: &mut CPU, memory: &mut Memory) {
//    let address = memory.get(cpu.program_counter) as u16 & (memory.get(cpu.program_counter+1) as u16) << 8;
//    cpu.accumulator = cpu.accumulator + memory.get(address);
//    cpu.program_counter += 2;
//}

type OpCodeExecute = fn(AddressingMode, &mut CPU, &mut Memory);

#[derive(Copy)]
pub struct OpCode(u8, fn(&mut CPU, &Memory) -> AddressingMode, OpCodeExecute);

impl Clone for OpCode {
    fn clone(&self) -> Self {
        OpCode(self.0, self.1, self.2)
    }
}

pub struct OpCodes {
    codes: [Option<OpCode>; 0xFF]
}

impl OpCodes {

    fn new() -> OpCodes {
        let mut codes: [Option<OpCode>; 0xFF] = [None; 0xFF];

        for &op_code in OP_CODES.into_iter() {
            codes[op_code.0 as usize] = Some(op_code);
        }

        return OpCodes {
            codes: codes,
        }
    }

    fn get(&self, code: u8) -> &Option<OpCode> {
        &self.codes[code as usize]
    }
}



fn adc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.add_accumulator(memory.get(addressing_mode.operand_address));
}

const OP_CODES: [OpCode; 2] = [
    OpCode(opcodes::ADC_IMMEDIATE, AddressingMode::immediate, adc),
    OpCode(opcodes::ADC_ZERO_PAGE, AddressingMode::zero_paged, adc),
];

#[cfg(test)]
mod tests {
    use memory::BasicMemory;
    use memory::Memory;
    use super::AddressingMode;
    use opcodes;

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
        let mut memory = BasicMemory::new();
        memory.set(0x8000, 0x69);
        memory.set(0x8001, 0x05);

        let mut cpu = super::CPU::new();

        super::execute_one_instruction(&mut cpu, &mut memory);
        assert_eq!(0x05, cpu.accumulator);
        assert_eq!(0x8002, cpu.program_counter);
    }

    #[test]
    fn test_add_with_carry_zero_page() {
        let mut memory = BasicMemory::new();
        memory.set(0x8000, 0x65);
        memory.set(0x8001, 0xAC);
        memory.set(0x00AC, 0x0A);

        let mut cpu = super::CPU::new();

        super::execute_one_instruction(&mut cpu, &mut memory);
        assert_eq!(10, cpu.accumulator);
        assert_eq!(0x8002, cpu.program_counter);
    }

    #[test]
    fn test_opcodes() {
        let op_codes = super::OpCodes::new();

        assert_eq!(opcodes::ADC_IMMEDIATE, op_codes.get(opcodes::ADC_IMMEDIATE).unwrap().0);
        assert_eq!(opcodes::ADC_ZERO_PAGE, op_codes.get(opcodes::ADC_ZERO_PAGE).unwrap().0);
    }

    #[test]
    fn test_add_with_carry() {
        let mut memory = BasicMemory::new();
        memory.set(0x8000, 0x05);

        let mut cpu = super::CPU::new();

        super::adc(AddressingMode::immediate(&mut cpu, &memory), &mut cpu, &mut memory);
        assert_eq!(0x05, cpu.accumulator);
    }
}