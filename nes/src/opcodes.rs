use instructions;
use addressing::AddressingMode;
use cpu::CPU;
use cpu;
use memory::Memory;

pub fn execute_instruction(cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let op_code: u8 = memory.get(cpu.get_and_increment_pc());

    match op_code {
        ADC_IMMEDIATE         => instructions::adc(AddressingMode::immediate(cpu), cpu, memory),
        ADC_ZERO_PAGE         => instructions::adc(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ADC_ZERO_PAGE_X       => instructions::adc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ADC_ABSOLUTE          => instructions::adc(AddressingMode::absolute(cpu, memory), cpu, memory),
        ADC_ABSOLUTE_X        => instructions::adc(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        ADC_ABSOLUTE_Y        => instructions::adc(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        ADC_INDIRECT_X        => instructions::adc(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        ADC_INDIRECT_Y        => instructions::adc(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        AND_IMMEDIATE         => instructions::and(AddressingMode::immediate(cpu), cpu, memory),
        AND_ZERO_PAGE         => instructions::and(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        AND_ZERO_PAGE_X       => instructions::and(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        AND_ABSOLUTE          => instructions::and(AddressingMode::absolute(cpu, memory), cpu, memory),
        AND_ABSOLUTE_X        => instructions::and(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        AND_ABSOLUTE_Y        => instructions::and(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        AND_INDIRECT_X        => instructions::and(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        AND_INDIRECT_Y        => instructions::and(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        ASL_ACCUMULATOR       => instructions::asl_accumulator(cpu),
        ASL_ZERO_PAGE         => instructions::asl(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ASL_ZERO_PAGE_X       => instructions::asl(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ASL_ABSOLUTE          => instructions::asl(AddressingMode::absolute(cpu, memory), cpu, memory),
        ASL_ABSOLUTE_X        => {instructions::asl(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        BIT_ZERO_PAGE         => instructions::bit(AddressingMode::zero_paged(cpu ,memory), cpu, memory),
        BIT_ABSOLUTE          => instructions::bit(AddressingMode::absolute(cpu ,memory), cpu, memory),
        BRANCH_PLUS           => instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, true),
        BRANCH_MINUS          => instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, false),
        BRANCH_OVERFLOW_SET   => instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, false),
        BRANCH_OVERFLOW_CLEAR => instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, true),
        BRANCH_CARRY_SET      => instructions::branch(cpu, memory, cpu::CARRY_FLAG, false),
        BRANCH_CARRY_CLEAR    => instructions::branch(cpu, memory, cpu::CARRY_FLAG, true),
        BRANCH_NOT_EQUAL      => instructions::branch(cpu, memory, cpu::ZERO_FLAG, true),
        BRANCH_EQUAL          => instructions::branch(cpu, memory, cpu::ZERO_FLAG, false),
        BRK_IMPLIED           => panic!("BRK instruction is not implemented"),
        CMP_IMMEDIATE         => instructions::cmp(AddressingMode::immediate(cpu), cpu, memory),
        CMP_ZERO_PAGE         => instructions::cmp(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        CMP_ZERO_PAGE_X       => instructions::cmp(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        CMP_ABSOLUTE          => instructions::cmp(AddressingMode::absolute(cpu, memory), cpu, memory),
        CMP_ABSOLUTE_X        => instructions::cmp(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        CMP_ABSOLUTE_Y        => instructions::cmp(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        CMP_INDIRECT_X        => instructions::cmp(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        CMP_INDIRECT_Y        => instructions::cmp(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        CPX_IMMEDIATE         => instructions::cpx(AddressingMode::immediate(cpu), cpu, memory),
        CPX_ZERO_PAGE         => instructions::cpx(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        CPX_ABSOLUTE          => instructions::cpx(AddressingMode::absolute(cpu, memory), cpu, memory),
        CPY_IMMEDIATE         => instructions::cpy(AddressingMode::immediate(cpu), cpu, memory),
        CPY_ZERO_PAGE         => instructions::cpy(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        CPY_ABSOLUTE          => instructions::cpy(AddressingMode::absolute(cpu, memory), cpu, memory),
        DEC_ZERO_PAGE         => instructions::dec(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        DEC_ZERO_PAGE_X       => instructions::dec(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        DEC_ABSOLUTE          => instructions::dec(AddressingMode::absolute(cpu, memory), cpu, memory),
        DEC_ABSOLUTE_X        => {instructions::dec(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        EOR_IMMEDIATE         => instructions::eor(AddressingMode::immediate(cpu), cpu, memory),
        EOR_ZERO_PAGE         => instructions::eor(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        EOR_ZERO_PAGE_X       => instructions::eor(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        EOR_ABSOLUTE          => instructions::eor(AddressingMode::absolute(cpu, memory), cpu, memory),
        EOR_ABSOLUTE_X        => instructions::eor(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        EOR_ABSOLUTE_Y        => instructions::eor(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        EOR_INDIRECT_X        => instructions::eor(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        EOR_INDIRECT_Y        => instructions::eor(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        CLC                   => { cpu.clear_flags(cpu::CARRY_FLAG); 2},
        SEC                   => { cpu.set_flags(cpu::CARRY_FLAG); 2},
        CLI                   => {cpu.clear_flags(cpu::INTERRUPT_DISABLE_FLAG); 2},
        SEI                   => { cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG); 2},
        CLV                   => {cpu.clear_flags(cpu::OVERFLOW_FLAG); 2},
        CLD                   => {cpu.clear_flags(cpu::DECIMAL_FLAG); 2},
        SED                   => { cpu.set_flags(cpu::DECIMAL_FLAG); 2},
        INC_ZERO_PAGE         => instructions::inc(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        INC_ZERO_PAGE_X       => instructions::inc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        INC_ABSOLUTE          => instructions::inc(AddressingMode::absolute(cpu, memory), cpu, memory),
        INC_ABSOLUTE_X        => {instructions::inc(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        JMP_ABSOLUTE          => {instructions::jmp(AddressingMode::absolute(cpu, memory), cpu); 3 },
        JMP_INDIRECT          => {instructions::jmp(AddressingMode::indirect(cpu, memory), cpu); 5 },
        JSR_ABSOLUTE          => instructions::jsr(cpu, memory),
        LDA_IMMEDIATE         => instructions::lda(AddressingMode::immediate(cpu), cpu, memory),
        LDA_ZERO_PAGE         => instructions::lda(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        LDA_ZERO_PAGE_X       => instructions::lda(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        LDA_ABSOLUTE          => instructions::lda(AddressingMode::absolute(cpu, memory), cpu, memory),
        LDA_ABSOLUTE_X        => instructions::lda(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        LDA_ABSOLUTE_Y        => instructions::lda(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        LDA_INDIRECT_X        => instructions::lda(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        LDA_INDIRECT_Y        => instructions::lda(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        LDX_IMMEDIATE         => instructions::ldx(AddressingMode::immediate(cpu), cpu, memory),
        LDX_ZERO_PAGE         => instructions::ldx(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        LDX_ZERO_PAGE_Y       => instructions::ldx(AddressingMode::zero_paged_y(cpu, memory), cpu, memory),
        LDX_ABSOLUTE          => instructions::ldx(AddressingMode::absolute(cpu, memory), cpu, memory),
        LDX_ABSOLUTE_Y        => instructions::ldx(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        LDY_IMMEDIATE         => instructions::ldy(AddressingMode::immediate(cpu), cpu, memory),
        LDY_ZERO_PAGE         => instructions::ldy(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        LDY_ZERO_PAGE_X       => instructions::ldy(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        LDY_ABSOLUTE          => instructions::ldy(AddressingMode::absolute(cpu, memory), cpu, memory),
        LDY_ABSOLUTE_X        => instructions::ldy(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        LSR_ACCUMULATOR       => {cpu.logical_shift_right_accumulator(); 2},
        LSR_ZERO_PAGE         => instructions::lsr(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        LSR_ZERO_PAGE_X       => instructions::lsr(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        LSR_ABSOLUTE          => instructions::lsr(AddressingMode::absolute(cpu, memory), cpu, memory),
        LSR_ABSOLUTE_X        => {instructions::lsr(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        NOP_IMPLIED           => 2,
        ORA_IMMEDIATE         => instructions::or(AddressingMode::immediate(cpu), cpu, memory),
        ORA_ZERO_PAGE         => instructions::or(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ORA_ZERO_PAGE_X       => instructions::or(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ORA_ABSOLUTE          => instructions::or(AddressingMode::absolute(cpu, memory), cpu, memory),
        ORA_ABSOLUTE_X        => instructions::or(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        ORA_ABSOLUTE_Y        => instructions::or(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        ORA_INDIRECT_X        => instructions::or(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        ORA_INDIRECT_Y        => instructions::or(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        TAX                   => { let acc = cpu.accumulator(); cpu.load_x(acc); 2},
        TXA                   => { let temp = cpu.register_x(); cpu.load_accumulator(temp); 2},
        DEX                   => { cpu.decrement_x(); 2 },
        INX                   => { cpu.increment_x(); 2 },
        TAY                   => { let temp = cpu.accumulator(); cpu.load_y(temp); 2},
        TYA                   => { let temp = cpu.register_y(); cpu.load_accumulator(temp); 2},
        DEY                   => { cpu.decrement_y(); 2 },
        INY                   => { cpu.increment_y(); 2 },
        ROL_ACCUMULATOR       => {cpu.rotate_accumulator_left(); 2},
        ROL_ZERO_PAGE         => instructions::rol(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ROL_ZERO_PAGE_X       => instructions::rol(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ROL_ABSOLUTE          => instructions::rol(AddressingMode::absolute(cpu, memory), cpu, memory),
        ROL_ABSOLUTE_X        => {instructions::rol(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        ROR_ACCUMULATOR       => {cpu.rotate_accumulator_right(); 2},
        ROR_ZERO_PAGE         => instructions::ror(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        ROR_ZERO_PAGE_X       => instructions::ror(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        ROR_ABSOLUTE          => instructions::ror(AddressingMode::absolute(cpu, memory), cpu, memory),
        ROR_ABSOLUTE_X        => {instructions::ror(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7},
        RTI_IMPLIED           => panic!("RTI instruction is not implemented"),
        RTS                   => instructions::rts(cpu, memory),
        SBC_IMMEDIATE         => instructions::sbc(AddressingMode::immediate(cpu), cpu, memory),
        SBC_ZERO_PAGE         => instructions::sbc(AddressingMode::zero_paged(cpu, memory), cpu, memory),
        SBC_ZERO_PAGE_X       => instructions::sbc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory),
        SBC_ABSOLUTE          => instructions::sbc(AddressingMode::absolute(cpu, memory), cpu, memory),
        SBC_ABSOLUTE_X        => instructions::sbc(AddressingMode::absolute_x(cpu, memory), cpu, memory),
        SBC_ABSOLUTE_Y        => instructions::sbc(AddressingMode::absolute_y(cpu, memory), cpu, memory),
        SBC_INDIRECT_X        => instructions::sbc(AddressingMode::indirect_x(cpu, memory), cpu, memory),
        SBC_INDIRECT_Y        => instructions::sbc(AddressingMode::indirect_y(cpu, memory), cpu, memory),
        STA_ZERO_PAGE         => {instructions::sta(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3},
        STA_ZERO_PAGE_X       => {instructions::sta(AddressingMode::zero_paged_x(cpu, memory), cpu, memory); 4},
        STA_ABSOLUTE          => {instructions::sta(AddressingMode::absolute(cpu, memory), cpu, memory); 4},
        STA_ABSOLUTE_X        => {instructions::sta(AddressingMode::absolute_x(cpu, memory), cpu, memory); 5},
        STA_ABSOLUTE_Y        => {instructions::sta(AddressingMode::absolute_y(cpu, memory), cpu, memory); 5},
        STA_INDIRECT_X        => {instructions::sta(AddressingMode::indirect_x(cpu, memory), cpu, memory); 6},
        STA_INDIRECT_Y        => {instructions::sta(AddressingMode::indirect_y(cpu, memory), cpu, memory); 6},
        TXS                   => { let temp = cpu.register_x(); cpu.stack_pointer = temp; 2},
        TSX                   => { let temp = cpu.stack_pointer; cpu.load_x(temp); 2},
        PHA                   => { memory.set(cpu.push_stack(), cpu.accumulator()); 3 },
        PLA                   => { let temp = memory.get(cpu.pop_stack()); cpu.load_accumulator(temp); 4},
        PHP                   => { memory.set(cpu.push_stack(), cpu.processor_status()); 3 },
        PLP                   => { let temp = memory.get(cpu.pop_stack()); cpu.clear_flags(0xFF); cpu.set_flags(temp); 4},
        STX_ZERO_PAGE         => {instructions::stx(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3},
        STX_ZERO_PAGE_Y       => {instructions::stx(AddressingMode::zero_paged_y(cpu, memory), cpu, memory); 4},
        STX_ABSOLUTE          => {instructions::stx(AddressingMode::absolute(cpu, memory), cpu, memory); 4},
        STY_ZERO_PAGE         => {instructions::sty(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3},
        STY_ZERO_PAGE_X       => {instructions::sty(AddressingMode::zero_paged_x(cpu, memory), cpu, memory); 4},
        STY_ABSOLUTE          => {instructions::sty(AddressingMode::absolute(cpu, memory), cpu, memory); 4},
        _                     => panic!("Unknown OpCode"),
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
pub const RTS        : OpCode = 0x60;
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

#[cfg(test)]
mod tests {
    use cpu;
    use memory::Memory;
    use opcodes;

    fn test_program(memory: &mut Memory, expected_cpu_states: &[cpu::CPU]) {
        let mut cpu = cpu::CPU::new();

        for &expected_cpu in expected_cpu_states.iter() {
            super::execute_instruction(&mut cpu, memory);
            assert_eq!(expected_cpu, cpu);
        }
    }

    #[test]
    fn instruction_test1() {
        test_program(
            &mut memory!(
                0x00A5 => 0xF0,
                //ADC $05
                0x8000 => 0x69,
                0x8001 => 0x05,

                //AND $00
                0x8002 => 0x29,
                0x8003 => 0x00,
                //ORA $05
                0x8004 => opcodes::ORA_IMMEDIATE,
                0x8005 => 0x05,

                0x8006 => opcodes::ASL_ACCUMULATOR,

                0x8007 => opcodes::SEC,
                //SBC $05
                0x8008 => opcodes::SBC_IMMEDIATE,
                0x8009 => 0x05,

                0x800A => opcodes::TAX,
                0x800B => opcodes::TAY,
                //STX Y
                0x800C => opcodes::STX_ZERO_PAGE_Y,
                0x800D => 0x0A,
                0x800E => opcodes::AND_IMMEDIATE,
                0x800F => 0x00,
                0x8010 => opcodes::TAX,
                //LDX Y
                0x8011 => opcodes::LDX_ZERO_PAGE_Y,
                0x8012 => 0x0A
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
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8008)
                    .flags(cpu::CARRY_FLAG)
                    .accumulator(0x0A) //1010
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800A)
                    .flags(cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800B)
                    .flags(cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800C)
                    .flags(cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800E)
                    .flags(cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8010)
                    .flags(cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8011)
                    .flags(cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8013)
                    .flags(cpu::CARRY_FLAG)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build()
            ]
        )
    }

    #[test]
    fn instruction_test2() {
        test_program(
            &mut memory!(
                //ADC $05
                0x8000 => 0x69,
                0x8001 => 0x05,

                0x8002 => opcodes::PHA,
                0x8003 => opcodes::PLP
            ),
            &[
                cpu::CpuBuilder::new()
                    .program_counter(0x8002)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
                    .stack_pointer(0xFF)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
                    .flags(0x05)
                    .accumulator(0x05)
                    .build(),
            ]
        );
    }

    #[test]
    fn test_subroutine() {
        test_program(
            &mut memory!(
                0x8000 => opcodes::JSR_ABSOLUTE,
                0x8001 => 0x20,
                0x8002 => 0x80,

                0x8003 => opcodes::ADC_IMMEDIATE,
                0x8004 => 0x05,

                //Sub routine
                0x8020 => opcodes::LDA_IMMEDIATE,
                0x8021 => 0x01,
                0x8022 => opcodes::RTS
            ),
            &[
                cpu::CpuBuilder::new()
                    .program_counter(0x8020)
                    .stack_pointer(0xFE)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8022)
                    .stack_pointer(0xFE)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8005)
                    .accumulator(0x06)
                    .build(),
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
        let mut cpu = cpu::CPU::new();
        super::execute_instruction(&mut cpu, memory);

        assert_eq!(expected_cpu, cpu);
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

                let mut cpu = cpu::CPU::new();
                cpu.set_flags(flag);
                super::execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new();
                cpu.set_flags(flag);
                super::execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new();
                cpu.clear_flags(flag);
                super::execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new();
                cpu.clear_flags(flag);
                super::execute_instruction(&mut cpu, &mut memory);
                if negative {
                    assert_eq!(0x7FFC, cpu.program_counter());
                } else {
                    assert_eq!(0x8002, cpu.program_counter());
                }
            }
        }
    }
}