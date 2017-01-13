use addressing::AddressingMode;
use cpu::CPU;
use memory::Memory;

pub fn adc(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.add_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn and(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.and_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn or(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    cpu.or_accumulator(memory.get(addressing_mode.operand_address));
}

pub fn asl_accumulator(cpu: &mut CPU) {
    cpu.asl_accumulator();
}

pub fn asl(addressing_mode: AddressingMode, cpu: &mut CPU, memory: &mut Memory) {
    let new_value = cpu.arithmetic_shift_left(memory.get(addressing_mode.operand_address));
    memory.set(addressing_mode.operand_address, new_value);
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