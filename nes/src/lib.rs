
pub mod cpu;
#[macro_use] pub mod memory;
mod opcodes;
mod instructions;
pub mod addressing;
pub mod ppu;

use cpu::CPU;
use memory::Memory;

pub struct NES {
    cpu: CPU,
    op_codes: opcodes::OpCodes,
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

        //One cycle: 500 ns,
        println!("Took {} ns", start.elapsed().subsec_nanos());
        panic!("Took {} ms", start.elapsed().subsec_nanos());

    }
}