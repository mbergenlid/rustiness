#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;

fn create_ppu() -> PPU {
    let memory = external_memory!(
            0x3F10 => 0x1F, //Black
            0x3F11 => 0x20, //White
            0x3F13 => 0x0B, //(0x00,0x3F,0x17)

            0x3F14 => 0xFF, //Invalid
            0x3F15 => 0x17, //(0xCB,0x4F,0x0F)
            0x3F17 => 0x3B  //(0xB3,0xFF,0xCF)
        );
    let mut ppu = PPU::new(box memory);
    ppu.set_ppu_ctrl(0x08);

    let mut sprites = [0; 64*4];
    sprites[0..4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    ppu.load_sprites(&sprites);

    ppu.load(
        0x1010,
        &[
            //Pattern table 0
            //Layer 1
            0b11111111, 0b11111111, 0b11000011, 0b11000011, 0b11000011, 0b11000011, 0b11111111, 0b11111111,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,

            //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
        ]
    );

    return ppu;
}

#[test]
fn test_basic_sprite_rendering() {
    let mut ppu = create_ppu();

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

    assert_pixels(
        &[
/* tile 1 */ 255,255,255, 255,255,255, 255,255,255, 255,255,255, 255,255,255, 255,255,255, 255,255,255, 255,255,255,
        ],
        &pixel_buffer[0..8*3]
    );
}

use std::fmt::format;
trait PixelDebug {
    fn debug(&self) -> String;
}
impl <'a> PixelDebug for &'a [u8] {
    fn debug(&self) -> String {
        let mut i = 0;
        let mut string = String::new();
        while i < self.len() {
            string = string + &format(format_args!("({},{},{})", self[i], self[i+1], self[i+2]));
            i += 3;
            if i % 24 == 0 {
                string = string + "\n";
            }
        }
        return string;
    }
}

pub fn assert_pixels(expected: &[u8], actual: &[u8]) {
    assert_eq!(expected == actual, true, "Expected\n{}\nbut was\n{}\n", expected.debug(), actual.debug());
}
