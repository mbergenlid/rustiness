use nes::{cpu, memory};

pub trait BreakPoint {
    fn breakpoint(&self, cpu: &cpu::CPU, memory: &memory::Memory) -> bool;
}

impl BreakPoint for Vec<Box<BreakPoint>> {
    fn breakpoint(&self, cpu: &cpu::CPU, memory: &memory::Memory) -> bool {
        for b in self.iter() {
            if b.breakpoint(cpu, memory) {
                return true;
            }
        }
        return false;
    }
}

impl BreakPoint for u16 {
    fn breakpoint(&self, cpu: &cpu::CPU, _: &memory::Memory) -> bool {
        cpu.program_counter() == *self
    }
}
