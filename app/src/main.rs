#[macro_use]
extern crate nes;
extern crate gliumscreen;

mod debugger;

use nes::ppu::PPU;
use gliumscreen::GliumScreen;

use std::time::Duration;

fn main() {
    debugger::start();
}

fn test_ppu() {
    let mut screen = GliumScreen::new(4);
    let mut ppu = PPU::new(
        Box::new(external_memory!(
                    //Pattern table
                        //Layer 1
                    0x0000 => 0b00011100,
                    0x0001 => 0b00110010,
                    0x0002 => 0b00111000,
                    0x0003 => 0b00011100,
                    0x0004 => 0b00001110,
                    0x0005 => 0b00100110,
                    0x0006 => 0b00011100,
                    0x0007 => 0b00000000,
                        //Layer 2
                    0x0008 => 0b00011100,
                    0x0009 => 0b00110010,
                    0x000A => 0b00111000,
                    0x000B => 0b00011100,
                    0x000C => 0b00001110,
                    0x000D => 0b00100110,
                    0x000E => 0b00011100,
                    0x000F => 0b00000000,
                    //Pattern table end

                    //Name table
                    0x2000 => 0x0000, //points to pattern table
                        //Attribute table
                    0x23C0 => 0x0000,  //points to colour palette

                    //PPU Palettes
                    0x3F00 => 0x3F,
                    0x3F01 => 0x00,
                    0x3F02 => 0x00,
                    0x3F03 => 0x20
                )),
        &mut screen
    );
    ppu.draw();

    loop {
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn performance_test() {
    use std::time::Instant;

    let mut memory = external_memory!(
            0x00A5 => 0xF0,
            0x00A6 => 0x10,
            //ADC $05
            0x8000 => 0x69,
            0x8001 => 0x05,
            0x8002 => 0x10
        );

    let mut nes = nes::NES::new();

    let start = Instant::now();
    nes.execute(&mut memory);

    //One cycle: 500 ns,
    println!("Took {} ns", start.elapsed().subsec_nanos());
}
