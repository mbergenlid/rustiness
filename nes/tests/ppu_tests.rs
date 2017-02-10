#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::PixelBuffer;
use nes::ppu::PPU;

#[test]
fn draw_buffer_from_name_table_1() {
    let memory = external_memory!(
            0x3F00 => 0x1F, //Black
            0x3F01 => 0x20, //White
            0x3F03 => 0x0B, //(0x00, 0x3F, 0x17)

            0x3F04 => 0xFF, //Invalid
            0x3F05 => 0x0B, //(0x00, 0x3F, 0x17)

            0x2000 => 0x00, //pattern 0 (palette 0)
            0x2001 => 0x01, //pattern 1 (palette 0)
            0x2002 => 0x00, //pattern 0 (palette 1)

            0x23C0 => 0b00_00_01_00
        );
    let mut ppu = PPU::new(box memory);

    ppu.load(
        0x0000,
        &[
            //Pattern table 0
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,

            //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
        ]
    );

    let mut pixel_buffer = [0; 256*240*3];
    {
        let mut buf = PixelBuffer{ buffer: &mut pixel_buffer, pitch: 256*3, scale: 1 };
        ppu.draw_buffer(&mut buf);
    }

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 255,255,255, 0,0,0, 0,0,0,
        ],
        &pixel_buffer[0..24]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 0,0,0
        ],
        &pixel_buffer[768..(768+24)]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0
        ],
        &pixel_buffer[8*3..(8*3+24)]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0
        ],
        &pixel_buffer[16*3..(16*3+24)]
    );
}

#[test]
fn draw_buffer_from_name_table_2() {
    let memory = external_memory!(
            0x3F00 => 0x1F, //Black
            0x3F01 => 0x20, //White
            0x3F03 => 0x0B, //(0x00, 0x3F, 0x17)

            0x3F04 => 0xFF, //Invalid
            0x3F05 => 0x0B, //(0x00, 0x3F, 0x17)

            0x2000 => 0x17, //some unknown pattern
            0x2001 => 0x17, //some unknown pattern
            0x2002 => 0x17, //some unknown pattern

            0x2400 => 0x00, //pattern 0 (palette 0)
            0x2401 => 0x01, //pattern 1 (palette 0)
            0x2402 => 0x00, //pattern 0 (palette 1)

            0x27C0 => 0b00_00_01_00
        );
    let mut ppu = PPU::new(box memory);

    ppu.load(
        0x0000,
        &[
            //Pattern table 0
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,

            //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
        ]
    );

    ppu.set_ppu_ctrl(0x01);

    let mut pixel_buffer = [0; 256*240*3];
    {
        let mut buf = PixelBuffer{ buffer: &mut pixel_buffer, pitch: 256*3, scale: 1 };
        ppu.draw_buffer(&mut buf);
    }

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 255,255,255, 0,0,0, 0,0,0,
        ],
        &pixel_buffer[0..24]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 0,0,0
        ],
        &pixel_buffer[768..(768+24)]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0
        ],
        &pixel_buffer[8*3..(8*3+24)]
    );

    assert_pixels(
        &[
            0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0
        ],
        &pixel_buffer[16*3..(16*3+24)]
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

fn assert_pixels(expected: &[u8], actual: &[u8]) {
    assert_eq!(expected == actual, true, "Expected\n{}\nbut was\n{}\n", expected.debug(), actual.debug());
}