#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;
use nes::ppu::ppumemory::Mirroring;

fn create_ppu(mirroring: Mirroring) -> PPU {
    let memory = external_memory!(
            0x3F00 => 0x1F, //Black
            0x3F01 => 0x20, //White
            0x3F03 => 0x0B, //(0x00,0x3F,0x17)

            0x3F04 => 0xFF, //Invalid
            0x3F05 => 0x17, //(0xCB,0x4F,0x0F)
            0x3F07 => 0x3B, //(0xB3,0xFF,0xCF)

            0x2000 => 0x01, //pattern 1 (palette 0) 

            0x2400 => 0x02, //pattern 2 (palette 0)

            0x2800 => 0x01, //pattern 1 (palette 1)
            0x2BC0 => 0b00_00_00_01,

            0x2C00 => 0x02, //pattern 2 (palette 1)
            0x2FC0 => 0b00_00_00_01
        );
    let mut ppu = PPU::with_mirroring(box memory, mirroring);
    ppu.load(
        0x0010,
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

    return ppu;
}

#[test]
fn no_mirroring() {
    let mut ppu = create_ppu(Mirroring::NoMirroring);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(5); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0,
            ],
            &pixel_buffer[0..8*3]
        );
        let last_row = 239*256*3;
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0,0,0
            ],
            &pixel_buffer[last_row..last_row+8*3] //last line
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 255,255,255, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0
            ],
            &pixel_buffer[0..8*3]
        );
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17
            ],
            &pixel_buffer[248*3..248*3+8*3]
        );
    }
}

#[test]
fn horizontal_mirroring() {
    let mut ppu = create_ppu(Mirroring::Horizontal);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(5); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0,
            ],
            &pixel_buffer[0..8*3]
        );
        let last_row = 239*256*3;
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0,0,0
            ],
            &pixel_buffer[last_row..last_row+8*3] //last line
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 255,255,255, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0
            ],
            &pixel_buffer[0..8*3]
        );
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 255,255,255, 255,255,255
            ],
            &pixel_buffer[248*3..248*3+8*3]
        );
    }
}

#[test]
fn vertical_mirroring() {
    let mut ppu = create_ppu(Mirroring::Vertical);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(5); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0,
            ],
            &pixel_buffer[0..8*3]
        );
        let last_row = 239*256*3;
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 255,255,255, 0,0,0
            ],
            &pixel_buffer[last_row..last_row+8*3] //last line
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 255,255,255, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0
            ],
            &pixel_buffer[0..8*3]
        );
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17
            ],
            &pixel_buffer[248*3..248*3+8*3]
        );
    }
}

#[test]
fn horizontal_and_vertical_scroll() {
    let mut ppu = create_ppu(Mirroring::NoMirroring);
    ppu.set_ppu_ctrl(0x03); //Pick fourth name table
    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(5); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        //upper left corner
        assert_pixels(
            &[
    /* tile 1 */ 0xB3,0xFF,0xCF, 0xB3,0xFF,0xCF, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,
            ],
            &pixel_buffer[0..8*3]
        );
        //upper right corner
        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0,0,0, 0,0,0,
            ],
            &pixel_buffer[248*3..248*3+8*3]
        );
        //lower left corner
        let last_row = 239*256*3;
        assert_pixels(
            &[
                0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0
            ],
            &pixel_buffer[last_row..last_row+8*3] //last line
        );
        //lower right corner
        println!("{},{},{}", pixel_buffer[184_317], pixel_buffer[184_318], pixel_buffer[184_319]);
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0, 255,255,255,
            ],
            {
                let start = (256*240-8)*3;
                &pixel_buffer[start..start+8*3] //last line
            }
        );
    }
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
