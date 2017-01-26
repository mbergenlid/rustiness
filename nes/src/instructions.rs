use addressing::AddressingMode;
use cpu::CPU;
use memory::Memory;
use cpu;

pub fn adc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.add_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn sbc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.sub_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn inc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let new_value = cpu.increment(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
    3 + addressing_mode.cycles
}

pub fn dec(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.decrement(memory.get(addressing_mode.operand_address));
    3 + addressing_mode.cycles
}

pub fn and(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.and_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn or(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.or_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn eor(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.xor_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn asl_accumulator(cpu: &mut CPU) -> u8 {
    cpu.asl_accumulator();
    2
}

pub fn asl(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let new_value = cpu.arithmetic_shift_left(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
    3 + addressing_mode.cycles
}

pub fn lsr(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let new_value = cpu.logical_shift_right(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
    3 + addressing_mode.cycles
}

pub fn rol(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let new_value = cpu.rotate_left(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
    3 + addressing_mode.cycles
}

pub fn ror(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let new_value = cpu.rotate_right(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
    3 + addressing_mode.cycles
}

pub fn bit(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.bit_test(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn branch(cpu: &mut CPU, memory: &Memory, flag: u8, inverted: bool) -> u8 {
    let condition =
        if inverted {
            !cpu.is_flag_set(flag)
        } else {
            cpu.is_flag_set(flag)
        };

    let branch_distance: i8 = memory.get(cpu.get_and_increment_pc()) as i8;
    if condition {
        cpu.add_program_counter(branch_distance as u16);
        3 //TODO: Should depend on page_crossing
    } else {
        2
    }
}

pub fn jmp(addressing_mode: AddressingMode, cpu: &mut CPU) {
    cpu.set_program_counter(addressing_mode.operand_address);
}

pub fn brk(cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let current_pc = cpu.program_counter();
    memory.set(cpu.push_stack(), (current_pc >> 8) as u8);
    memory.set(cpu.push_stack(), current_pc as u8);
    memory.set(cpu.push_stack(), cpu.processor_status());

    let lsbs: u8 = memory.get(0xFFFE);
    let msbs: u8 = memory.get(0xFFFF);
    cpu.set_program_counter((msbs as u16) << 8 | lsbs as u16);
    cpu.set_flags(cpu::BREAK_FLAG);
    return 7;
}

pub fn rti(cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let processor_status = memory.get(cpu.pop_stack());
    cpu.set_processor_status(processor_status);
    let return_address = memory.get(cpu.pop_stack()) as u16 | (memory.get(cpu.pop_stack()) as u16) << 8;
    cpu.set_program_counter(return_address);
    return 6;
}

pub fn jsr(cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let destination_address = AddressingMode::absolute(cpu, memory).operand_address;
    let current_pc = cpu.program_counter() - 1;
    memory.set(cpu.push_stack(), (current_pc >> 8) as u8);
    memory.set(cpu.push_stack(), current_pc as u8);

    cpu.set_program_counter(destination_address);
    return 6;
}

pub fn rts(cpu: &mut CPU, memory: &mut Memory) -> u8 {
    let return_address = memory.get(cpu.pop_stack()) as u16 | (memory.get(cpu.pop_stack()) as u16) << 8;
    cpu.set_program_counter(return_address + 1);
    return 6;
}

pub fn sta(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    memory.set(addressing_mode.operand_address, cpu.accumulator());
}

pub fn stx(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    memory.set(addressing_mode.operand_address, cpu.register_x());
}

pub fn sty(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    memory.set(addressing_mode.operand_address, cpu.register_y());
}

pub fn ldx(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.load_x(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn ldy(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.load_y(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn lda(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.load_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn cmp(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.cmp_accumulator(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn cpx(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.cmp_register_x(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}

pub fn cpy(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) -> u8 {
    cpu.cmp_register_y(memory.get(addressing_mode.operand_address));
    1 + addressing_mode.cycles
}