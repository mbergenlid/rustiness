extern crate nes;

use audio_device;
use controller;

use nes::input::standard_controller::StandardController;
use nes::memory::Memory;
use nes::ppu::screen::ScreenMock;
use nes::NES;

use nes::borrow::MutableRef;

pub fn test(rom_file: &str) {
    let controller = controller::FakeController::new();
    let standard_controller = StandardController::new(&controller);

    let screen = box ScreenMock::new();
    let mut nes = NES::from_file(
        rom_file,
        MutableRef::Box(box standard_controller),
        audio_device::AudioDevice {},
        screen,
    );

    while nes.memory.get(0x6000, 0) == 0 && nes.cycle_count < 10000000 {
        nes.execute();
    }
    println!("Test started");
    while nes.memory.get(0x6000, 0) == 0x80 && nes.cycle_count < 10000000 {
        nes.execute();
    }

    if nes.cycle_count >= 10000000 {
        panic!("{} Timed out!", rom_file);
    }

    for _ in 0..29781 {
        nes.execute();
    }

    match nes.memory.get(0x6000, 0) {
        0x00 => {}
        code => {
            println!("{}", rom_file);
            let base_address = 0x2000;
            for row in 0..30 {
                for col in 0..32 {
                    let tile = nes
                        .ppu
                        .borrow()
                        .memory()
                        .get(base_address + row * 32 + col, 0);
                    print!("{}", tile as char);
                }
                println!("");
            }
            panic!("Failed with code {:x}", code);
        }
    }
}
