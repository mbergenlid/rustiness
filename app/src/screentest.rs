use nes::ppu::screen::{Screen, Pattern, Tile};
use gliumscreen::GliumScreen;

use std;
use std::time::Duration;


pub fn start() {
    let mut screen = GliumScreen::new(4);

    screen.set_universal_background(0x3F);
    screen.update_palette(0, 1, 0x00);
    screen.update_palette(0, 2, 0x00);
    screen.update_palette(0, 3, 0x20);

    screen.update_palette(1, 1, 0x00);
    screen.update_palette(1, 2, 0x00);
    screen.update_palette(1, 3, 0x15);

    let mut patterns = vec!(
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
        }
    );
    for _ in 0..510 {
        patterns.push(Pattern {
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
        });
    }

    screen.update_patterns(&patterns);

    screen.update_tile(0, 0, &Tile { pattern_index: 1, palette_index: 0 });
    screen.update_tile(0, 1, &Tile { pattern_index: 1, palette_index: 1 });

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

    for row in 0..32 {
        for col in 0..30 {
            screen.draw();

            screen.update_tile(col, row, &Tile { pattern_index: 1, palette_index: 0});
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}