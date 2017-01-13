use instructions;
use addressing::AddressingMode;
use cpu::CPU;
use cpu;
use memory::Memory;

pub fn execute_instruction(cpu: &mut CPU, memory: &mut Memory) {
    let op_code: u8 = memory.get(cpu.get_and_increment_pc());

    match op_code {
        ADC_IMMEDIATE       => instructions::adc(AddressingMode::immediate(cpu), cpu, memory),
        ADC_ZERO_PAGE       => instructions::adc(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ADC_ZERO_PAGE_X     => instructions::adc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ADC_ABSOLUTE        => instructions::adc(AddressingMode::absolute(cpu, memory), cpu, memory),
        ADC_ABSOLUTE_X      => instructions::adc(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        ADC_ABSOLUTE_Y      => instructions::adc(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        ADC_INDIRECT_X      => instructions::adc(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        ADC_INDIRECT_Y      => instructions::adc(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        AND_IMMEDIATE       => instructions::and(AddressingMode::immediate(cpu), cpu, memory),
        AND_ZERO_PAGE       => instructions::and(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        AND_ZERO_PAGE_X     => instructions::and(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        AND_ABSOLUTE        => instructions::and(AddressingMode::absolute(cpu, memory), cpu, memory),
        AND_ABSOLUTE_X      => instructions::and(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        AND_ABSOLUTE_Y      => instructions::and(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        AND_INDIRECT_X      => instructions::and(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        AND_INDIRECT_Y      => instructions::and(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        ASL_ACCUMULATOR     => instructions::asl_accumulator(cpu),
        ASL_ZERO_PAGE       => instructions::asl(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ASL_ZERO_PAGE_X     => instructions::asl(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ASL_ABSOLUTE        => instructions::asl(AddressingMode::absolute(cpu, memory), cpu, memory),
        ASL_ABSOLUTE_X      => instructions::asl(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        BIT_ZERO_PAGE       => println!("asd"),
        BIT_ABSOLUTE        => println!("asd"),
        BRANCH_PLUS           => instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, true),
        BRANCH_MINUS          => instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, false),
        BRANCH_OVERFLOW_SET   => instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, false),
        BRANCH_OVERFLOW_CLEAR => instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, true),
        BRANCH_CARRY_SET      => instructions::branch(cpu, memory, cpu::CARRY_FLAG, false),
        BRANCH_CARRY_CLEAR    => instructions::branch(cpu, memory, cpu::CARRY_FLAG, true),
        BRANCH_NOT_EQUAL      => instructions::branch(cpu, memory, cpu::ZERO_FLAG, true),
        BRANCH_EQUAL          => instructions::branch(cpu, memory, cpu::ZERO_FLAG, false),
        BRK_IMPLIED         => println!("asd"),
        CMP_IMMEDIATE       => println!("asd"),
        CMP_ZERO_PAGE       => println!("asd"),
        CMP_ZERO_PAGE_X     => println!("asd"),
        CMP_ABSOLUTE        => println!("asd"),
        CMP_ABSOLUTE_X      => println!("asd"),
        CMP_ABSOLUTE_Y      => println!("asd"),
        CMP_INDIRECT_X      => println!("asd"),
        CMP_INDIRECT_Y      => println!("asd"),
        CPX_IMMEDIATE       => println!("asd"),
        CPX_ZERO_PAGE       => println!("asd"),
        CPX_ABSOLUTE        => println!("asd"),
        CPY_IMMEDIATE       => println!("asd"),
        CPY_ZERO_PAGE       => println!("asd"),
        CPY_ABSOLUTE        => println!("asd"),
        DEC_ZERO_PAGE       => println!("asd"),
        DEC_ZERO_PAGE_X     => println!("asd"),
        DEC_ABSOLUTE        => println!("asd"),
        DEC_ABSOLUTE_X      => println!("asd"),
        EOR_IMMEDIATE       => println!("asd"),
        EOR_ZERO_PAGE       => println!("asd"),
        EOR_ZERO_PAGE_X     => println!("asd"),
        EOR_ABSOLUTE        => println!("asd"),
        EOR_ABSOLUTE_X      => println!("asd"),
        EOR_ABSOLUTE_Y      => println!("asd"),
        EOR_INDIRECT_X      => println!("asd"),
        EOR_INDIRECT_Y      => println!("asd"),
        CLC                 => println!("hej"),
        SEC                 => println!("hej"),
        CLI                 => println!("hej"),
        SEI                 => println!("hej"),
        CLV                 => println!("hej"),
        CLD                 => println!("hej"),
        SED                 => println!("hej"),
        INC_ZERO_PAGE       => println!("hej"),
        INC_ZERO_PAGE_X     => println!("hej"),
        INC_ABSOLUTE        => println!("hej"),
        INC_ABSOLUTE_X      => println!("hej"),
        JMP_ABSOLUTE        => println!("hej"),
        JMP_INDIRECT        => println!("hej"),
        JSR_ABSOLUTE        => println!("hej"),
        LDA_IMMEDIATE       => println!("hej"),
        LDA_ZERO_PAGE       => println!("hej"),
        LDA_ZERO_PAGE_X     => println!("hej"),
        LDA_ABSOLUTE        => println!("hej"),
        LDA_ABSOLUTE_X      => println!("hej"),
        LDA_ABSOLUTE_Y      => println!("hej"),
        LDA_INDIRECT_X      => println!("hej"),
        LDA_INDIRECT_Y      => println!("hej"),
        LDX_IMMEDIATE       => println!("hej"),
        LDX_ZERO_PAGE       => println!("hej"),
        LDX_ZERO_PAGE_Y     => println!("hej"),
        LDX_ABSOLUTE        => println!("hej"),
        LDX_ABSOLUTE_Y      => println!("hej"),
        LDY_IMMEDIATE       => println!("hej"),
        LDY_ZERO_PAGE       => println!("hej"),
        LDY_ZERO_PAGE_X     => println!("hej"),
        LDY_ABSOLUTE        => println!("hej"),
        LDY_ABSOLUTE_X      => println!("hej"),
        LSR_ACCUMULATOR     => println!("hej"),
        LSR_ZERO_PAGE       => println!("hej"),
        LSR_ZERO_PAGE_X     => println!("hej"),
        LSR_ABSOLUTE        => println!("hej"),
        LSR_ABSOLUTE_X      => println!("hej"),
        NOP_IMPLIED         => println!("hej"),
        ORA_IMMEDIATE       => instructions::or(AddressingMode::immediate(cpu), cpu, memory),
        ORA_ZERO_PAGE       => instructions::or(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ORA_ZERO_PAGE_X     => instructions::or(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ORA_ABSOLUTE        => instructions::or(AddressingMode::absolute(cpu, memory), cpu, memory),
        ORA_ABSOLUTE_X      => instructions::or(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        ORA_ABSOLUTE_Y      => instructions::or(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        ORA_INDIRECT_X      => instructions::or(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        ORA_INDIRECT_Y      => instructions::or(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        TAX                 => println!("hej"),
        TXA                 => println!("hej"),
        DEX                 => println!("hej"),
        INX                 => println!("hej"),
        TAY                 => println!("hej"),
        TYA                 => println!("hej"),
        DEY                 => println!("hej"),
        INY                 => println!("hej"),
        ROL_ACCUMULATOR     => println!("hej"),
        ROL_ZERO_PAGE       => println!("hej"),
        ROL_ZERO_PAGE_X     => println!("hej"),
        ROL_ABSOLUTE        => println!("hej"),
        ROL_ABSOLUTE_X      => println!("hej"),
        ROR_ACCUMULATOR     => println!("hej"),
        ROR_ZERO_PAGE       => println!("hej"),
        ROR_ZERO_PAGE_X     => println!("hej"),
        ROR_ABSOLUTE        => println!("hej"),
        ROR_ABSOLUTE_X      => println!("hej"),
        RTI_IMPLIED         => println!("hej"),
        RTS_IMPLIED         => println!("hej"),
        SBC_IMMEDIATE       => println!("hej"),
        SBC_ZERO_PAGE       => println!("hej"),
        SBC_ZERO_PAGE_X     => println!("hej"),
        SBC_ABSOLUTE        => println!("hej"),
        SBC_ABSOLUTE_X      => println!("hej"),
        SBC_ABSOLUTE_Y      => println!("hej"),
        SBC_INDIRECT_X      => println!("hej"),
        SBC_INDIRECT_Y      => println!("hej"),
        STA_ZERO_PAGE       => println!("hej"),
        STA_ZERO_PAGE_X     => println!("hej"),
        STA_ABSOLUTE        => println!("hej"),
        STA_ABSOLUTE_X      => println!("hej"),
        STA_ABSOLUTE_Y      => println!("hej"),
        STA_INDIRECT_X      => println!("hej"),
        STA_INDIRECT_Y      => println!("hej"),
        TXS                 => println!("hej"),
        TSX                 => println!("hej"),
        PHA                 => println!("hej"),
        PLA                 => println!("hej"),
        PHP                 => println!("hej"),
        PLP                 => println!("hej"),
        STX_ZERO_PAGE       => println!("hej"),
        STX_ZERO_PAGE_Y     => println!("hej"),
        STX_ABSOLUTE        => println!("hej"),
        STY_ZERO_PAGE       => println!("hej"),
        STY_ZERO_PAGE_X     => println!("hej"),
        STY_ABSOLUTE        => println!("hej"),
        _                   => panic!("Unknown OpCode"),
    }

}

pub type OpCode = u8;

pub const ADC_IMMEDIATE: OpCode = 0x69;
pub const ADC_ZERO_PAGE: OpCode = 0x65;
pub const ADC_ZERO_PAGE_X: OpCode = 0x75;
pub const ADC_ABSOLUTE: OpCode = 0x6D;
pub const ADC_ABSOLUTE_X: OpCode = 0x7D;
pub const ADC_ABSOLUTE_Y: OpCode = 0x79;
pub const ADC_INDIRECT_X: OpCode = 0x61;
pub const ADC_INDIRECT_Y: OpCode = 0x71;
pub const AND_IMMEDIATE: OpCode = 0x29;
pub const AND_ZERO_PAGE: OpCode = 0x25;
pub const AND_ZERO_PAGE_X: OpCode = 0x35;
pub const AND_ABSOLUTE: OpCode = 0x2D;
pub const AND_ABSOLUTE_X: OpCode = 0x3D;
pub const AND_ABSOLUTE_Y: OpCode = 0x39;
pub const AND_INDIRECT_X: OpCode = 0x21;
pub const AND_INDIRECT_Y: OpCode = 0x31;
pub const ASL_ACCUMULATOR: OpCode = 0x0A;
pub const ASL_ZERO_PAGE: OpCode = 0x06;
pub const ASL_ZERO_PAGE_X: OpCode = 0x16;
pub const ASL_ABSOLUTE: OpCode = 0x0E;
pub const ASL_ABSOLUTE_X: OpCode = 0x1E;
pub const BIT_ZERO_PAGE: OpCode = 0x24;
pub const BIT_ABSOLUTE: OpCode = 0x2C;
pub const BRANCH_PLUS: OpCode           = 0x10;
pub const BRANCH_MINUS: OpCode          = 0x30;
pub const BRANCH_OVERFLOW_SET: OpCode   = 0x70;
pub const BRANCH_OVERFLOW_CLEAR: OpCode = 0x50;
pub const BRANCH_CARRY_SET: OpCode      = 0xB0;
pub const BRANCH_CARRY_CLEAR: OpCode    = 0x90;
pub const BRANCH_NOT_EQUAL: OpCode      = 0xD0;
pub const BRANCH_EQUAL: OpCode          = 0xF0;
pub const BRK_IMPLIED: OpCode = 0x00;
pub const CMP_IMMEDIATE: OpCode = 0xC9;
pub const CMP_ZERO_PAGE: OpCode = 0xC5;
pub const CMP_ZERO_PAGE_X: OpCode = 0xD5;
pub const CMP_ABSOLUTE: OpCode = 0xCD;
pub const CMP_ABSOLUTE_X: OpCode = 0xDD;
pub const CMP_ABSOLUTE_Y: OpCode = 0xD9;
pub const CMP_INDIRECT_X: OpCode = 0xC1;
pub const CMP_INDIRECT_Y: OpCode = 0xD1;
pub const CPX_IMMEDIATE: OpCode = 0xE0;
pub const CPX_ZERO_PAGE: OpCode = 0xE4;
pub const CPX_ABSOLUTE: OpCode = 0xEC;
pub const CPY_IMMEDIATE: OpCode = 0xC0;
pub const CPY_ZERO_PAGE: OpCode = 0xC4;
pub const CPY_ABSOLUTE: OpCode = 0xCC;
pub const DEC_ZERO_PAGE: OpCode = 0xC6;
pub const DEC_ZERO_PAGE_X: OpCode = 0xD6;
pub const DEC_ABSOLUTE: OpCode = 0xCE;
pub const DEC_ABSOLUTE_X: OpCode = 0xDE;
pub const EOR_IMMEDIATE: OpCode = 0x49;
pub const EOR_ZERO_PAGE: OpCode = 0x45;
pub const EOR_ZERO_PAGE_X: OpCode = 0x55;
pub const EOR_ABSOLUTE: OpCode = 0x4D;
pub const EOR_ABSOLUTE_X: OpCode = 0x5D;
pub const EOR_ABSOLUTE_Y: OpCode = 0x59;
pub const EOR_INDIRECT_X: OpCode = 0x41;
pub const EOR_INDIRECT_Y: OpCode = 0x51;
pub const CLC            : OpCode = 0x18;
pub const SEC            : OpCode = 0x38;
pub const CLI            : OpCode = 0x58;
pub const SEI            : OpCode = 0x78;
pub const CLV            : OpCode = 0xB8;
pub const CLD            : OpCode = 0xD8;
pub const SED            : OpCode = 0xF8;
pub const INC_ZERO_PAGE: OpCode = 0xE6;
pub const INC_ZERO_PAGE_X: OpCode = 0xF6;
pub const INC_ABSOLUTE: OpCode = 0xEE;
pub const INC_ABSOLUTE_X: OpCode = 0xFE;
pub const JMP_ABSOLUTE: OpCode = 0x4C;
pub const JMP_INDIRECT: OpCode = 0x6C;
pub const JSR_ABSOLUTE: OpCode = 0x20;
pub const LDA_IMMEDIATE: OpCode = 0xA9;
pub const LDA_ZERO_PAGE: OpCode = 0xA5;
pub const LDA_ZERO_PAGE_X: OpCode = 0xB5;
pub const LDA_ABSOLUTE: OpCode = 0xAD;
pub const LDA_ABSOLUTE_X: OpCode = 0xBD;
pub const LDA_ABSOLUTE_Y: OpCode = 0xB9;
pub const LDA_INDIRECT_X: OpCode = 0xA1;
pub const LDA_INDIRECT_Y: OpCode = 0xB1;
pub const LDX_IMMEDIATE: OpCode = 0xA2;
pub const LDX_ZERO_PAGE: OpCode = 0xA6;
pub const LDX_ZERO_PAGE_Y: OpCode = 0xB6;
pub const LDX_ABSOLUTE: OpCode = 0xAE;
pub const LDX_ABSOLUTE_Y: OpCode = 0xBE;
pub const LDY_IMMEDIATE: OpCode = 0xA0;
pub const LDY_ZERO_PAGE: OpCode = 0xA4;
pub const LDY_ZERO_PAGE_X: OpCode = 0xB4;
pub const LDY_ABSOLUTE: OpCode = 0xAC;
pub const LDY_ABSOLUTE_X: OpCode = 0xBC;
pub const LSR_ACCUMULATOR: OpCode = 0x4A;
pub const LSR_ZERO_PAGE: OpCode = 0x46;
pub const LSR_ZERO_PAGE_X: OpCode = 0x56;
pub const LSR_ABSOLUTE: OpCode = 0x4E;
pub const LSR_ABSOLUTE_X: OpCode = 0x5E;
pub const NOP_IMPLIED: OpCode = 0xEA;
pub const ORA_IMMEDIATE: OpCode = 0x09;
pub const ORA_ZERO_PAGE: OpCode = 0x05;
pub const ORA_ZERO_PAGE_X: OpCode = 0x15;
pub const ORA_ABSOLUTE: OpCode = 0x0D;
pub const ORA_ABSOLUTE_X: OpCode = 0x1D;
pub const ORA_ABSOLUTE_Y: OpCode = 0x19;
pub const ORA_INDIRECT_X: OpCode = 0x01;
pub const ORA_INDIRECT_Y: OpCode = 0x11;
pub const TAX            : OpCode = 0xAA;
pub const TXA            : OpCode = 0x8A;
pub const DEX            : OpCode = 0xCA;
pub const INX            : OpCode = 0xE8;
pub const TAY            : OpCode = 0xA8;
pub const TYA            : OpCode = 0x98;
pub const DEY            : OpCode = 0x88;
pub const INY            : OpCode = 0xC8;
pub const ROL_ACCUMULATOR: OpCode = 0x2A;
pub const ROL_ZERO_PAGE: OpCode = 0x26;
pub const ROL_ZERO_PAGE_X: OpCode = 0x36;
pub const ROL_ABSOLUTE: OpCode = 0x2E;
pub const ROL_ABSOLUTE_X: OpCode = 0x3E;
pub const ROR_ACCUMULATOR: OpCode = 0x6A;
pub const ROR_ZERO_PAGE: OpCode = 0x66;
pub const ROR_ZERO_PAGE_X: OpCode = 0x76;
pub const ROR_ABSOLUTE: OpCode = 0x6E;
pub const ROR_ABSOLUTE_X: OpCode = 0x7E;
pub const RTI_IMPLIED: OpCode = 0x40;
pub const RTS_IMPLIED: OpCode = 0x60;
pub const SBC_IMMEDIATE: OpCode = 0xE9;
pub const SBC_ZERO_PAGE: OpCode = 0xE5;
pub const SBC_ZERO_PAGE_X: OpCode = 0xF5;
pub const SBC_ABSOLUTE: OpCode = 0xED;
pub const SBC_ABSOLUTE_X: OpCode = 0xFD;
pub const SBC_ABSOLUTE_Y: OpCode = 0xF9;
pub const SBC_INDIRECT_X: OpCode = 0xE1;
pub const SBC_INDIRECT_Y: OpCode = 0xF1;
pub const STA_ZERO_PAGE: OpCode = 0x85;
pub const STA_ZERO_PAGE_X: OpCode = 0x95;
pub const STA_ABSOLUTE: OpCode = 0x8D;
pub const STA_ABSOLUTE_X: OpCode = 0x9D;
pub const STA_ABSOLUTE_Y: OpCode = 0x99;
pub const STA_INDIRECT_X: OpCode = 0x81;
pub const STA_INDIRECT_Y: OpCode = 0x91;
pub const TXS            : OpCode = 0x9A;
pub const TSX            : OpCode = 0xBA;
pub const PHA            : OpCode = 0x48;
pub const PLA            : OpCode = 0x68;
pub const PHP            : OpCode = 0x08;
pub const PLP            : OpCode = 0x28;
pub const STX_ZERO_PAGE: OpCode = 0x86;
pub const STX_ZERO_PAGE_Y: OpCode = 0x96;
pub const STX_ABSOLUTE: OpCode = 0x8E;
pub const STY_ZERO_PAGE: OpCode = 0x84;
pub const STY_ZERO_PAGE_X: OpCode = 0x94;
pub const STY_ABSOLUTE: OpCode = 0x8C;