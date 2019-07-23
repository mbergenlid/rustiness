#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod screen;
use nes::memory::SharedMemory;
use nes::ppu::ppumemory::{Mirroring, PPUMemory};
use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;

use screen::{assert_pixels, BACK_DROP, BROWN, GREEN, ORANGE, WHITE};

fn create_ppu(mirroring: Mirroring) -> PPU {
    let memory = memory!(
        0x3F00 => 0x1F, //Black
        0x3F01 => 0x20, //White
        0x3F03 => 0x0B, //(0x00,0x3F,0x17)

        0x3F04 => 0xFF, //Invalid
        0x3F05 => 0x17, //(0xCB,0x4F,0x0F)
        0x3F07 => 0x3B, //(0xB3,0xFF,0xCF)

        0x2000 => 0x01, //pattern 1 (palette 0)
        0x23A0 => 0x01, //pattern 1 (palette 0)

        0x2400 => 0x02, //pattern 2 (palette 0)

        0x2800 => 0x01, //pattern 1 (palette 1)
        0x2BC0 => 0b00_00_00_01,

        0x2C00 => 0x02, //pattern 2 (palette 1)
        0x2FC0 => 0b00_00_00_01
    );
    let mut ppu = PPU::new(PPUMemory::wrap(SharedMemory::wrap(memory), mirroring));
    ppu.set_ppu_ctrl(0x08);
    ppu.load(
        0x0010,
        &[
            //Pattern table 0
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000001, //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000000, //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000000,
        ],
    );

    return ppu;
}

#[test]
fn no_mirroring_no_scroll() {
    let mut ppu = create_ppu(Mirroring::NoMirroring);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(0); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
                /* tile 1 */ BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP,
                BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        let last_row = 239 * 256;
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, WHITE,
            ],
            pixel_buffer,
            last_row..last_row + 8,
        );
    }
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
                /* tile 1 */ BACK_DROP, BACK_DROP, WHITE, BACK_DROP, BACK_DROP, WHITE, WHITE,
                BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        let last_row = 239 * 256;
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, ORANGE, ORANGE, ORANGE, BACK_DROP,
            ],
            pixel_buffer,
            last_row..last_row + 8,
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
                /* tile 1 */ WHITE, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP,
                BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN,
            ],
            pixel_buffer,
            248..248 + 8,
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
                /* tile 1 */ BACK_DROP, BACK_DROP, WHITE, BACK_DROP, BACK_DROP, WHITE, WHITE,
                BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        let last_row = 239 * 256;
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, ORANGE, ORANGE, ORANGE, BACK_DROP,
            ],
            pixel_buffer,
            last_row..last_row + 8,
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
                /* tile 1 */ WHITE, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP,
                BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE,
            ],
            pixel_buffer,
            248..248 + 8,
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
                /* tile 1 */ BACK_DROP, BACK_DROP, WHITE, BACK_DROP, BACK_DROP, WHITE, WHITE,
                BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        let last_row = 239 * 256;
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP,
            ],
            pixel_buffer,
            last_row..last_row + 8,
        );
    }

    ppu.set_scroll(5); //x scroll
    ppu.set_scroll(0); //y scroll

    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        assert_pixels(
            &[
                /* tile 1 */ WHITE, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP,
                BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN,
            ],
            pixel_buffer,
            248..248 + 8,
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
                /* tile 1 */ GREEN, GREEN, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP,
                BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8,
        );
        //upper right corner
        assert_pixels(
            &[
                /* tile 1 */ BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, ORANGE,
                BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            248..248 + 8,
        );
        //lower left corner
        let last_row = 239 * 256;
        assert_pixels(
            &[
                BROWN, BROWN, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            last_row..last_row + 8,
        );
        //lower right corner
        assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, BACK_DROP, WHITE,
            ],
            pixel_buffer,
            {
                let start = 256 * 240 - 8;
                start..start + 8
            },
        );
    }
}
