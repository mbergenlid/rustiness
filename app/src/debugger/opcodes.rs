use std::string::String;
use nes::cpu::CPU;
use nes::memory::{Memory, CPUMemory, CPUMemoryReference};
use nes::addressing::AddressingMode;
use nes;

struct OpCodeDebug(u8, &'static str, &'static Fn(&mut CPU, &CPUMemory) -> String);

pub fn debug_instruction(op_code: u8, cpu: &CPU, memory: &CPUMemory) -> String {
    let mut cloned_cpu = cpu.clone();
    cloned_cpu.get_and_increment_pc();
    match OP_CODES.iter().find(|opd| opd.0 == op_code) {
        Some(&OpCodeDebug(_, name, args)) => format!("${:04x}: {} (0x{:x}): {}", cpu.program_counter(), name, op_code, args(&mut cloned_cpu, memory)),
        None => format!("Next instruction is unknown: {:x}", op_code),
    }
}

fn debug_immediate(addressing_mode: AddressingMode, memory: &CPUMemory) -> String {
    return format!("address {:?}", CPUMemoryReference(addressing_mode.operand_address, memory));
}

fn debug_indirect_y(cpu: &CPU, memory: &CPUMemory) -> String {
    let base_address = memory.get(cpu.program_counter()) as u16;
    let lsb: u16 = memory.get(base_address) as u16;
    let msb: u16 = memory.get(base_address+1) as u16;
    let indexed_address: u32 = ((msb << 8) | lsb) as u32;

    let operand_address = (indexed_address + cpu.register_y() as u32) as u16;

    return format!(
        "address {:?} (Base: 0x{:02x} -> {:02x},{:02x} ({:04x}))",
        CPUMemoryReference(operand_address, memory),
        base_address,
        lsb,
        msb,
        indexed_address
    );
}
fn debug_indirect_x(cpu: &CPU, memory: &CPUMemory) -> String {
    let index = memory.get(cpu.program_counter());
    let base_address = index.wrapping_add(cpu.register_x());
    let lsb: u16 = memory.get(base_address as u16) as u16;
    let msb: u16 = memory.get(base_address.wrapping_add(1) as u16) as u16;
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

const OP_CODES: [OpCodeDebug; 154] = [
    OpCodeDebug(nes::opcodes::ADC_IMMEDIATE         , "ADC_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::ADC_ZERO_PAGE         , "ADC_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ADC_ZERO_PAGE_X       , "ADC_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ADC_ABSOLUTE          , "ADC_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ADC_ABSOLUTE_X        , "ADC_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ADC_ABSOLUTE_Y        , "ADC_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ADC_INDIRECT_X        , "ADC_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::ADC_INDIRECT_Y        , "ADC_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_IMMEDIATE         , "AND_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::AND_ZERO_PAGE         , "AND_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_ZERO_PAGE_X       , "AND_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_ABSOLUTE          , "AND_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_ABSOLUTE_X        , "AND_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_ABSOLUTE_Y        , "AND_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::AND_INDIRECT_X        , "AND_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::AND_INDIRECT_Y        , "AND_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ASL_ACCUMULATOR       , "ASL_ACCUMULATOR", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::ASL_ZERO_PAGE         , "ASL_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ASL_ZERO_PAGE_X       , "ASL_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ASL_ABSOLUTE          , "ASL_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ASL_ABSOLUTE_X        , "ASL_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::BIT_ZERO_PAGE         , "BIT_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu ,memory), memory)),
    OpCodeDebug(nes::opcodes::BIT_ABSOLUTE          , "BIT_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu ,memory), memory)),
    OpCodeDebug(nes::opcodes::BRANCH_PLUS           , "BRANCH_PLUS", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_MINUS          , "BRANCH_MINUS", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_OVERFLOW_SET   , "BRANCH_OVERFLOW_SET", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_OVERFLOW_CLEAR , "BRANCH_OVERFLOW_CLEAR", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_CARRY_SET      , "BRANCH_CARRY_SET", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_CARRY_CLEAR    , "BRANCH_CARRY_CLEAR", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_NOT_EQUAL      , "BRANCH_NOT_EQUAL", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRANCH_EQUAL          , "BRANCH_EQUAL", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::BRK                   , "BRK_IMPLIED", &|  _,      memory| format!("0x{:x}{:x}", memory.get(0xFFFF), memory.get(0xFFFE))),
    OpCodeDebug(nes::opcodes::CMP_IMMEDIATE         , "CMP_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::CMP_ZERO_PAGE         , "CMP_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CMP_ZERO_PAGE_X       , "CMP_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CMP_ABSOLUTE          , "CMP_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CMP_ABSOLUTE_X        , "CMP_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CMP_ABSOLUTE_Y        , "CMP_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CMP_INDIRECT_X        , "CMP_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::CMP_INDIRECT_Y        , "CMP_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CPX_IMMEDIATE         , "CPX_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::CPX_ZERO_PAGE         , "CPX_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CPX_ABSOLUTE          , "CPX_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CPY_IMMEDIATE         , "CPY_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::CPY_ZERO_PAGE         , "CPY_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CPY_ABSOLUTE          , "CPY_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::DEC_ZERO_PAGE         , "DEC_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::DEC_ZERO_PAGE_X       , "DEC_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::DEC_ABSOLUTE          , "DEC_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::DEC_ABSOLUTE_X        , "DEC_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_IMMEDIATE         , "EOR_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::EOR_ZERO_PAGE         , "EOR_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_ZERO_PAGE_X       , "EOR_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_ABSOLUTE          , "EOR_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_ABSOLUTE_X        , "EOR_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_ABSOLUTE_Y        , "EOR_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::EOR_INDIRECT_X        , "EOR_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::EOR_INDIRECT_Y        , "EOR_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::CLC                   , "CLC", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::SEC                   , "SEC", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::CLI                   , "CLI", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::SEI                   , "SEI", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::CLV                   , "CLV", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::CLD                   , "CLD", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::SED                   , "SED", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::INC_ZERO_PAGE         , "INC_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::INC_ZERO_PAGE_X       , "INC_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::INC_ABSOLUTE          , "INC_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::INC_ABSOLUTE_X        , "INC_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::JMP_ABSOLUTE          , "JMP_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::JMP_INDIRECT          , "JMP_INDIRECT", &|cpu, memory| debug_immediate(AddressingMode::indirect(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::JSR_ABSOLUTE          , "JSR_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_IMMEDIATE         , "LDA_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::LDA_ZERO_PAGE         , "LDA_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_ZERO_PAGE_X       , "LDA_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_ABSOLUTE          , "LDA_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_ABSOLUTE_X        , "LDA_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_ABSOLUTE_Y        , "LDA_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDA_INDIRECT_X        , "LDA_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::LDA_INDIRECT_Y        , "LDA_INDIRECT_Y", &|cpu, memory| debug_indirect_y(cpu, memory)),
    OpCodeDebug(nes::opcodes::LDX_IMMEDIATE         , "LDX_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::LDX_ZERO_PAGE         , "LDX_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDX_ZERO_PAGE_Y       , "LDX_ZERO_PAGE_Y", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDX_ABSOLUTE          , "LDX_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDX_ABSOLUTE_Y        , "LDX_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDY_IMMEDIATE         , "LDY_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::LDY_ZERO_PAGE         , "LDY_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDY_ZERO_PAGE_X       , "LDY_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDY_ABSOLUTE          , "LDY_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LDY_ABSOLUTE_X        , "LDY_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LSR_ACCUMULATOR       , "LSR_ACCUMULATOR", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::LSR_ZERO_PAGE         , "LSR_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LSR_ZERO_PAGE_X       , "LSR_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LSR_ABSOLUTE          , "LSR_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::LSR_ABSOLUTE_X        , "LSR_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::NOP_IMPLIED           , "NOP_IMPLIED", &|  _,      _| String::new()),
    OpCodeDebug(nes::opcodes::ORA_IMMEDIATE         , "ORA_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::ORA_ZERO_PAGE         , "ORA_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ORA_ZERO_PAGE_X       , "ORA_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ORA_ABSOLUTE          , "ORA_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ORA_ABSOLUTE_X        , "ORA_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ORA_ABSOLUTE_Y        , "ORA_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ORA_INDIRECT_X        , "ORA_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::ORA_INDIRECT_Y        , "ORA_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::TAX                   , "TAX", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::TXA                   , "TXA", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::DEX                   , "DEX", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::INX                   , "INX", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::TAY                   , "TAY", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::TYA                   , "TYA", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::DEY                   , "DEY", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::INY                   , "INY", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::ROL_ACCUMULATOR       , "ROL_ACCUMULATOR", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::ROL_ZERO_PAGE         , "ROL_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROL_ZERO_PAGE_X       , "ROL_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROL_ABSOLUTE          , "ROL_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROL_ABSOLUTE_X        , "ROL_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROR_ACCUMULATOR       , "ROR_ACCUMULATOR", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::ROR_ZERO_PAGE         , "ROR_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROR_ZERO_PAGE_X       , "ROR_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROR_ABSOLUTE          , "ROR_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ROR_ABSOLUTE_X        , "ROR_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::RTI                   , "RTI_IMPLIED", &|_  ,      _| String::new()),
    OpCodeDebug(nes::opcodes::RTS                   , "RTS", &|_, _| String::new()),
    OpCodeDebug(nes::opcodes::SBC_IMMEDIATE         , "SBC_IMMEDIATE", &|cpu, memory| debug_immediate(AddressingMode::immediate(cpu), memory)),
    OpCodeDebug(nes::opcodes::SBC_ZERO_PAGE         , "SBC_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SBC_ZERO_PAGE_X       , "SBC_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SBC_ABSOLUTE          , "SBC_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SBC_ABSOLUTE_X        , "SBC_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SBC_ABSOLUTE_Y        , "SBC_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SBC_INDIRECT_X        , "SBC_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::SBC_INDIRECT_Y        , "SBC_INDIRECT_Y", &|cpu, memory| debug_immediate(AddressingMode::indirect_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_ZERO_PAGE         , "STA_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_ZERO_PAGE_X       , "STA_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_ABSOLUTE          , "STA_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_ABSOLUTE_X        , "STA_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_ABSOLUTE_Y        , "STA_ABSOLUTE_Y", &|cpu, memory| debug_immediate(AddressingMode::absolute_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STA_INDIRECT_X        , "STA_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::STA_INDIRECT_Y        , "STA_INDIRECT_Y", &|cpu, memory| debug_indirect_y(cpu, memory)),
    OpCodeDebug(nes::opcodes::TXS                   , "TXS", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::TSX                   , "TSX", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::PHA                   , "PHA", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::PLA                   , "PLA", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::PHP                   , "PHP", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::PLP                   , "PLP", &|_,      _| String::new()),
    OpCodeDebug(nes::opcodes::STX_ZERO_PAGE         , "STX_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STX_ZERO_PAGE_Y       , "STX_ZERO_PAGE_Y", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_y(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STX_ABSOLUTE          , "STX_ABSOLUTE",  &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STY_ZERO_PAGE         , "STY_ZERO_PAGE", &|cpu, memory| debug_immediate(AddressingMode::zero_paged(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STY_ZERO_PAGE_X       , "STY_ZERO_PAGE_X", &|cpu, memory| debug_immediate(AddressingMode::zero_paged_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::STY_ABSOLUTE          , "STY_ABSOLUTE", &|cpu, memory| debug_immediate(AddressingMode::absolute(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::ISC_INDIRECT_X        , "ISC_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
    OpCodeDebug(nes::opcodes::ISC_ABSOLUTE_X        , "ISC_ABSOLUTE_X", &|cpu, memory| debug_immediate(AddressingMode::absolute_x(cpu, memory), memory)),
    OpCodeDebug(nes::opcodes::SRE_INDIRECT_X        , "SRE_INDIRECT_X", &|cpu, memory| debug_indirect_x(cpu, memory)),
];
