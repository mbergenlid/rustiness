extern crate nes;

use controller;
use audio_device;

use nes::NES;
use nes::memory::Memory;
use nes::ppu::screen::ScreenMock;
use nes::input::standard_controller::StandardController;

use nes::borrow::MutableRef;

pub fn test(rom_file: &str) {

    let controller = controller::FakeController::new();
    let standard_controller = StandardController::new(&controller);

    let screen = box ScreenMock::new();
    let mut nes = NES::from_file(rom_file, MutableRef::Box(box standard_controller), audio_device::AudioDevice {}, screen);


    while nes.memory.get(0x6000) == 0 {
        nes.execute();
    }
    println!("Test started");
    while nes.memory.get(0x6000) == 0x80 {
        nes.execute();
    }

    match nes.memory.get(0x6000) {
        0x00 => {},
        code => panic!("Failed with code {:x}", code),
    }
}
