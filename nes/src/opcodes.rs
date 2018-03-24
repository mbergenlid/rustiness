use instructions;
use addressing::{AddressingMode, NO_ADDRESSING};
use cpu::CPU;
use cpu;
use memory::Memory;
use std::rc::Rc;

pub struct OpCodes {
    codes: Vec<Option<InstructionFactory>>,
}

impl OpCodes {
    pub fn new() -> OpCodes {

        OpCodes {
            codes: generate_instructions(),
        }
    }

    pub fn fetch_instruction(&self, cpu: &mut CPU, memory: &mut Memory) -> Instruction {
        let pc = cpu.get_and_increment_pc();
        let op_code: u8 = memory.get(pc);

        match self.codes[op_code as usize] {
            Some(ref factory) => Instruction { addressing_mode: (factory.0)(cpu, memory), function: factory.1.clone() },
            None => panic!("Unknown opcode {:x} at location 0x{:x}.", op_code, pc),
        }
    }

    pub fn execute_instruction(&self, cpu: &mut CPU, memory: &mut Memory) -> u8 {
        let instruction = self.fetch_instruction(cpu, memory);
        instruction.execute(cpu, memory)
    }
}

pub struct Instruction {
    addressing_mode: AddressingMode,
    function: Rc<Box<Fn(&AddressingMode, &mut CPU, &mut Memory) -> u8>>,
}
impl Instruction {
    pub fn execute(&self, cpu: &mut CPU, memory: &mut Memory) -> u8 {
        (self.function)(&self.addressing_mode, cpu, memory)
    }
    pub fn fetch_cycles(&self) -> u8 {
        self.addressing_mode.cycles
    }
}
type InstructionFactory = (Box<Fn(&mut CPU, &mut Memory) -> AddressingMode>, Rc<Box<Fn(&AddressingMode, &mut CPU, &mut Memory) -> u8>>);

fn generate_instructions() -> Vec<Option<InstructionFactory>> {
    let mut codes: Vec<Option<InstructionFactory>> = vec![];
    for _ in 0..0x100 {
        codes.push(None);
    }
    codes[ADC_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[ADC_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::adc(mode, cpu, memory)))));
    codes[AND_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[AND_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::and(mode, cpu, memory)))));
    codes[ASL_ACCUMULATOR       as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| instructions::asl_accumulator(cpu)))));
    codes[ASL_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::asl(mode, cpu, memory)))));
    codes[ASL_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::asl(mode, cpu, memory)))));
    codes[ASL_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::asl(mode, cpu, memory)))));
    codes[ASL_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|mode, cpu, memory| {instructions::asl(mode, cpu, memory); 7}))));
    codes[BIT_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu ,memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::bit(mode, cpu, memory)))));
    codes[BIT_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu ,memory)), Rc::new(Box::new(|mode, cpu, memory| instructions::bit(mode, cpu, memory)))));
    codes[BRANCH_PLUS           as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, true)))));
    codes[BRANCH_MINUS          as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::NEGATIVE_FLAG, false)))));
    codes[BRANCH_OVERFLOW_SET   as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, false)))));
    codes[BRANCH_OVERFLOW_CLEAR as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::OVERFLOW_FLAG, true)))));
    codes[BRANCH_CARRY_SET      as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::CARRY_FLAG, false)))));
    codes[BRANCH_CARRY_CLEAR    as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::CARRY_FLAG, true)))));
    codes[BRANCH_NOT_EQUAL      as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::ZERO_FLAG, true)))));
    codes[BRANCH_EQUAL          as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::branch(cpu, memory, cpu::ZERO_FLAG, false)))));
    codes[BRK                   as usize] = Some((Box::new(|_,_| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::brk(cpu, memory)))));
    codes[CMP_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CMP_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cmp(mode, cpu, memory)))));
    codes[CPX_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpx(mode, cpu, memory)))));
    codes[CPX_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpx(mode, cpu, memory)))));
    codes[CPX_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpx(mode, cpu, memory)))));
    codes[CPY_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpy(mode, cpu, memory)))));
    codes[CPY_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpy(mode, cpu, memory)))));
    codes[CPY_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::cpy(mode, cpu, memory)))));
    codes[DEC_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::dec(mode, cpu, memory)))));
    codes[DEC_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::dec(mode, cpu, memory)))));
    codes[DEC_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::dec(mode, cpu, memory)))));
    codes[DEC_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::dec(mode, cpu, memory); 7}))));
    codes[EOR_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[EOR_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::eor(mode, cpu, memory)))));
    codes[CLC                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.clear_flags(cpu::CARRY_FLAG); 2}))));
    codes[SEC                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.set_flags(cpu::CARRY_FLAG); 2}))));
    codes[CLI                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.clear_flags(cpu::INTERRUPT_DISABLE_FLAG); 2}))));
    codes[SEI                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG); 2}))));
    codes[CLV                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.clear_flags(cpu::OVERFLOW_FLAG); 2}))));
    codes[CLD                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.clear_flags(cpu::DECIMAL_FLAG); 2}))));
    codes[SED                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, _| { cpu.set_flags(cpu::DECIMAL_FLAG); 2}))));
    codes[INC_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::inc(mode, cpu, memory)))));
    codes[INC_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::inc(mode, cpu, memory)))));
    codes[INC_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::inc(mode, cpu, memory)))));
    codes[INC_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::inc(mode, cpu, memory); 7}))));
    codes[JMP_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, _| {instructions::jmp(mode, cpu); 3 }))));
    codes[JMP_INDIRECT          as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, _| {instructions::jmp(mode, cpu); 5 }))));
    codes[JSR_ABSOLUTE          as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::jsr(cpu, memory)))));
    codes[LDA_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDA_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lda(mode, cpu, memory)))));
    codes[LDX_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldx(mode, cpu, memory)))));
    codes[LDX_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldx(mode, cpu, memory)))));
    codes[LDX_ZERO_PAGE_Y       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldx(mode, cpu, memory)))));
    codes[LDX_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldx(mode, cpu, memory)))));
    codes[LDX_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldx(mode, cpu, memory)))));
    codes[LDY_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldy(mode, cpu, memory)))));
    codes[LDY_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldy(mode, cpu, memory)))));
    codes[LDY_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldy(mode, cpu, memory)))));
    codes[LDY_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldy(mode, cpu, memory)))));
    codes[LDY_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ldy(mode, cpu, memory)))));
    codes[LSR_ACCUMULATOR       as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| {cpu.logical_shift_right_accumulator(); 2}))));
    codes[LSR_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lsr(mode, cpu, memory)))));
    codes[LSR_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lsr(mode, cpu, memory)))));
    codes[LSR_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::lsr(mode, cpu, memory)))));
    codes[LSR_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::lsr(mode, cpu, memory); 7}))));
    codes[NOP_IMPLIED           as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_,   _,      _| 2))));
    codes[ORA_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[ORA_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::or(mode, cpu, memory)))));
    codes[TAX                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let acc = cpu.accumulator(); cpu.load_x(acc); 2}))));
    codes[TXA                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let temp = cpu.register_x(); cpu.load_accumulator(temp); 2}))));
    codes[DEX                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { cpu.decrement_x(); 2 }))));
    codes[INX                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { cpu.increment_x(); 2 }))));
    codes[TAY                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let temp = cpu.accumulator(); cpu.load_y(temp); 2}))));
    codes[TYA                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let temp = cpu.register_y(); cpu.load_accumulator(temp); 2}))));
    codes[DEY                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { cpu.decrement_y(); 2 }))));
    codes[INY                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { cpu.increment_y(); 2 }))));
    codes[ROL_ACCUMULATOR       as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| {cpu.rotate_accumulator_left(); 2}))));
    codes[ROL_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::rol(mode, cpu, memory)))));
    codes[ROL_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::rol(mode, cpu, memory)))));
    codes[ROL_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::rol(mode, cpu, memory)))));
    codes[ROL_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::rol(mode, cpu, memory); 7}))));
    codes[ROR_ACCUMULATOR       as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| {cpu.rotate_accumulator_right(); 2}))));
    codes[ROR_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ror(mode, cpu, memory)))));
    codes[ROR_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ror(mode, cpu, memory)))));
    codes[ROR_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::ror(mode, cpu, memory)))));
    codes[ROR_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::ror(mode, cpu, memory); 7}))));
    codes[RTI                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::rti(cpu, memory)))));
    codes[RTS                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| instructions::rts(cpu, memory)))));
    codes[SBC_IMMEDIATE         as usize] = Some((Box::new(|cpu, _| AddressingMode::immediate(cpu)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[SBC_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| instructions::sbc(mode, cpu, memory)))));
    codes[STA_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 3}))));
    codes[STA_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 4}))));
    codes[STA_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 4}))));
    codes[STA_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 5}))));
    codes[STA_ABSOLUTE_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 5}))));
    codes[STA_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 6}))));
    codes[STA_INDIRECT_Y        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sta(mode, cpu, memory); 6}))));
    codes[TXS                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let temp = cpu.register_x(); cpu.stack_pointer = temp; 2}))));
    codes[TSX                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu,      _| { let temp = cpu.stack_pointer; cpu.load_x(temp); 2}))));
    codes[PHA                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| { memory.set(cpu.push_stack(), cpu.accumulator()); 3 }))));
    codes[PLA                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| { let temp = memory.get(cpu.pop_stack()); cpu.load_accumulator(temp); 4}))));
    codes[PHP                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| { memory.set(cpu.push_stack(), cpu.processor_status() | 0x30); 3 }))));
    codes[PLP                   as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| { let temp = memory.get(cpu.pop_stack()); cpu.set_processor_status(temp); 4}))));
    codes[STX_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::stx(mode, cpu, memory); 3}))));
    codes[STX_ZERO_PAGE_Y       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_y(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::stx(mode, cpu, memory); 4}))));
    codes[STX_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::stx(mode, cpu, memory); 4}))));
    codes[STY_ZERO_PAGE         as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sty(mode, cpu, memory); 3}))));
    codes[STY_ZERO_PAGE_X       as usize] = Some((Box::new(|cpu, memory| AddressingMode::zero_paged_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sty(mode, cpu, memory); 4}))));
    codes[STY_ABSOLUTE          as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sty(mode, cpu, memory); 4}))));
    codes[ISC_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::isc(mode, cpu, memory)}))));
    codes[IGN_INDIRECT_X_1      as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| {AddressingMode::indirect_x(cpu, memory); 4}))));
    codes[IGN_INDIRECT_X_3      as usize] = Some((Box::new(|_, _| NO_ADDRESSING), Rc::new(Box::new(|_, cpu, memory| {AddressingMode::indirect_x(cpu, memory); 4}))));
    codes[ISC_ABSOLUTE_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::absolute_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::isc(mode, cpu, memory); 7}))));
    codes[SRE_INDIRECT_X        as usize] = Some((Box::new(|cpu, memory| AddressingMode::indirect_x(cpu, memory)), Rc::new(Box::new(|ref mode, cpu, memory| {instructions::sre(mode, cpu, memory); 6}))));
    return codes;
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
                    .flags(0x04 | cpu::ZERO_FLAG)
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
                    .flags(0x04 | cpu::CARRY_FLAG)
                    .accumulator(0x0A) //1010
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800A)
                    .flags(0x04 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800B)
                    .flags(0x04 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800C)
                    .flags(0x04 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x800E)
                    .flags(0x04 | cpu::CARRY_FLAG)
                    .accumulator(0x05)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8010)
                    .flags(0x04 | cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_x(0x05)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8011)
                    .flags(0x04 | cpu::CARRY_FLAG | cpu::ZERO_FLAG)
                    .register_y(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8013)
                    .flags(0x04 | cpu::CARRY_FLAG)
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
                    .flags(0x04)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8003)
                    .stack_pointer(0xFE)
                    .flags(0x04)
                    .accumulator(0x05)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
                    .stack_pointer(0xFF)
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
                0x8001 => opcodes::NOP_IMPLIED,

                0x8002 => opcodes::ADC_IMMEDIATE,
                0x8003 => 0x05,

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
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8022)
                    .stack_pointer(0xFC)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8002)
                    .accumulator(0x01)
                    .build(),
                cpu::CpuBuilder::new()
                    .program_counter(0x8004)
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
        let memory = &mut memory!(
            0x0010 => 5,
            0x8000 => 0xE6, //inc $10
            0x8001 => 0x10
        );
        execute_instruction(&mut cpu, memory);

        assert_eq!(6, memory.get(0x0010));
    }

    #[test]
    fn php_should_set_bits_4_and_5() {
        let mut cpu = cpu::CpuBuilder::new()
            .program_counter(0x8000)
            .flags(0)
            .build();
        let memory = &mut memory!(
            0x8000 => super::PHP
        );
        execute_instruction(&mut cpu, memory);

        assert_eq!(0x30, memory.get(0x01ff));
    }

    #[test]
    fn brk_should_set_interrupt_disable_flag() {
        let memory = &mut memory!(
            0x8000 => opcodes::BRK
        );
        let mut cpu = cpu::CpuBuilder::new()
            .program_counter(0x8000)
            .flags(0)
            .build();
        execute_instruction(&mut cpu, memory);
        assert_eq!(true, cpu.is_flag_set(cpu::INTERRUPT_DISABLE_FLAG));
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
