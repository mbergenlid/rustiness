use instructions;
use addressing::AddressingMode;
use cpu::CPU;
use cpu;
use memory::Memory;

pub struct OpCodes {
    codes: Vec<Option<Instruction>>,
}

impl OpCodes {
    pub fn new() -> OpCodes {
        let mut codes: Vec<Option<Instruction>> = vec![];
        for _ in 0..0x100 {
            codes.push(None);
        }

        for op_code in OP_CODES.iter() {
            let c = op_code.0;
            codes[c as usize] = Some(op_code.1);
        }

        OpCodes {
            codes: codes,
        }
    }

    pub fn execute_instruction(&self, cpu: &mut CPU, memory: &mut Memory) -> u8 {
        let pc = cpu.get_and_increment_pc();
        let op_code: u8 = memory.get(pc);

        match self.codes[op_code as usize] {
            Some(ref instruction) => (instruction)(cpu, memory),
            None => panic!("Unknown opcode {} at location 0x{:x}.", op_code, pc),
        }
    }
}

type Instruction = &'static Fn(&mut CPU, &mut Memory) -> u8;
struct OpCodeInstruction(OpCode, &'static Fn(&mut CPU, &mut Memory) -> u8);

const OP_CODES: [OpCodeInstruction; 156] = [
    OpCodeInstruction(ADC_IMMEDIATE         , &|cpu, memory| instructions::adc(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(ADC_ZERO_PAGE         , &|cpu, memory| instructions::adc(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_ZERO_PAGE_X       , &|cpu, memory| instructions::adc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_ABSOLUTE          , &|cpu, memory| instructions::adc(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_ABSOLUTE_X        , &|cpu, memory| instructions::adc(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_ABSOLUTE_Y        , &|cpu, memory| instructions::adc(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_INDIRECT_X        , &|cpu, memory| instructions::adc(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ADC_INDIRECT_Y        , &|cpu, memory| instructions::adc(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_IMMEDIATE         , &|cpu, memory| instructions::and(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(AND_ZERO_PAGE         , &|cpu, memory| instructions::and(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_ZERO_PAGE_X       , &|cpu, memory| instructions::and(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_ABSOLUTE          , &|cpu, memory| instructions::and(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_ABSOLUTE_X        , &|cpu, memory| instructions::and(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_ABSOLUTE_Y        , &|cpu, memory| instructions::and(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_INDIRECT_X        , &|cpu, memory| instructions::and(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(AND_INDIRECT_Y        , &|cpu, memory| instructions::and(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(ASL_ACCUMULATOR       , &|cpu,      _| instructions::asl_accumulator(cpu)),
    OpCodeInstruction(ASL_ZERO_PAGE         , &|cpu, memory| instructions::asl(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(ASL_ZERO_PAGE_X       , &|cpu, memory| instructions::asl(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ASL_ABSOLUTE          , &|cpu, memory| instructions::asl(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(ASL_ABSOLUTE_X        , &|cpu, memory| {instructions::asl(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(BIT_ZERO_PAGE         , &|cpu, memory| instructions::bit(AddressingMode::zero_paged(cpu ,memory), cpu, memory)),
    OpCodeInstruction(BIT_ABSOLUTE          , &|cpu, memory| instructions::bit(AddressingMode::absolute(cpu ,memory), cpu, memory)),
    OpCodeInstruction(BRANCH_PLUS           , &|cpu, memory| instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, true)),
    OpCodeInstruction(BRANCH_MINUS          , &|cpu, memory| instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, false)),
    OpCodeInstruction(BRANCH_OVERFLOW_SET   , &|cpu, memory| instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, false)),
    OpCodeInstruction(BRANCH_OVERFLOW_CLEAR , &|cpu, memory| instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, true)),
    OpCodeInstruction(BRANCH_CARRY_SET      , &|cpu, memory| instructions::branch(cpu, memory, cpu::CARRY_FLAG, false)),
    OpCodeInstruction(BRANCH_CARRY_CLEAR    , &|cpu, memory| instructions::branch(cpu, memory, cpu::CARRY_FLAG, true)),
    OpCodeInstruction(BRANCH_NOT_EQUAL      , &|cpu, memory| instructions::branch(cpu, memory, cpu::ZERO_FLAG, true)),
    OpCodeInstruction(BRANCH_EQUAL          , &|cpu, memory| instructions::branch(cpu, memory, cpu::ZERO_FLAG, false)),
    OpCodeInstruction(BRK                   , &|cpu, memory| instructions::brk(cpu, memory)),
    OpCodeInstruction(CMP_IMMEDIATE         , &|cpu, memory| instructions::cmp(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(CMP_ZERO_PAGE         , &|cpu, memory| instructions::cmp(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_ZERO_PAGE_X       , &|cpu, memory| instructions::cmp(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_ABSOLUTE          , &|cpu, memory| instructions::cmp(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_ABSOLUTE_X        , &|cpu, memory| instructions::cmp(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_ABSOLUTE_Y        , &|cpu, memory| instructions::cmp(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_INDIRECT_X        , &|cpu, memory| instructions::cmp(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(CMP_INDIRECT_Y        , &|cpu, memory| instructions::cmp(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(CPX_IMMEDIATE         , &|cpu, memory| instructions::cpx(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(CPX_ZERO_PAGE         , &|cpu, memory| instructions::cpx(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(CPX_ABSOLUTE          , &|cpu, memory| instructions::cpx(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(CPY_IMMEDIATE         , &|cpu, memory| instructions::cpy(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(CPY_ZERO_PAGE         , &|cpu, memory| instructions::cpy(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(CPY_ABSOLUTE          , &|cpu, memory| instructions::cpy(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(DEC_ZERO_PAGE         , &|cpu, memory| instructions::dec(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(DEC_ZERO_PAGE_X       , &|cpu, memory| instructions::dec(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(DEC_ABSOLUTE          , &|cpu, memory| instructions::dec(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(DEC_ABSOLUTE_X        , &|cpu, memory| {instructions::dec(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(EOR_IMMEDIATE         , &|cpu, memory| instructions::eor(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(EOR_ZERO_PAGE         , &|cpu, memory| instructions::eor(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_ZERO_PAGE_X       , &|cpu, memory| instructions::eor(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_ABSOLUTE          , &|cpu, memory| instructions::eor(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_ABSOLUTE_X        , &|cpu, memory| instructions::eor(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_ABSOLUTE_Y        , &|cpu, memory| instructions::eor(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_INDIRECT_X        , &|cpu, memory| instructions::eor(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(EOR_INDIRECT_Y        , &|cpu, memory| instructions::eor(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(CLC                   , &|cpu,      _| { cpu.clear_flags(cpu::CARRY_FLAG); 2}),
    OpCodeInstruction(SEC                   , &|cpu,      _| { cpu.set_flags(cpu::CARRY_FLAG); 2}),
    OpCodeInstruction(CLI                   , &|cpu,      _| {cpu.clear_flags(cpu::INTERRUPT_DISABLE_FLAG); 2}),
    OpCodeInstruction(SEI                   , &|cpu,      _| { cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG); 2}),
    OpCodeInstruction(CLV                   , &|cpu,      _| {cpu.clear_flags(cpu::OVERFLOW_FLAG); 2}),
    OpCodeInstruction(CLD                   , &|cpu,      _| {cpu.clear_flags(cpu::DECIMAL_FLAG); 2}),
    OpCodeInstruction(SED                   , &|cpu,      _| { cpu.set_flags(cpu::DECIMAL_FLAG); 2}),
    OpCodeInstruction(INC_ZERO_PAGE         , &|cpu, memory| instructions::inc(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(INC_ZERO_PAGE_X       , &|cpu, memory| instructions::inc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(INC_ABSOLUTE          , &|cpu, memory| instructions::inc(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(INC_ABSOLUTE_X        , &|cpu, memory| {instructions::inc(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(JMP_ABSOLUTE          , &|cpu, memory| {instructions::jmp(AddressingMode::absolute(cpu, memory), cpu); 3 }),
    OpCodeInstruction(JMP_INDIRECT          , &|cpu, memory| {instructions::jmp(AddressingMode::indirect(cpu, memory), cpu); 5 }),
    OpCodeInstruction(JSR_ABSOLUTE          , &|cpu, memory| instructions::jsr(cpu, memory)),
    OpCodeInstruction(LDA_IMMEDIATE         , &|cpu, memory| instructions::lda(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(LDA_ZERO_PAGE         , &|cpu, memory| instructions::lda(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_ZERO_PAGE_X       , &|cpu, memory| instructions::lda(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_ABSOLUTE          , &|cpu, memory| instructions::lda(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_ABSOLUTE_X        , &|cpu, memory| instructions::lda(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_ABSOLUTE_Y        , &|cpu, memory| instructions::lda(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_INDIRECT_X        , &|cpu, memory| instructions::lda(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDA_INDIRECT_Y        , &|cpu, memory| instructions::lda(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDX_IMMEDIATE         , &|cpu, memory| instructions::ldx(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(LDX_ZERO_PAGE         , &|cpu, memory| instructions::ldx(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDX_ZERO_PAGE_Y       , &|cpu, memory| instructions::ldx(AddressingMode::zero_paged_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDX_ABSOLUTE          , &|cpu, memory| instructions::ldx(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDX_ABSOLUTE_Y        , &|cpu, memory| instructions::ldx(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDY_IMMEDIATE         , &|cpu, memory| instructions::ldy(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(LDY_ZERO_PAGE         , &|cpu, memory| instructions::ldy(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDY_ZERO_PAGE_X       , &|cpu, memory| instructions::ldy(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDY_ABSOLUTE          , &|cpu, memory| instructions::ldy(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(LDY_ABSOLUTE_X        , &|cpu, memory| instructions::ldy(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LSR_ACCUMULATOR       , &|cpu,      _| {cpu.logical_shift_right_accumulator(); 2}),
    OpCodeInstruction(LSR_ZERO_PAGE         , &|cpu, memory| instructions::lsr(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(LSR_ZERO_PAGE_X       , &|cpu, memory| instructions::lsr(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(LSR_ABSOLUTE          , &|cpu, memory| instructions::lsr(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(LSR_ABSOLUTE_X        , &|cpu, memory| {instructions::lsr(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(NOP_IMPLIED           , &|  _,      _| 2),
    OpCodeInstruction(ORA_IMMEDIATE         , &|cpu, memory| instructions::or(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(ORA_ZERO_PAGE         , &|cpu, memory| instructions::or(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_ZERO_PAGE_X       , &|cpu, memory| instructions::or(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_ABSOLUTE          , &|cpu, memory| instructions::or(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_ABSOLUTE_X        , &|cpu, memory| instructions::or(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_ABSOLUTE_Y        , &|cpu, memory| instructions::or(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_INDIRECT_X        , &|cpu, memory| instructions::or(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ORA_INDIRECT_Y        , &|cpu, memory| instructions::or(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(TAX                   , &|cpu,      _| { let acc = cpu.accumulator(); cpu.load_x(acc); 2}),
    OpCodeInstruction(TXA                   , &|cpu,      _| { let temp = cpu.register_x(); cpu.load_accumulator(temp); 2}),
    OpCodeInstruction(DEX                   , &|cpu,      _| { cpu.decrement_x(); 2 }),
    OpCodeInstruction(INX                   , &|cpu,      _| { cpu.increment_x(); 2 }),
    OpCodeInstruction(TAY                   , &|cpu,      _| { let temp = cpu.accumulator(); cpu.load_y(temp); 2}),
    OpCodeInstruction(TYA                   , &|cpu,      _| { let temp = cpu.register_y(); cpu.load_accumulator(temp); 2}),
    OpCodeInstruction(DEY                   , &|cpu,      _| { cpu.decrement_y(); 2 }),
    OpCodeInstruction(INY                   , &|cpu,      _| { cpu.increment_y(); 2 }),
    OpCodeInstruction(ROL_ACCUMULATOR       , &|cpu,      _| {cpu.rotate_accumulator_left(); 2}),
    OpCodeInstruction(ROL_ZERO_PAGE         , &|cpu, memory| instructions::rol(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROL_ZERO_PAGE_X       , &|cpu, memory| instructions::rol(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROL_ABSOLUTE          , &|cpu, memory| instructions::rol(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROL_ABSOLUTE_X        , &|cpu, memory| {instructions::rol(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(ROR_ACCUMULATOR       , &|cpu,      _| {cpu.rotate_accumulator_right(); 2}),
    OpCodeInstruction(ROR_ZERO_PAGE         , &|cpu, memory| instructions::ror(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROR_ZERO_PAGE_X       , &|cpu, memory| instructions::ror(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROR_ABSOLUTE          , &|cpu, memory| instructions::ror(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(ROR_ABSOLUTE_X        , &|cpu, memory| {instructions::ror(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(RTI                   , &|cpu, memory| instructions::rti(cpu, memory)),
    OpCodeInstruction(RTS                   , &|cpu, memory| instructions::rts(cpu, memory)),
    OpCodeInstruction(SBC_IMMEDIATE         , &|cpu, memory| instructions::sbc(AddressingMode::immediate(cpu), cpu, memory)),
    OpCodeInstruction(SBC_ZERO_PAGE         , &|cpu, memory| instructions::sbc(AddressingMode::zero_paged(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_ZERO_PAGE_X       , &|cpu, memory| instructions::sbc(AddressingMode::zero_paged_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_ABSOLUTE          , &|cpu, memory| instructions::sbc(AddressingMode::absolute(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_ABSOLUTE_X        , &|cpu, memory| instructions::sbc(AddressingMode::absolute_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_ABSOLUTE_Y        , &|cpu, memory| instructions::sbc(AddressingMode::absolute_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_INDIRECT_X        , &|cpu, memory| instructions::sbc(AddressingMode::indirect_x(cpu, memory), cpu, memory)),
    OpCodeInstruction(SBC_INDIRECT_Y        , &|cpu, memory| instructions::sbc(AddressingMode::indirect_y(cpu, memory), cpu, memory)),
    OpCodeInstruction(STA_ZERO_PAGE         , &|cpu, memory| {instructions::sta(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3}),
    OpCodeInstruction(STA_ZERO_PAGE_X       , &|cpu, memory| {instructions::sta(AddressingMode::zero_paged_x(cpu, memory), cpu, memory); 4}),
    OpCodeInstruction(STA_ABSOLUTE          , &|cpu, memory| {instructions::sta(AddressingMode::absolute(cpu, memory), cpu, memory); 4}),
    OpCodeInstruction(STA_ABSOLUTE_X        , &|cpu, memory| {instructions::sta(AddressingMode::absolute_x(cpu, memory), cpu, memory); 5}),
    OpCodeInstruction(STA_ABSOLUTE_Y        , &|cpu, memory| {instructions::sta(AddressingMode::absolute_y(cpu, memory), cpu, memory); 5}),
    OpCodeInstruction(STA_INDIRECT_X        , &|cpu, memory| {instructions::sta(AddressingMode::indirect_x(cpu, memory), cpu, memory); 6}),
    OpCodeInstruction(STA_INDIRECT_Y        , &|cpu, memory| {instructions::sta(AddressingMode::indirect_y(cpu, memory), cpu, memory); 6}),
    OpCodeInstruction(TXS                   , &|cpu,      _| { let temp = cpu.register_x(); cpu.stack_pointer = temp; 2}),
    OpCodeInstruction(TSX                   , &|cpu,      _| { let temp = cpu.stack_pointer; cpu.load_x(temp); 2}),
    OpCodeInstruction(PHA                   , &|cpu, memory| { memory.set(cpu.push_stack(), cpu.accumulator()); 3 }),
    OpCodeInstruction(PLA                   , &|cpu, memory| { let temp = memory.get(cpu.pop_stack()); cpu.load_accumulator(temp); 4}),
    OpCodeInstruction(PHP                   , &|cpu, memory| { memory.set(cpu.push_stack(), cpu.processor_status()); 3 }),
    OpCodeInstruction(PLP                   , &|cpu, memory| { let temp = memory.get(cpu.pop_stack()); cpu.clear_flags(0xDF); cpu.set_flags(temp); 4}),
    OpCodeInstruction(STX_ZERO_PAGE         , &|cpu, memory| {instructions::stx(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3}),
    OpCodeInstruction(STX_ZERO_PAGE_Y       , &|cpu, memory| {instructions::stx(AddressingMode::zero_paged_y(cpu, memory), cpu, memory); 4}),
    OpCodeInstruction(STX_ABSOLUTE          , &|cpu, memory| {instructions::stx(AddressingMode::absolute(cpu, memory), cpu, memory); 4}),
    OpCodeInstruction(STY_ZERO_PAGE         , &|cpu, memory| {instructions::sty(AddressingMode::zero_paged(cpu, memory), cpu, memory); 3}),
    OpCodeInstruction(STY_ZERO_PAGE_X       , &|cpu, memory| {instructions::sty(AddressingMode::zero_paged_x(cpu, memory), cpu, memory); 4}),
    OpCodeInstruction(STY_ABSOLUTE          , &|cpu, memory| {instructions::sty(AddressingMode::absolute(cpu, memory), cpu, memory); 4}),

    OpCodeInstruction(ISC_INDIRECT_X        , &|cpu, memory| {instructions::isc(AddressingMode::indirect_x(cpu, memory), cpu, memory)}),
    OpCodeInstruction(IGN_INDIRECT_X_1      , &|cpu, memory| {AddressingMode::indirect_x(cpu, memory); 4}),
    OpCodeInstruction(IGN_INDIRECT_X_3      , &|cpu, memory| {AddressingMode::indirect_x(cpu, memory); 4}),
    OpCodeInstruction(ISC_ABSOLUTE_X        , &|cpu, memory| {instructions::isc(AddressingMode::absolute_x(cpu, memory), cpu, memory); 7}),
    OpCodeInstruction(SRE_INDIRECT_X        , &|cpu, memory| {instructions::sre(AddressingMode::indirect_x(cpu, memory), cpu, memory); 6}),
];

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
pub const BRK         : OpCode = 0x00;
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
pub const RTI        : OpCode = 0x40;
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


//Unofficial opcodes
pub const ISC_INDIRECT_X: OpCode = 0xE3;
pub const IGN_INDIRECT_X_1: OpCode = 0x14;
pub const IGN_INDIRECT_X_3: OpCode = 0x54;
pub const ISC_ABSOLUTE_X: OpCode = 0xFF;
pub const SRE_INDIRECT_X: OpCode = 0x57;


#[cfg(test)]
mod tests {
    use cpu;
    use memory::Memory;
    use opcodes;

    fn execute_instruction(cpu: &mut cpu::CPU, memory: &mut Memory) -> u8 {
        super::OpCodes::new().execute_instruction(cpu, memory)
    }

    fn test_program(memory: &mut Memory, expected_cpu_states: &[cpu::CPU]) {
        let op_codes = super::OpCodes::new();
        let mut cpu = cpu::CPU::new(0x8000);

        for &expected_cpu in expected_cpu_states.iter() {
            op_codes.execute_instruction(&mut cpu, memory);
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
                    .flags(0x34 | cpu::ZERO_FLAG)
                    .accumulator(0x00)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8006)
//                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8007)
                    .accumulator(0x0A) //1010
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8008)
                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x0A) //1010
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800A)
                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800B)
                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800C)
                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800E)
                    .flags(0x34 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8010)
                    .flags(0x34 | cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8011)
                    .flags(0x34 | cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8013)
                    .flags(0x34 | cpu::CARRY_FLAG)
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
                    .stack_pointer(0xFF)
                    .flags(0x34)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
                    .stack_pointer(0xFE)
                    .flags(0x34)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
                    .stack_pointer(0xFF)
                    .flags(0x25)
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
                    .stack_pointer(0xFD)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8022)
                    .stack_pointer(0xFD)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
                    .stack_pointer(0xFF)
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
    fn test_break() {
        test_program(
            &mut memory!(
                0x8000 => opcodes::BRK,

                0x8001 => opcodes::ADC_IMMEDIATE,
                0x8002 => 0x05,

                //Interrupt routine
                0x8020 => opcodes::LDA_IMMEDIATE,
                0x8021 => 0x01,
                0x8022 => opcodes::RTI,

                0xFFFE => 0x20,
                0xFFFF => 0x80
            ),
            &[
                cpu::CpuBuilder::new()
                    .program_counter(0x8020)
                    .stack_pointer(0xFC)
                    .flags(0x34 | cpu::BREAK_FLAG)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8022)
                    .stack_pointer(0xFC)
                    .accumulator(0x01)
                    .flags(0x34 | cpu::BREAK_FLAG)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8001)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
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

    #[test]
    fn test_incrememnt_memory() {
        let mut cpu = cpu::CPU::new(0x8000);
        let mut memory = &mut memory!(
            0x0010 => 5,
            0x8000 => 0xE6, //inc $10
            0x8001 => 0x10
        );
        execute_instruction(&mut cpu, memory);

        assert_eq!(6, memory.get(0x0010));
    }

    fn test_instruction(memory: &mut Memory, expected_cpu: cpu::CPU) {
        let mut cpu = cpu::CPU::new(0x8000);
        execute_instruction(&mut cpu, memory);

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

                let mut cpu = cpu::CPU::new(0x8000);
                cpu.set_flags(flag);
                execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new(0x8000);
                cpu.set_flags(flag);
                execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new(0x8000);
                cpu.clear_flags(flag);
                execute_instruction(&mut cpu, &mut memory);
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

                let mut cpu = cpu::CPU::new(0x8000);
                cpu.clear_flags(flag);
                execute_instruction(&mut cpu, &mut memory);
                if negative {
                    assert_eq!(0x7FFC, cpu.program_counter());
                } else {
                    assert_eq!(0x8002, cpu.program_counter());
                }
            }
        }
    }
}
