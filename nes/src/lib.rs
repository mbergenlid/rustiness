#![feature(box_syntax)]
pub mod cpu;
#[macro_use] pub mod memory;
pub mod opcodes;
mod instructions;
pub mod addressing;
pub mod ppu;
pub mod ines;

use cpu::CPU;
use memory::{BasicMemory, CPUMemory};
use ppu::PPU;

pub struct NES {
    pub cpu: CPU,
    pub cycle_count: u64,
    pub ppu: PPU,
    pub op_codes: opcodes::OpCodes,
}

impl NES {
    pub fn new(ppu: PPU) -> NES {
        NES {
            cpu: CPU::new(),
            cycle_count: 0,
            ppu: ppu,
            op_codes: opcodes::OpCodes::new(),
        }
    }

    pub fn execute(&mut self, memory: &mut BasicMemory) {
        let cycles = self.op_codes.execute_instruction(&mut self.cpu, &mut CPUMemory::new(&mut self.ppu, memory));
        let nmi = self.ppu.update(cycles as u32);
        self.cycle_count += cycles as u64;

        if nmi {
            let cycles =
                instructions::nmi(&mut self.cpu, &mut CPUMemory::new(&mut self.ppu, memory)) as u64;
            self.ppu.update(cycles as u32);
            self.cycle_count += cycles;
        }
    }

}

#[cfg(test)]
mod test {

//    use std::time::Instant;
//    use memory::Memory;
//    use ppu::PPU;
//    use memory::BasicMemory;
//    use ppu::screen::ScreenMock;
//
//    impl super::NES {
//        pub fn without_ppu() -> super::NES {
//            super::NES::new(PPU::new(
//                box (BasicMemory::new()),
//                box (ScreenMock::new())
//            ))
//        }
//    }

//    #[test]
//    fn timing_test() {
//        let mut execution_time_total = 0;
//        //One cycle: 500 ns,
////        for _ in 1..100 {
//            let mut memory = memory!(
//                0x00A5 => 0xF0,
//                0x00A6 => 0x10,
//                //ADC $05
//                0x8000 => 0x69,
//                0x8001 => 0x05,
//                0x8002 => 0x10
//            );
//
//            let mut nes = super::NES::without_ppu();
//            let start = Instant::now();
//            nes.execute(&mut memory);
//
//            execution_time_total += start.elapsed().subsec_nanos();
////        }
//
//        assert!(execution_time_total < 500, "Execution time more {}ns >= {}ns", execution_time_total, 500);
//    }
}