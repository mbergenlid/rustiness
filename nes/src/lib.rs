
pub mod cpu;
#[macro_use] pub mod memory;
pub mod opcodes;
mod instructions;
pub mod addressing;
pub mod ppu;
pub mod ines;

use cpu::CPU;
use memory::Memory;

pub struct NES {
    pub cpu: CPU,
    pub op_codes: opcodes::OpCodes,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: CPU::new(),
            op_codes: opcodes::OpCodes::new(),
        }
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        self.op_codes.execute_instruction(&mut self.cpu, memory);
    }

}

#[cfg(test)]
mod test {

    use std::time::Instant;
    use memory::Memory;

    #[test]
    fn timing_test() {
        let mut execution_time_total = 0;
        //One cycle: 500 ns,
        for _ in 1..100 {
            let mut memory = memory!(
                0x00A5 => 0xF0,
                0x00A6 => 0x10,
                //ADC $05
                0x8000 => 0x69,
                0x8001 => 0x05,
                0x8002 => 0x10
            );

            let mut nes = super::NES::new();
            let start = Instant::now();
            nes.execute(&mut memory);

            execution_time_total += start.elapsed().subsec_nanos();
        }

        assert!(execution_time_total/100 < 500, "Execution time more {} >= {}ns", execution_time_total/100, 500);
    }
}