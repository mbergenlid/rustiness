#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod fakes;
use nes::opcodes;
use nes::input::standard_controller::StandardController;
use nes::ppu::screen::ScreenMock;
use nes::ppu::ppumemory::PPUMemory;
use nes::NES;
use nes::borrow::MutableRef;
use nes::ines::mapper::Mapper;

#[test]
fn vbl_timing() {
    let memory = memory!(
            0x800B => opcodes::BIT_ABSOLUTE,
            0x800C => 0x02,
            0x800D => 0x20,

            0x800E => opcodes::BRANCH_PLUS,
            0x800F => (-(0xE as i32)) as u8,


            0xFFFC => 0x0B,
            0xFFFD => 0x80
        );
    let controller = fakes::controller::FakeController::new();
    let standard_controller = StandardController::new(&controller);

    let screen = box ScreenMock::new();

    let mut nes = NES::new(
        Mapper { cpu_memory: box memory, ppu_memory: PPUMemory::no_mirroring() },
        MutableRef::Box(box standard_controller),
        fakes::audio_device::AudioDevice {},
        screen
    );

    nes.ppu.borrow_mut().update(27_393, &mut ScreenMock::new());
    nes.execute();
    nes.execute();

    assert_eq!(nes.cpu.program_counter(), 0x8010);

}
