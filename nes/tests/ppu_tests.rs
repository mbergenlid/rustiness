#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::ScreenMock;
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
    ppu.set_ppu_ctrl(0x08);

    load_ppu_patterns(&mut ppu);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = &screen.screen_buffer;

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
    load_ppu_patterns(&mut ppu);

    ppu.set_ppu_ctrl(0x08 | 0x01);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

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
fn draw_buffer_from_name_table_1_with_scrolling_y() {
    let memory = external_memory!(
            0x3F00 => 0x1F, //Black
            0x3F01 => 0x20, //White
            0x3F03 => 0x0B, //(0x00, 0x3F, 0x17)

            0x3F04 => 0xFF, //Invalid
            0x3F05 => 0x17, //(0xCB, 0x4F, 0x0F)
            0x3F07 => 0x3B, //(0xB3,0xFF,0xCF)

            0x2000 => 0x00, //pattern 0 (palette 0) Should not be visible
            0x2001 => 0x01, //pattern 1 (palette 0) Should not be visible
            0x2002 => 0x00, //pattern 0 (palette 1) Should not be visible

            0x2020 => 0x01, //pattern 1 (palette 1) Should not be visible
            0x2021 => 0x00, //pattern 0 (palette 1) Should not be visible
            0x2022 => 0x00, //pattern 0 (palette 0) Should not be visible

            0x23C0 => 0b00_00_00_01,

            0x2400 => 0x00 //pattern 0 (palette 0)
        );
    let mut ppu = PPU::new(box memory);
    ppu.set_ppu_ctrl(0x08);
    load_ppu_patterns(&mut ppu);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(8); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 0,0,0, 0xB3,0xFF,0xCF, 0xB3,0xFF,0xCF, 0xB3,0xFF,0xCF, 0,0,0, 0,0,0,
    /* tile 2 */ 0,0,0, 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0,0,0, 0,0,0,
    /* tile 3 */ 0,0,0, 0,0,0, 0,0,0, 255,255,255, 255,255,255, 255,255,255, 0,0,0, 0,0,0,
            ],
            &pixel_buffer[0..(16*3+24)]
        );
        assert_pixels(
            &[
                0,0,0, 0,0,0, 0xB3,0xFF,0xCF, 0xB3,0xFF,0xCF, 0,0,0, 0,0,0, 0xB3,0xFF,0xCF, 0,0,0
            ],
            &pixel_buffer[768..(768+24)] //line 2
        );
    }
    ppu.set_scroll(0);
    ppu.set_scroll(9);
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
    /* tile 1 */ 0,0,0, 0,0,0, 0xB3,0xFF,0xCF, 0xB3,0xFF,0xCF, 0,0,0, 0,0,0, 0xB3,0xFF,0xCF, 0,0,0,
    /* tile 2 */ 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0xCB,0x4F,0x0F, 0,0,0, 0,0,0, 0xCB,0x4F,0x0F, 0,0,0,
    /* tile 3 */ 0,0,0, 0,0,0, 255,255,255, 255,255,255, 0,0,0, 0,0,0, 255,255,255, 0,0,0,
            ],
            &pixel_buffer[0..(16*3+24)]
        );
    }
}


#[test]
fn draw_buffer_from_name_table_1_with_scrolling_x() {
    let memory = external_memory!(
            0x3F00 => 0x1F, //Black
            0x3F01 => 0x20, //White
            0x3F03 => 0x0B, //(0x00, 0x3F, 0x17)

            0x3F04 => 0xFF, //Invalid
            0x3F05 => 0x0B, //(0x00, 0x3F, 0x17)

            0x2000 => 0x00, //pattern 0 (palette 0)
            0x2001 => 0x01, //pattern 1 (palette 0)
            0x2002 => 0x00, //pattern 0 (palette 1)

            0x23C0 => 0b00_00_01_00,

            0x2400 => 0x00 //pattern 0 (palette 0)
        );
    let mut ppu = PPU::new(box memory);
    ppu.set_ppu_ctrl(0x08);
    load_ppu_patterns(&mut ppu);
    ppu.set_scroll(8); //x scroll
    ppu.set_scroll(0); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
                0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0,
                0,0,0, 0,0,0, 0,0,0, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0x00,0x3F,0x17, 0,0,0, 0,0,0,
            ],
            &pixel_buffer[0..8*3*2]
        );
    }
}

//mod donkey_kong_title_screen;
//mod donkey_kong_title_screen_result;
//
//#[test]
//fn test_donkey_kong_title_screen() {
//    use nes::memory::BasicMemory;
//    let mut ppu = PPU::new(box BasicMemory::new());
//    donkey_kong_title_screen::load_ppu(&mut ppu);
//
//    use nes::ppu::screen::PixelBuffer;
//    let mut buffer = [0; 256*240*3];
//    ppu.draw_buffer(&mut PixelBuffer { buffer: &mut buffer, pitch: 256*3, scale: 1});
//
//    for y in 0..240 {
//        for x in 0..256 {
//            let expected_colour = donkey_kong_title_screen_result::DONKEY_KONG_TITLE_SCREEN[y*256 + x];
//            let actual_colour = (buffer[y*256*3 + x*3], buffer[y*256*3 + x*3+1], buffer[y*256*3 + x*3+2]);
//
//            assert_eq!(
//                expected_colour,
//                actual_colour,
//                "Pixel {},{} is wrong\nExpected {:?}\nbut was {:?}\n",
//                x,
//                y,
//                expected_colour,
//                actual_colour
//            );
//        }
//    }
//}

fn load_ppu_patterns(ppu: &mut PPU) {
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
