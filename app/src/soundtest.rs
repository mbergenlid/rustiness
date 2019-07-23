use nes_sdl2::SDL2;

use nes::memory::{BasicMemory, Memory};
use nes::sound::registers::*;
use nes::sound::APU;
use nes::Clock;

pub fn start() {
    let sdl = SDL2::new();
    let audio = sdl.audio();
    let mut apu = APU::new(audio, 100);
    let square1 = apu.square1();
    let mut cpu_memory = cpu_memory!(
        box BasicMemory::new(),
        0x4000 => MutableRef::Box(box Register1(square1.clone())),
        0x4002 => MutableRef::Box(box Register3(square1.clone())),
        0x4003 => MutableRef::Box(box Register4(square1.clone()))
    );

    {
        cpu_memory.set(0x4000, 0x84, 0);
        cpu_memory.set(0x4002, 0xAA, 0);
        cpu_memory.set(0x4003, 0b0000_1001, 0);
    }

    let mut clock = Clock::start();

    for _ in 1..1000000 {
        apu.update(2);
        clock.tick(2);
    }
}
