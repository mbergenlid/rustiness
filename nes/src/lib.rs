#![feature(box_syntax)]
#[macro_use] pub mod memory;
pub mod cpu;
pub mod ppu;
pub mod ines;
pub mod input;
pub mod borrow;
pub mod sound;

use cpu::CPU;
use memory::{CPUMemory, Memory};
use ppu::PPU;
use ppu::screen::Screen;

use std::rc::Rc;
use std::cell::RefCell;

use sound::registers::{Register1, Register3, Register4};
use sound::AudioDevice;
use sound::APU;


const NANOS_PER_CLOCK_CYCLE: u32 = 559;

pub struct NES<'a, T, A>
    where T: Screen + Sized, A: AudioDevice + Sized
{
    pub cpu: CPU,
    pub cycle_count: u64,
    pub ppu: Rc<RefCell<PPU>>,
    pub apu: APU<A>,
    pub op_codes: opcodes::OpCodes,
    pub screen: Box<T>,
    pub memory: CPUMemory<'a>,

    pub clock: Clock,
}

use ines::mapper;
use borrow::MutableRef;
use cpu::instructions::Instruction;
use cpu::instructions;
use cpu::opcodes;
use cpu::addressing;

impl <'a, T, A> NES<'a, T, A> where T: Screen + Sized, A: AudioDevice + Sized {

    pub fn from_file(file: &str, controller: MutableRef<'a, MemoryMappedIO>, audio: A, screen: Box<T>) -> NES<'a, T, A> {
        let mapper = mapper::from_file(file);
        NES::new(mapper, controller, audio, screen)
    }

    pub fn new(mapper: mapper::Mapper, controller: MutableRef<'a, MemoryMappedIO>, audio: A, screen: Box<T>) -> NES<'a, T, A> {
        let memory = mapper.cpu_memory;

        let ppu = Rc::new(RefCell::new(PPU::new(mapper.ppu_memory)));

        let apu = APU::new(audio, 500);

        let cpu_start = {
            let lsbs: u8 = memory.get(0xFFFC, 0);
            let msbs: u8 = memory.get(0xFFFD, 0);
            (msbs as u16) << 8 | lsbs as u16
        };
        let cpu_memory = CPUMemory::default(memory, ppu.clone(), &apu, Some(controller));
        NES {
            cpu: CPU::new(cpu_start),
            cycle_count: 0,
            ppu: ppu.clone(),
            apu: apu,
            op_codes: opcodes::OpCodes::new(),
            screen: screen,
            memory: cpu_memory,
            clock: Clock::start(),
        }
    }

    pub fn execute(&mut self) {
        let cycles = self.op_codes.execute_instruction(&mut self.cpu, &mut self.memory);
        let nmi = self.ppu.borrow_mut().sync(cycles as u32, self.screen.as_mut());

        if cfg!(feature = "sound") {
            self.apu.update(cycles);
        }
        self.cycle_count += cycles as u64;
        self.clock.tick(cycles as u32);

        if nmi {
            let nmi_instruction = instructions::NMI::new();
            let cycles = nmi_instruction.estimated_cycles();
            nmi_instruction.execute(&mut self.cpu, &mut self.memory);
            self.ppu.borrow_mut().sync(cycles as u32, self.screen.as_mut());
            self.cycle_count += cycles as u64;
        }
    }

    #[inline]
    pub fn resume(&mut self) {
        self.clock = Clock::start();
    }
}

use ppu::ppuregisters::*;

use memory::MemoryMappedIO;
impl <'a> CPUMemory<'a>  {
    pub fn default<A>(
        memory: Box<Memory>,
        ppu: Rc<RefCell<PPU>>,
        apu: &APU<A>,
        controller: Option<MutableRef<'a, MemoryMappedIO>>
    ) -> CPUMemory<'a> where A: AudioDevice + Sized {
        //apu: &mut APU
        cpu_memory!(
            memory,
            0x2000 => MutableRef::Box(box PPUCtrl(ppu.clone())),
            0x2001 => MutableRef::Box(box PPUMask(ppu.clone())),
            0x2002 => MutableRef::Box(box PPUStatus(ppu.clone())),
            0x2003 => MutableRef::Box(box OAMAddress(ppu.clone())),
            0x2004 => MutableRef::Box(box OAMData(ppu.clone())),
            0x2005 => MutableRef::Box(box PPUScroll(ppu.clone())),
            0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
            0x2007 => MutableRef::Box(box PPUData(ppu.clone())),

            0x4000 => MutableRef::Box(box Register1(apu.square1())),
            0x4002 => MutableRef::Box(box Register3(apu.square1())),
            0x4003 => MutableRef::Box(box Register4(apu.square1())),
            0x4004 => MutableRef::Box(box Register1(apu.square2())),
            0x4006 => MutableRef::Box(box Register3(apu.square2())),
            0x4007 => MutableRef::Box(box Register4(apu.square2())),

            0x4014 => MutableRef::Box(box OAMDMA(ppu.clone())),
            0x4016 => controller.unwrap_or_else(|| MutableRef::Box(box ()))
        )
    }
}

impl MemoryMappedIO for () {
    fn read(&self, _: &Memory) -> u8 { 0 }
    fn write(&mut self, _: &mut Memory, _: u8) { }
}


use std::thread::sleep;
use std::time::{Instant, Duration};

pub struct Clock {
    start: Instant,
    should_have_elapsed: Duration,
    total_sleep_time: Duration
}

impl Clock {
    pub fn start() -> Clock {
        Clock {
            start: Instant::now(),
            should_have_elapsed: Duration::new(0,0),
            total_sleep_time: Duration::new(0,0)
        }
    }

    pub fn tick(&mut self, cycles: u32) {
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
        let expected_max_duration = Duration::new(0, 10500*NANOS_PER_CLOCK_CYCLE);
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
}
