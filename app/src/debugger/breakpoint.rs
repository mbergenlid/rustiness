extern crate getopts;
use self::getopts::{Fail, Options};

use debugger::opcodes;
use nes::ppu::PPU;
use nes::{cpu, memory};
use std::rc::Rc;
use std::result;

type Result = result::Result<Box<dyn BreakPoint>, Fail>;
pub trait BreakPoint {
    fn breakpoint(&self, cpu: &cpu::CPU, ppu: &PPU, memory: &dyn memory::Memory) -> bool;
}

impl dyn BreakPoint {
    pub fn parse(command: &[String], opcodes: Rc<OpCodes>) -> Result {
        let mut opts = Options::new();
        opts.optopt("l", "", "Address to break at", "address");
        opts.optopt("a", "", "Memory location", "address");
        opts.optopt("v", "", "VRAM value", "hex value");

        let matches = match opts.parse(command) {
            Ok(m) => m,
            Err(f) => return Err(f),
        };

        let location_option = matches.opt_str("l").and_then(|option| parse_hex(&option));
        let address_access_option = matches.opt_str("a").and_then(|option| parse_hex(&option));
        let vram_value = matches.opt_str("v").and_then(|option| parse_hex(&option));
        let mut vector: Vec<Box<dyn BreakPoint>> = vec![];
        if location_option.is_some() {
            vector.push(box location_option.unwrap());
        }
        if address_access_option.is_some() {
            vector.push(box MemoryAccess(address_access_option.unwrap(), opcodes));
        }
        if vram_value.is_some() {
            vector.push(box VRAMValue(vram_value.unwrap()));
        }
        if vector.len() > 0 {
            Ok(box And(vector))
        } else {
            Ok(box 0)
        }
    }
}

fn parse_hex(string: &String) -> Option<u16> {
    let mut value: u16 = 0;
    for c in string.chars() {
        let digit = c as u16;
        if digit >= 0x30 && digit <= 0x39 {
            value = value * 16 + (digit - 0x30);
        } else if digit >= 0x41 && digit <= 0x46 {
            value = value * 16 + (digit - 0x41 + 10);
        } else {
            return None;
        }
    }
    return Some(value);
}

struct And(Vec<Box<dyn BreakPoint>>);
impl BreakPoint for And {
    fn breakpoint(&self, cpu: &cpu::CPU, ppu: &PPU, memory: &dyn memory::Memory) -> bool {
        for b in self.0.iter() {
            if !b.breakpoint(cpu, ppu, memory) {
                return false;
            }
        }
        return true;
    }
}

impl BreakPoint for Vec<Box<dyn BreakPoint>> {
    fn breakpoint(&self, cpu: &cpu::CPU, ppu: &PPU, memory: &dyn memory::Memory) -> bool {
        for b in self.iter() {
            if b.breakpoint(cpu, ppu, memory) {
                return true;
            }
        }
        return false;
    }
}

impl BreakPoint for u16 {
    fn breakpoint(&self, cpu: &cpu::CPU, _: &PPU, _: &dyn memory::Memory) -> bool {
        cpu.program_counter() == *self
    }
}

use self::opcodes::OpCodes;
pub struct MemoryAccess(u16, Rc<OpCodes>);

impl BreakPoint for MemoryAccess {
    fn breakpoint(&self, cpu: &cpu::CPU, _: &PPU, memory: &dyn memory::Memory) -> bool {
        let address = self.1.addressing_mode(cpu, memory).operand_address;
        address == self.0
    }
}

struct VRAMValue(u16);
impl BreakPoint for VRAMValue {
    fn breakpoint(&self, _: &cpu::CPU, ppu: &PPU, _: &dyn memory::Memory) -> bool {
        ppu.vram() == self.0
    }
}

#[cfg(test)]
mod test {
    use super::BreakPoint;
    use nes::cpu::opcodes;
    use nes::cpu::CpuBuilder;
    use nes::memory::BasicMemory;
    use nes::ppu::ppumemory::PPUMemory;
    use nes::ppu::PPU;

    use debugger::opcodes::OpCodes;
    use std::rc::Rc;

    #[test]
    fn address_breakpoint() {
        let break_point = BreakPoint::parse(
            &[String::from("-l"), String::from("C013")],
            Rc::new(OpCodes::new()),
        )
        .unwrap();
        let cpu = CpuBuilder::new().program_counter(0xC013).build();
        let ppu = PPU::new(PPUMemory::no_mirroring());
        assert_eq!(
            break_point.breakpoint(&cpu, &ppu, &BasicMemory::new()),
            true
        );
    }

    #[test]
    fn operand_address_breakpoint() {
        let break_point = BreakPoint::parse(
            &[String::from("-a"), String::from("2")],
            Rc::new(OpCodes::new()),
        )
        .unwrap();
        let cpu = CpuBuilder::new().program_counter(0x8000).build();
        let memory = memory!(
            0x8000 => opcodes::ADC_ZERO_PAGE,
            0x8001 => 0x02
        );
        let ppu = PPU::new(PPUMemory::no_mirroring());
        assert_eq!(break_point.breakpoint(&cpu, &ppu, &memory), true);
    }

    #[test]
    fn multiple_breakpoints() {
        let args: Vec<String> = ["-a", "2", "-l", "8002"]
            .iter()
            .map(|&s| String::from(s))
            .collect();
        let break_point = BreakPoint::parse(args.as_slice(), Rc::new(OpCodes::new())).unwrap();
        let cpu = CpuBuilder::new().program_counter(0x8002).build();
        let memory = memory!(
            0x8002 => opcodes::ADC_ZERO_PAGE,
            0x8003 => 0x01,
            0x8004 => opcodes::ADC_ZERO_PAGE,
            0x8005 => 0x02
        );
        let ppu = PPU::new(PPUMemory::no_mirroring());
        assert_eq!(break_point.breakpoint(&cpu, &ppu, &memory), false);
    }
}
