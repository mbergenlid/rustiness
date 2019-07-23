use nes;
use nes::cpu::addressing::AddressingMode;
use nes::cpu::CPU;
use nes::memory::{CPUMemory, CPUMemoryReference, Memory};
use std::string::String;

pub struct OpCodeDebug(
    u8,
    &'static str,
    Box<Fn(&mut CPU, &Memory) -> AddressingMode>,
    Box<Fn(&mut CPU, &CPUMemory) -> String>,
);

pub struct OpCodes {
    codes: Vec<OpCodeDebug>,
}

impl OpCodes {
    pub fn new() -> OpCodes {
        OpCodes {
            codes: generate_opcode_debug(),
        }
    }

    pub fn addressing_mode(&self, cpu: &CPU, memory: &Memory) -> AddressingMode {
        let mut cloned_cpu = cpu.clone();
        let op_code = memory.get(cloned_cpu.get_and_increment_pc(), 0);
        match self.codes.iter().find(|opd| opd.0 == op_code) {
            Some(&OpCodeDebug(_, _, ref addr, _)) => addr(&mut cloned_cpu, memory),
            None => no_addressing(),
        }
    }

    pub fn debug_instruction(&self, op_code: u8, cpu: &CPU, memory: &CPUMemory) -> String {
        let mut cloned_cpu = cpu.clone();
        cloned_cpu.get_and_increment_pc();
        match self.codes.iter().find(|opd| opd.0 == op_code) {
            Some(&OpCodeDebug(_, ref name, _, ref args)) => format!(
                "${:04x}: {} (0x{:x}): {}",
                cpu.program_counter(),
                name,
                op_code,
                args(&mut cloned_cpu, memory)
            ),
            None => format!("Next instruction is unknown: {:x}", op_code),
        }
    }
}

fn debug_immediate(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(AddressingMode::immediate(cpu).operand_address, memory)
    );
}

fn debug_zero_paged(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::zero_paged(cpu, memory).operand_address,
            memory
        )
    );
}

fn debug_zero_paged_x(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::zero_paged_x(cpu, memory).operand_address,
            memory
        )
    );
}

fn debug_zero_paged_y(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::zero_paged_y(cpu, memory).operand_address,
            memory
        )
    );
}

fn debug_absolute(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::absolute(cpu, memory).operand_address,
            memory
        )
    );
}
fn debug_absolute_x(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::absolute_x(cpu, memory).operand_address,
            memory
        )
    );
}
fn debug_absolute_y(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::absolute_y(cpu, memory).operand_address,
            memory
        )
    );
}
fn debug_indirect(cpu: &mut CPU, memory: &CPUMemory) -> String {
    return format!(
        "address {:?}",
        CPUMemoryReference(
            AddressingMode::indirect(cpu, memory).operand_address,
            memory
        )
    );
}

fn debug_indirect_y(cpu: &mut CPU, memory: &CPUMemory) -> String {
    let ial = memory.get(cpu.program_counter(), 0);
    let bal = memory.get(ial as u16, 0);
    let bah = memory.get(ial.wrapping_add(1) as u16, 0);

    let base_address = ((bah as u16) << 8) | bal as u16;
    let operand_address = base_address.wrapping_add(cpu.register_y() as u16);

    return format!(
        "address {:?} ({:04x} -> 0x{:02x}, {:04x} -> 0x{:02x} => {:02x}{:02x} + {:02x})",
        CPUMemoryReference(operand_address, memory),
        ial,
        bal,
        ial.wrapping_add(1),
        bah,
        bah,
        bal,
        cpu.register_y()
    );
}
fn debug_indirect_x(cpu: &mut CPU, memory: &CPUMemory) -> String {
    let index = memory.get(cpu.program_counter(), 0);
    let base_address = index.wrapping_add(cpu.register_x());
    let lsb: u16 = memory.get(base_address as u16, 0) as u16;
    let msb: u16 = memory.get(base_address.wrapping_add(1) as u16, 0) as u16;
    let operand_address = (msb << 8) | lsb;

    return format!(
        "address {:?} (Base: 0x{:02x} + {:02x} = 0x{:02x}) => 0x{:02x} -> {:02x}, 0x{:02x} -> {:02x})",
        CPUMemoryReference(operand_address, memory),
        index,
        cpu.register_x(),
        base_address,
        base_address,
        lsb,
        base_address.wrapping_add(1),
        msb
    );
}

fn no_addressing() -> AddressingMode {
    AddressingMode {
        cycles: 0,
        operand_address: 0,
    }
}

fn generate_opcode_debug() -> Vec<OpCodeDebug> {
    let opcodes: Vec<OpCodeDebug> = vec![
        OpCodeDebug(
            nes::cpu::opcodes::ADC_IMMEDIATE,
            "ADC_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_ZERO_PAGE,
            "ADC_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_ZERO_PAGE_X,
            "ADC_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_ABSOLUTE,
            "ADC_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_ABSOLUTE_X,
            "ADC_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_ABSOLUTE_Y,
            "ADC_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_INDIRECT_X,
            "ADC_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ADC_INDIRECT_Y,
            "ADC_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_IMMEDIATE,
            "AND_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_ZERO_PAGE,
            "AND_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_ZERO_PAGE_X,
            "AND_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_ABSOLUTE,
            "AND_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_ABSOLUTE_X,
            "AND_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_ABSOLUTE_Y,
            "AND_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_INDIRECT_X,
            "AND_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::AND_INDIRECT_Y,
            "AND_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ASL_ACCUMULATOR,
            "ASL_ACCUMULATOR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ASL_ZERO_PAGE,
            "ASL_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ASL_ZERO_PAGE_X,
            "ASL_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ASL_ABSOLUTE,
            "ASL_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ASL_ABSOLUTE_X,
            "ASL_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BIT_ZERO_PAGE,
            "BIT_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BIT_ABSOLUTE,
            "BIT_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_PLUS,
            "BRANCH_PLUS",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_MINUS,
            "BRANCH_MINUS",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_OVERFLOW_SET,
            "BRANCH_OVERFLOW_SET",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_OVERFLOW_CLEAR,
            "BRANCH_OVERFLOW_CLEAR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_CARRY_SET,
            "BRANCH_CARRY_SET",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_CARRY_CLEAR,
            "BRANCH_CARRY_CLEAR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_NOT_EQUAL,
            "BRANCH_NOT_EQUAL",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRANCH_EQUAL,
            "BRANCH_EQUAL",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::BRK,
            "BRK_IMPLIED",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, memory| {
                format!("0x{:x}{:x}", memory.get(0xFFFF, 0), memory.get(0xFFFE, 0))
            }),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_IMMEDIATE,
            "CMP_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_ZERO_PAGE,
            "CMP_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_ZERO_PAGE_X,
            "CMP_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_ABSOLUTE,
            "CMP_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_ABSOLUTE_X,
            "CMP_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_ABSOLUTE_Y,
            "CMP_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_INDIRECT_X,
            "CMP_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CMP_INDIRECT_Y,
            "CMP_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPX_IMMEDIATE,
            "CPX_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPX_ZERO_PAGE,
            "CPX_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPX_ABSOLUTE,
            "CPX_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPY_IMMEDIATE,
            "CPY_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPY_ZERO_PAGE,
            "CPY_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CPY_ABSOLUTE,
            "CPY_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEC_ZERO_PAGE,
            "DEC_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEC_ZERO_PAGE_X,
            "DEC_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEC_ABSOLUTE,
            "DEC_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEC_ABSOLUTE_X,
            "DEC_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_IMMEDIATE,
            "EOR_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_ZERO_PAGE,
            "EOR_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_ZERO_PAGE_X,
            "EOR_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_ABSOLUTE,
            "EOR_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_ABSOLUTE_X,
            "EOR_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_ABSOLUTE_Y,
            "EOR_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_INDIRECT_X,
            "EOR_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::EOR_INDIRECT_Y,
            "EOR_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CLC,
            "CLC",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SEC,
            "SEC",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CLI,
            "CLI",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SEI,
            "SEI",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CLV,
            "CLV",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::CLD,
            "CLD",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SED,
            "SED",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INC_ZERO_PAGE,
            "INC_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INC_ZERO_PAGE_X,
            "INC_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INC_ABSOLUTE,
            "INC_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INC_ABSOLUTE_X,
            "INC_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::JMP_ABSOLUTE,
            "JMP_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::JMP_INDIRECT,
            "JMP_INDIRECT",
            Box::new(AddressingMode::indirect),
            Box::new(debug_indirect),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::JSR_ABSOLUTE,
            "JSR_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_IMMEDIATE,
            "LDA_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_ZERO_PAGE,
            "LDA_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_ZERO_PAGE_X,
            "LDA_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_ABSOLUTE,
            "LDA_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_ABSOLUTE_X,
            "LDA_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_ABSOLUTE_Y,
            "LDA_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_INDIRECT_X,
            "LDA_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDA_INDIRECT_Y,
            "LDA_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDX_IMMEDIATE,
            "LDX_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDX_ZERO_PAGE,
            "LDX_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDX_ZERO_PAGE_Y,
            "LDX_ZERO_PAGE_Y",
            Box::new(AddressingMode::zero_paged_y),
            Box::new(debug_zero_paged_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDX_ABSOLUTE,
            "LDX_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDX_ABSOLUTE_Y,
            "LDX_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDY_IMMEDIATE,
            "LDY_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDY_ZERO_PAGE,
            "LDY_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDY_ZERO_PAGE_X,
            "LDY_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDY_ABSOLUTE,
            "LDY_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LDY_ABSOLUTE_X,
            "LDY_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LSR_ACCUMULATOR,
            "LSR_ACCUMULATOR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LSR_ZERO_PAGE,
            "LSR_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LSR_ZERO_PAGE_X,
            "LSR_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LSR_ABSOLUTE,
            "LSR_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::LSR_ABSOLUTE_X,
            "LSR_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::NOP_IMPLIED,
            "NOP_IMPLIED",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_IMMEDIATE,
            "ORA_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_ZERO_PAGE,
            "ORA_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_ZERO_PAGE_X,
            "ORA_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_ABSOLUTE,
            "ORA_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_ABSOLUTE_X,
            "ORA_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_ABSOLUTE_Y,
            "ORA_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_INDIRECT_X,
            "ORA_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ORA_INDIRECT_Y,
            "ORA_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TAX,
            "TAX",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TXA,
            "TXA",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEX,
            "DEX",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INX,
            "INX",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TAY,
            "TAY",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TYA,
            "TYA",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::DEY,
            "DEY",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::INY,
            "INY",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROL_ACCUMULATOR,
            "ROL_ACCUMULATOR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROL_ZERO_PAGE,
            "ROL_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROL_ZERO_PAGE_X,
            "ROL_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROL_ABSOLUTE,
            "ROL_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROL_ABSOLUTE_X,
            "ROL_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROR_ACCUMULATOR,
            "ROR_ACCUMULATOR",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROR_ZERO_PAGE,
            "ROR_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROR_ZERO_PAGE_X,
            "ROR_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROR_ABSOLUTE,
            "ROR_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ROR_ABSOLUTE_X,
            "ROR_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::RTI,
            "RTI_IMPLIED",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::RTS,
            "RTS",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_IMMEDIATE,
            "SBC_IMMEDIATE",
            Box::new(|cpu, _| AddressingMode::immediate(cpu)),
            Box::new(debug_immediate),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_ZERO_PAGE,
            "SBC_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_ZERO_PAGE_X,
            "SBC_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_ABSOLUTE,
            "SBC_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_ABSOLUTE_X,
            "SBC_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_ABSOLUTE_Y,
            "SBC_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_INDIRECT_X,
            "SBC_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SBC_INDIRECT_Y,
            "SBC_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_ZERO_PAGE,
            "STA_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_ZERO_PAGE_X,
            "STA_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_ABSOLUTE,
            "STA_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_ABSOLUTE_X,
            "STA_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_ABSOLUTE_Y,
            "STA_ABSOLUTE_Y",
            Box::new(AddressingMode::absolute_y),
            Box::new(debug_absolute_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_INDIRECT_X,
            "STA_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STA_INDIRECT_Y,
            "STA_INDIRECT_Y",
            Box::new(AddressingMode::indirect_y),
            Box::new(debug_indirect_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TXS,
            "TXS",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::TSX,
            "TSX",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::PHA,
            "PHA",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::PLA,
            "PLA",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::PHP,
            "PHP",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::PLP,
            "PLP",
            Box::new(|_, _| no_addressing()),
            Box::new(|_, _| String::new()),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STX_ZERO_PAGE,
            "STX_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STX_ZERO_PAGE_Y,
            "STX_ZERO_PAGE_Y",
            Box::new(AddressingMode::zero_paged_y),
            Box::new(debug_zero_paged_y),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STX_ABSOLUTE,
            "STX_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STY_ZERO_PAGE,
            "STY_ZERO_PAGE",
            Box::new(AddressingMode::zero_paged),
            Box::new(debug_zero_paged),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STY_ZERO_PAGE_X,
            "STY_ZERO_PAGE_X",
            Box::new(AddressingMode::zero_paged_x),
            Box::new(debug_zero_paged_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::STY_ABSOLUTE,
            "STY_ABSOLUTE",
            Box::new(AddressingMode::absolute),
            Box::new(debug_absolute),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ISC_INDIRECT_X,
            "ISC_INDIRECT_X",
            Box::new(AddressingMode::indirect_x),
            Box::new(debug_indirect_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::ISC_ABSOLUTE_X,
            "ISC_ABSOLUTE_X",
            Box::new(AddressingMode::absolute_x),
            Box::new(debug_absolute_x),
        ),
        OpCodeDebug(
            nes::cpu::opcodes::SRE_INDIRECT_X,
            "SRE_INDIRECT_X",
            Box::new(|_, _| no_addressing()),
            Box::new(|cpu, memory| debug_indirect_x(cpu, memory)),
        ),
    ];
    return opcodes;
}
