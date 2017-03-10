#![feature(box_syntax)]
pub mod cpu;
#[macro_use] pub mod memory;
pub mod opcodes;
mod instructions;
pub mod addressing;
pub mod ppu;
pub mod ines;
pub mod input;
pub mod borrow;

use cpu::CPU;
use memory::{BasicMemory, CPUMemory, Memory};
use ppu::PPU;
use ppu::screen::Screen;

use std::rc::Rc;
use std::cell::RefCell;


const NANOS_PER_CLOCK_CYCLE: u32 = 559;

pub struct NES<'a, T>
    where T: Screen + Sized
{
    pub cpu: CPU,
    pub cycle_count: u64,
    pub ppu: Rc<RefCell<PPU>>,
    pub op_codes: opcodes::OpCodes,
    pub screen: Box<T>,
    pub memory: CPUMemory<'a>,

    pub clock: Clock,
}

impl <'a, T> NES<'a, T> where T: Screen + Sized {
    pub fn new(ppu: PPU, memory: Box<BasicMemory>, screen: Box<T>, controller: &'a mut MemoryMappedIO) -> NES<T> {
        let cpu_start = {
            let lsbs: u8 = memory.get(0xFFFC);
            let msbs: u8 = memory.get(0xFFFD);
            (msbs as u16) << 8 | lsbs as u16
        };
        let ppu = Rc::new(RefCell::new(ppu));
        NES {
            cpu: CPU::new(cpu_start),
            cycle_count: 0,
            ppu: ppu.clone(),
            op_codes: opcodes::OpCodes::new(),
            screen: screen,
            memory: CPUMemory::default(memory, ppu, Some(controller)),
            clock: Clock::start(),
        }
    }

    pub fn execute(&mut self) {
        let cycles = self.op_codes.execute_instruction(&mut self.cpu, &mut self.memory);
        let nmi = self.ppu.borrow_mut().update(cycles as u32, self.screen.as_mut());
        self.cycle_count += cycles as u64;
        self.clock.tick(cycles as u32);

        if nmi {
            let cycles =
                instructions::nmi(&mut self.cpu, &mut self.memory) as u64;
            self.ppu.borrow_mut().update(cycles as u32, self.screen.as_mut());
            self.cycle_count += cycles;
        }
    }

    #[inline]
    pub fn resume(&mut self) {
        self.clock = Clock::start();
    }
}

use ppu::ppuregisters::*;

use memory::MemoryMappedIO;
impl <'a> CPUMemory<'a> {
    pub fn default(memory: Box<BasicMemory>, ppu: Rc<RefCell<PPU>>, controller: Option<&'a mut MemoryMappedIO>) -> CPUMemory {
        cpu_memory!(
            memory,
            0x2000 => MutableRef::Box(box PPUCtrl(ppu.clone())),
            0x2001 => MutableRef::Box(box PPUMask(ppu.clone())),
            0x2002 => MutableRef::Box(box PPUStatus(ppu.clone())),
            0x2005 => MutableRef::Box(box PPUScroll(ppu.clone())),
            0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
            0x2007 => MutableRef::Box(box PPUData(ppu.clone())),
            0x4014 => MutableRef::Box(box OAMAddress(ppu.clone())),
            0x4016 => controller.map(|c| MutableRef::Borrowed(c)).unwrap_or_else(|| MutableRef::Box(box ()))
        )
    }
}

impl MemoryMappedIO for () {
    fn read(&self, _: &BasicMemory) -> u8 { 0 }
    fn write(&mut self, _: &mut BasicMemory, _: u8) { }
}


use std::thread::sleep;
use std::time::{Instant, Duration};

pub struct Clock {
    start: Instant,
    should_have_elapsed: Duration,
    total_sleep_time: Duration
}

impl Clock {
    fn start() -> Clock {
        Clock {
            start: Instant::now(),
            should_have_elapsed: Duration::new(0,0),
            total_sleep_time: Duration::new(0,0)
        }
    }

    fn tick(&mut self, cycles: u32) {
        self.should_have_elapsed = self.should_have_elapsed + Duration::new(0, cycles*NANOS_PER_CLOCK_CYCLE);
        let elapsed = self.start.elapsed();
        if self.should_have_elapsed > elapsed {
            let sleep_time = self.should_have_elapsed - elapsed;
            sleep(sleep_time);
            self.total_sleep_time += sleep_time;
        }
    }
}

use std::fmt::{Display, Formatter, Error};
impl Display for Clock {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        let total_time = self.start.elapsed();
        let sleep_percent =
            ((self.total_sleep_time.as_secs()*1000000000 + self.total_sleep_time.subsec_nanos() as u64) as f64) /
                ((total_time.as_secs()*1000000000 + total_time.subsec_nanos() as u64) as f64);
        formatter.write_fmt(
            format_args!(
                "Total time: {}.{:09}s, Idle time: {}.{:09}s, sleep {}%",
                total_time.as_secs(), total_time.subsec_nanos(),
                self.total_sleep_time.as_secs(), self.total_sleep_time.subsec_nanos(),
                sleep_percent*100f64
                ))
    }
}


#[cfg(test)]
mod test {

    use std::time::{Instant, Duration};
    use super::{Clock, NANOS_PER_CLOCK_CYCLE};
    #[test]
    fn clock_test() {
        let start = Instant::now();
        let mut clock = Clock::start();
        clock.tick(10000);

        let elapsed = start.elapsed();
        let expected_duration = Duration::new(0, 10000*NANOS_PER_CLOCK_CYCLE);
        assert!(elapsed >= expected_duration, "Should take at least {:?} but took {:?}", expected_duration, elapsed);
        let expected_max_duration = Duration::new(0, 100000*NANOS_PER_CLOCK_CYCLE);
        assert!(elapsed <= expected_max_duration, "Should take at most {:?} but took {:?}", expected_max_duration, elapsed);
    }

    #[test]
    fn clock_test_2() {
        let start = Instant::now();
        let mut clock = Clock::start();
        for _ in 0..5000 {
            clock.tick(2);
        }

        let elapsed = start.elapsed();
        let expected_duration = Duration::new(0, 10000*NANOS_PER_CLOCK_CYCLE);
        assert!(elapsed >= expected_duration, "Should take at least {:?} but took {:?}", expected_duration, elapsed);
        let expected_max_duration = Duration::new(0, 10100*NANOS_PER_CLOCK_CYCLE);
        assert!(elapsed <= expected_max_duration, "Should take at most {:?} but took {:?}", expected_max_duration, elapsed);
    }

    #[test]
    #[ignore]
    fn clock_should_handle_large_tick() {
        let start = Instant::now();
        let mut clock = Clock::start();
        for _ in 0..5_000_000 {
            clock.tick(2);
        }

        let elapsed = start.elapsed();
        let expected_duration = Duration::new(5, 590_000_000);
        assert!(elapsed >= expected_duration, "Should take at least {:?} but took {:?}", expected_duration, elapsed);
        let expected_max_duration = Duration::new(5, 595_900_000);
        assert!(elapsed <= expected_max_duration, "Should take at most {:?} but took {:?}", expected_max_duration, elapsed);
    }
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
