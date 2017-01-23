use nes::ppu::screen::{Screen2, Pattern, Tile};
use gliumscreen::GliumScreen2;

use std;
use std::time::Duration;


pub fn start() {
    let mut screen = GliumScreen2::new(4);

    screen.set_universal_background(0x3F);
    screen.update_palette_0(1, 0x00);
    screen.update_palette_0(2, 0x00);
    screen.update_palette_0(3, 0x20);

    screen.update_palette_1(1, 0x00);
    screen.update_palette_1(2, 0x00);
    screen.update_palette_1(3, 0x15);

    screen.update_patterns(
        &[
            Pattern {
                data: [
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                    [0,0,0,0,0,0,0,0],
                ]
            }, 
            Pattern {
                data: [
                    [0,0,0,3,3,3,0,0],
                    [0,0,3,3,0,0,3,0],
                    [0,0,3,3,3,0,0,0],
                    [0,0,0,3,3,3,0,0],
                    [0,0,0,0,3,3,3,0],
                    [0,0,3,0,0,3,3,0],
                    [0,0,0,3,3,3,0,0],
                    [0,0,0,0,0,0,0,0],
                ]
        }]
    );

    screen.update_tile(0, 0, &Tile { pattern_index: 1, palette_index: 0 });
    screen.update_tile(0, 1, &Tile { pattern_index: 1, palette_index: 1 });

    screen.draw();
//    let mut ppu = PPU::new(
//        box (external_memory!(
//                    //Pattern table
//                        //Layer 1
//                    0x0000 => 0b00011100,
//                    0x0001 => 0b00110010,
//                    0x0002 => 0b00111000,
//                    0x0003 => 0b00011100,
//                    0x0004 => 0b00001110,
//                    0x0005 => 0b00100110,
//                    0x0006 => 0b00011100,
//                    0x0007 => 0b00000000,
//                        //Layer 2
//                    0x0008 => 0b00011100,
//                    0x0009 => 0b00110010,
//                    0x000A => 0b00111000,
//                    0x000B => 0b00011100,
//                    0x000C => 0b00001110,
//                    0x000D => 0b00100110,
//                    0x000E => 0b00011100,
//                    0x000F => 0b00000000,
//                    //Pattern table end
//
//                    //Name table
//                    0x2000 => 0x0000, //points to pattern table
//                        //Attribute table
//                    0x23C0 => 0x0000,  //points to colour palette
//
//                    //PPU Palettes
//                    0x3F00 => 0x3F,
//                    0x3F01 => 0x00,
//                    0x3F02 => 0x00,
//                    0x3F03 => 0x20
//                )),
//        screen
//    );
//    ppu.draw();

    loop {
        std::thread::sleep(Duration::from_millis(500));
    }
}