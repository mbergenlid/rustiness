use addressing::AddressingMode;
use cpu::CPU;
use cpu;
use memory::Memory;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory);
    fn estimated_cycles(&self) -> u8;
}

pub struct ADC(AddressingMode);
impl ADC {
    pub fn new(mode: AddressingMode) -> ADC {
        ADC(mode)
    }
}
impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.add_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct SBC(AddressingMode);
impl SBC {
    pub fn new(mode: AddressingMode) -> SBC {
        SBC(mode)
    }
}
impl Instruction for SBC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.sub_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct INC(AddressingMode);
impl INC {
    pub fn new(mode: AddressingMode) -> INC {
        INC(mode)
    }
}
impl Instruction for INC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.increment(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles+2);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}
pub struct INCAbsoluteX(INC);
impl INCAbsoluteX {
    pub fn new(cpu: &mut CPU, memory: &mut Memory) -> INCAbsoluteX {
        INCAbsoluteX(INC::new(AddressingMode::absolute_x(cpu, memory)))
    }
}
impl Instruction for INCAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        self.0.execute(cpu, memory);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct DEC(AddressingMode);
impl DEC {
    pub fn new(mode: AddressingMode) -> DEC {
        DEC(mode)
    }
}
impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.decrement(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles+2);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}
pub struct DECAbsoluteX(DEC);
impl DECAbsoluteX {
    pub fn new(cpu: &mut CPU, memory: &mut Memory) -> DECAbsoluteX {
        DECAbsoluteX(DEC::new(AddressingMode::absolute_x(cpu, memory)))
    }
}

impl Instruction for DECAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        self.0.execute(cpu, memory);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct AND(AddressingMode);
impl AND {
    pub fn new(mode: AddressingMode) -> AND {
        AND(mode)
    }
}
impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.and_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct OR(AddressingMode);
impl OR {
    pub fn new(mode: AddressingMode) -> OR {
        OR(mode)
    }
}
impl Instruction for OR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.or_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct EOR(AddressingMode);
impl EOR {
    pub fn new(mode: AddressingMode) -> EOR {
        EOR(mode)
    }
}
impl Instruction for EOR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.xor_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct ASLAccumulator;
impl Instruction for ASLAccumulator {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.asl_accumulator();
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}

pub struct ASL(AddressingMode);
impl ASL {
    pub fn new(mode: AddressingMode) -> ASL {
        ASL(mode)
    }
}
impl Instruction for ASL {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.arithmetic_shift_left(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}

pub struct ASLAbsoluteX;
impl Instruction for ASLAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addressing_mode = AddressingMode::absolute_x(cpu, memory);
        let new_value = cpu.arithmetic_shift_left(memory.get(addressing_mode.operand_address, 4));
        memory.set(addressing_mode.operand_address, new_value, 6);
    }
    fn estimated_cycles(&self) -> u8 {
        7
    }
}

pub struct LSRAccumulator;
impl Instruction for LSRAccumulator {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.logical_shift_right_accumulator();
    }
    fn estimated_cycles(&self) -> u8 {
        return 2;
    }
}
pub struct LSR(AddressingMode);
impl LSR {
    pub fn new(mode: AddressingMode) -> LSR {
        LSR(mode)
    }
}
impl Instruction for LSR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.logical_shift_right(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles+2);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}
pub struct LSRAbsoluteX(AddressingMode);
impl LSRAbsoluteX {
    pub fn new(cpu: &mut CPU, memory: &mut Memory) -> LSRAbsoluteX {
        LSRAbsoluteX(AddressingMode::absolute_x(cpu, memory))
    }
}
impl Instruction for LSRAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.logical_shift_right(memory.get(self.0.operand_address, 4));
        memory.set(self.0.operand_address, new_value, 6);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct ROLAccumulator;
impl Instruction for ROLAccumulator {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.rotate_accumulator_left();
    }
    fn estimated_cycles(&self) -> u8 {
        return 2;
    }
}
pub struct ROL(AddressingMode);
impl ROL {
    pub fn new(mode: AddressingMode) -> ROL {
        ROL(mode)
    }
}
impl Instruction for ROL {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.rotate_left(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles+2);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}
pub struct ROLAbsoluteX(AddressingMode);
impl ROLAbsoluteX {
    pub fn new(cpu: &mut CPU, memory: &mut Memory) -> ROLAbsoluteX {
        ROLAbsoluteX(AddressingMode::absolute_x(cpu, memory))
    }
}
impl Instruction for ROLAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.rotate_left(memory.get(self.0.operand_address, 4));
        memory.set(self.0.operand_address, new_value, 6);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct RORAccumulator;
impl Instruction for RORAccumulator {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.rotate_accumulator_right();
    }
    fn estimated_cycles(&self) -> u8 {
        return 2;
    }
}
pub struct ROR(AddressingMode);
impl ROR {
    pub fn new(mode: AddressingMode) -> ROR {
        ROR(mode)
    }
}
impl Instruction for ROR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.rotate_right(memory.get(self.0.operand_address, self.0.cycles));
        memory.set(self.0.operand_address, new_value, self.0.cycles+2);
    }
    fn estimated_cycles(&self) -> u8 {
        3 + self.0.cycles
    }
}
pub struct RORAbsoluteX(AddressingMode);
impl RORAbsoluteX {
    pub fn new(cpu: &mut CPU, memory: &mut Memory) -> RORAbsoluteX {
        RORAbsoluteX(AddressingMode::absolute_x(cpu, memory))
    }
}
impl Instruction for RORAbsoluteX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let new_value = cpu.rotate_right(memory.get(self.0.operand_address, 4));
        memory.set(self.0.operand_address, new_value, 6);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct BIT(AddressingMode);
impl BIT {
    pub fn new(mode: AddressingMode) -> BIT {
        BIT(mode)
    }
}
impl Instruction for BIT {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.bit_test(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct Branch(bool);
impl Branch {
    pub fn new(cpu: &mut CPU, flag: u8, inverted: bool) -> Branch {
        let condition =
            if inverted {
                !cpu.is_flag_set(flag)
            } else {
                cpu.is_flag_set(flag)
            };

        Branch(condition)
    }
}
impl Instruction for Branch {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let branch_distance: i8 = memory.get(cpu.get_and_increment_pc(), 1) as i8;
        if self.0 {
            cpu.add_program_counter(branch_distance as u16);
        }
    }
    fn estimated_cycles(&self) -> u8 {
        if self.0 {
            3 //TODO: Should depend on page_crossing
        } else {
            2
        }
    }
}

pub struct JMP(AddressingMode, u8);
impl JMP {
    pub fn new(mode: AddressingMode, cycles: u8) -> JMP {
        JMP(mode, cycles)
    }
}
impl Instruction for JMP {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.set_program_counter(self.0.operand_address);
    }
    fn estimated_cycles(&self) -> u8 {
        self.1
    }
}

pub struct BRK;
impl Instruction for BRK {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let current_pc = cpu.program_counter() + 1;
        memory.set(cpu.push_stack(), (current_pc >> 8) as u8, 2);
        memory.set(cpu.push_stack(), current_pc as u8, 3);
        memory.set(cpu.push_stack(), cpu.processor_status() | 0x30, 4);

        let lsbs: u8 = memory.get(0xFFFE, 5);
        let msbs: u8 = memory.get(0xFFFF, 6);
        cpu.set_program_counter((msbs as u16) << 8 | lsbs as u16);
        cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct NMI;
impl NMI {
    pub fn new() -> NMI {
        NMI
    }
}
impl Instruction for NMI {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let current_pc = cpu.program_counter();
        memory.set(cpu.push_stack(), (current_pc >> 8) as u8, 2);
        memory.set(cpu.push_stack(), current_pc as u8, 3);
        memory.set(cpu.push_stack(), cpu.processor_status() | 0x20, 4);

        let lsbs: u8 = memory.get(0xFFFA, 5);
        let msbs: u8 = memory.get(0xFFFB, 6);
        cpu.set_program_counter((msbs as u16) << 8 | lsbs as u16);
        cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        return 7;
    }
}

pub struct RTI;
impl Instruction for RTI {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let processor_status = memory.get(cpu.pop_stack(), 3);
        cpu.set_processor_status(processor_status);
        let return_address = memory.get(cpu.pop_stack(), 4) as u16 | (memory.get(cpu.pop_stack(), 5) as u16) << 8;
        cpu.set_program_counter(return_address);
    }
    fn estimated_cycles(&self) -> u8 {
        return 6;
    }
}

pub struct JSR;
impl Instruction for JSR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let destination_address = AddressingMode::absolute(cpu, memory).operand_address;
        let current_pc = cpu.program_counter() - 1;
        memory.set(cpu.push_stack(), (current_pc >> 8) as u8, 3);
        memory.set(cpu.push_stack(), current_pc as u8, 4);

        cpu.set_program_counter(destination_address);
    }
    fn estimated_cycles(&self) -> u8 {
        return 6;
    }
}

pub struct RTS;
impl Instruction for RTS {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let return_address = memory.get(cpu.pop_stack(), 3) as u16 | (memory.get(cpu.pop_stack(), 4) as u16) << 8;
        cpu.set_program_counter(return_address + 1);
    }
    fn estimated_cycles(&self) -> u8 {
        return 6;
    }
}

pub struct STA(AddressingMode, u8);
impl STA {
    pub fn new(mode: AddressingMode, cycles: u8) -> STA {
        STA(mode, cycles)
    }
}
impl Instruction for STA {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        memory.set(self.0.operand_address, cpu.accumulator(), self.0.cycles);
    }
    fn estimated_cycles(&self) -> u8 {
        self.1
    }
}

pub struct STX(AddressingMode, u8);
impl STX {
    pub fn new(mode: AddressingMode, cycles: u8) -> STX {
        STX(mode, cycles)
    }
}
impl Instruction for STX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        memory.set(self.0.operand_address, cpu.register_x(), self.0.cycles);
    }
    fn estimated_cycles(&self) -> u8 {
        self.1
    }
}

pub struct STY(AddressingMode, u8);
impl STY {
    pub fn new(mode: AddressingMode, cycles: u8) -> STY {
        STY(mode, cycles)
    }
}
impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        memory.set(self.0.operand_address, cpu.register_y(), self.0.cycles);
    }
    fn estimated_cycles(&self) -> u8 {
        self.1
    }
}

pub struct LDX(AddressingMode);
impl LDX {
    pub fn new(mode: AddressingMode) -> LDX {
        LDX(mode)
    }
}
impl Instruction for LDX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.load_x(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct LDY(AddressingMode);
impl LDY {
    pub fn new(mode: AddressingMode) -> LDY {
        LDY(mode)
    }
}
impl Instruction for LDY {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.load_y(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct LDA(AddressingMode);
impl LDA {
    pub fn new(mode: AddressingMode) -> LDA {
        LDA(mode)
    }
}
impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.load_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct CMP(AddressingMode);
impl CMP {
    pub fn new(mode: AddressingMode) ->CMP {
        CMP(mode)
    }
}
impl Instruction for CMP {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.cmp_accumulator(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct CPX(AddressingMode);
impl CPX {
    pub fn new(mode: AddressingMode) -> CPX {
        CPX(mode)
    }
}
impl Instruction for CPX {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.cmp_register_x(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

pub struct CPY(AddressingMode);
impl CPY {
    pub fn new(mode: AddressingMode) -> CPY {
        CPY(mode)
    }
}
impl Instruction for CPY {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        cpu.cmp_register_y(memory.get(self.0.operand_address, self.0.cycles));
    }
    fn estimated_cycles(&self) -> u8 {
        1 + self.0.cycles
    }
}

//Unofficial
//pub struct ISC(AddressingMode);
//impl Instruction for ISC {
//    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
//        let new_value = cpu.increment(memory.get(self.0.operand_address));
//        memory.set(self.0.operand_address, new_value);
//        cpu.sub_accumulator(new_value);
//    }
//    fn estimated_cycles(&self) -> u8 {
//        3 + self.0.cycles
//    }
//}
//
//const DO_NOT_KNOW: u8 = 0;
//pub struct SRE(AddressingMode);
//impl Instruction for SRE {
//    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
//        let new_value = cpu.logical_shift_right(memory.get(self.0.operand_address));
//        memory.set(self.0.operand_address, new_value);
//        cpu.xor_accumulator(memory.get(self.0.operand_address));
//    }
//    fn estimated_cycles(&self) -> u8 {
//        DO_NOT_KNOW
//    }
//}

pub struct CLC;
impl Instruction for CLC {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.clear_flags(cpu::CARRY_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct SEC;
impl Instruction for SEC {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.set_flags(cpu::CARRY_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct CLI;
impl Instruction for CLI {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.clear_flags(cpu::INTERRUPT_DISABLE_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct SEI;
impl Instruction for SEI {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.set_flags(cpu::INTERRUPT_DISABLE_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct CLV;
impl Instruction for CLV {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.clear_flags(cpu::OVERFLOW_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct CLD;
impl Instruction for CLD {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.clear_flags(cpu::DECIMAL_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct SED;
impl Instruction for SED {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.set_flags(cpu::DECIMAL_FLAG);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct NOP;
impl Instruction for NOP {
    fn execute(&self, _cpu: &mut CPU, _memory: &mut Memory) {
        //DO NOTHING
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}

pub struct TAX;
impl Instruction for TAX {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let acc = cpu.accumulator();
        cpu.load_x(acc);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct TXA;
impl Instruction for TXA {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let temp = cpu.register_x();
        cpu.load_accumulator(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct DEX;
impl Instruction for DEX {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.decrement_x();
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct INX;
impl Instruction for INX {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.increment_x()
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct TAY;
impl Instruction for TAY {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let temp = cpu.accumulator();
        cpu.load_y(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct TYA;
impl Instruction for TYA {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let temp = cpu.register_y();
        cpu.load_accumulator(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct DEY;
impl Instruction for DEY {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.decrement_y();
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct INY;
impl Instruction for INY {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.increment_y();
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}

pub struct TXS;
impl Instruction for TXS {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let temp = cpu.register_x();
        cpu.stack_pointer = temp;
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct TSX;
impl Instruction for TSX {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let temp = cpu.stack_pointer;
        cpu.load_x(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        2
    }
}
pub struct PHA;
impl Instruction for PHA {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        memory.set(cpu.push_stack(), cpu.accumulator(), 2);
    }
    fn estimated_cycles(&self) -> u8 {
        3
    }
}
pub struct PLA;
impl Instruction for PLA {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let temp = memory.get(cpu.pop_stack(), 4);
        cpu.load_accumulator(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        4
    }
}
pub struct PHP;
impl Instruction for PHP {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        memory.set(cpu.push_stack(), cpu.processor_status() | 0x30, 2);
    }
    fn estimated_cycles(&self) -> u8 {
        3
    }
}
pub struct PLP;
impl Instruction for PLP {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let temp = memory.get(cpu.pop_stack(), 3);
        cpu.set_processor_status(temp);
    }
    fn estimated_cycles(&self) -> u8 {
        4
    }
}
