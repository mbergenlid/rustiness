use addressing::AddressingMode;
use cpu::CPU;
use memory::Memory;

pub fn adc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.add_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn sbc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.sub_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn inc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.increment(memory.get(addressing_mode.operand_address));
}

pub fn dec(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.decrement(memory.get(addressing_mode.operand_address));
}

pub fn and(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.and_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn or(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.or_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn eor(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.xor_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn asl_accumulator(cpu: &mut CPU) {
    cpu.asl_accumulator();
}

pub fn asl(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    let new_value = cpu.arithmetic_shift_left(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
}

pub fn lsr(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    let new_value = cpu.logical_shift_right(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
}

pub fn rol(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    let new_value = cpu.rotate_left(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
}

pub fn ror(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    let new_value = cpu.rotate_right(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
}

pub fn bit(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.bit_test(memory.get(addressing_mode.operand_address));
}

pub fn branch(cpu: &mut CPU, memory: &Memory, flag: u8, inverted: bool) {
    let condition =
        if inverted {
            !cpu.is_flag_set(flag)
        } else {
            cpu.is_flag_set(flag)
        };

    let branch_distance: i8 = memory.get(cpu.get_and_increment_pc()) as i8;
    if condition {
        cpu.add_program_counter(branch_distance as u16);
    }
}

pub fn jmp(addressing_mode: AddressingMode, cpu: &mut CPU) {
    cpu.set_program_counter(addressing_mode.operand_address);
}

pub fn jsr(cpu: &mut CPU, memory: &mut Memory) {
    let destination_address = AddressingMode::absolute(cpu, memory).operand_address;
    let current_pc = cpu.program_counter() - 1;
    memory.set(cpu.push_stack(), (current_pc >> 8) as u8);
    memory.set(cpu.push_stack(), current_pc as u8);

    cpu.set_program_counter(destination_address);
}

pub fn rts(cpu: &mut CPU, memory: &mut Memory) {
    let return_address = memory.get(cpu.pop_stack()) as u16 | (memory.get(cpu.pop_stack()) as u16) << 8;
    cpu.set_program_counter(return_address + 1);
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

pub fn ldx(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.load_x(memory.get(addressing_mode.operand_address));
}

pub fn ldy(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.load_y(memory.get(addressing_mode.operand_address));
}

pub fn lda(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.load_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn cmp(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.cmp_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn cpx(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.cmp_register_x(memory.get(addressing_mode.operand_address));
}

pub fn cpy(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.cmp_register_y(memory.get(addressing_mode.operand_address));
}