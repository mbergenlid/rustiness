#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod screen;
use nes::memory::SharedMemory;
use nes::ppu::ppumemory::{Mirroring, PPUMemory};
use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;

use screen::{BACK_DROP, BROWN, GREEN, ORANGE, WHITE};

#[test]
fn draw_buffer_from_name_table_1() {
    let memory = memory!(
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
    let mut ppu = PPU::new(PPUMemory::wrap(
        SharedMemory::wrap(memory),
        Mirroring::NoMirroring,
    ));
    ppu.set_ppu_ctrl(0x08);

    load_ppu_patterns(&mut ppu, 0x0000);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        0..8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, WHITE, WHITE, BACK_DROP, BACK_DROP, WHITE, BACK_DROP,
        ],
        pixel_buffer,
        256..256 + 8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        8..16,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        16..16 + 8,
    );
}

#[test]
fn draw_buffer_from_name_table_1_with_high_background_patterns() {
    let memory = memory!(
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
    let mut ppu = PPU::new(PPUMemory::wrap(
        SharedMemory::wrap(memory),
        Mirroring::NoMirroring,
    ));
    ppu.set_ppu_ctrl(0x10);

    load_ppu_patterns(&mut ppu, 0x1000);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        0..8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, WHITE, WHITE, BACK_DROP, BACK_DROP, WHITE, BACK_DROP,
        ],
        pixel_buffer,
        256..256 + 8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        8..16,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        16..16 + 8,
    );
}

#[test]
fn draw_buffer_from_name_table_2() {
    let memory = memory!(
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
    let mut ppu = PPU::new(PPUMemory::wrap(
        SharedMemory::wrap(memory),
        Mirroring::NoMirroring,
    ));
    load_ppu_patterns(&mut ppu, 0x0000);

    ppu.set_ppu_ctrl(0x08 | 0x01);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        0..8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, WHITE, WHITE, BACK_DROP, BACK_DROP, WHITE, BACK_DROP,
        ],
        pixel_buffer,
        256..256 + 8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        8..16,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        16..16 + 8,
    );
}

#[test]
fn draw_buffer_from_name_table_1_with_scrolling_y() {
    let memory = memory!(
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
    let mut ppu = PPU::new(PPUMemory::wrap(
        SharedMemory::wrap(memory),
        Mirroring::NoMirroring,
    ));
    ppu.set_ppu_ctrl(0x08);
    load_ppu_patterns(&mut ppu, 0x0000);
    ppu.set_scroll(0); //x scroll
    ppu.set_scroll(8); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
                /* tile 1 */ BACK_DROP, BACK_DROP, BACK_DROP, GREEN, GREEN, GREEN, BACK_DROP,
                BACK_DROP, /* tile 2 */ BACK_DROP, BACK_DROP, BACK_DROP, ORANGE, ORANGE,
                ORANGE, BACK_DROP, BACK_DROP, /* tile 3 */ BACK_DROP, BACK_DROP, BACK_DROP,
                WHITE, WHITE, WHITE, BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8 * 3,
        );
        screen::assert_pixels(
            &[
                BACK_DROP, BACK_DROP, GREEN, GREEN, BACK_DROP, BACK_DROP, GREEN, BACK_DROP,
            ],
            pixel_buffer,
            256..256 + 8,
        );
    }
    ppu.set_scroll(0);
    ppu.set_scroll(9);
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
                /* tile 1 */ BACK_DROP, BACK_DROP, GREEN, GREEN, BACK_DROP, BACK_DROP, GREEN,
                BACK_DROP, /* tile 2 */ BACK_DROP, BACK_DROP, ORANGE, ORANGE, BACK_DROP,
                BACK_DROP, ORANGE, BACK_DROP, /* tile 3 */ BACK_DROP, BACK_DROP, WHITE, WHITE,
                BACK_DROP, BACK_DROP, WHITE, BACK_DROP,
            ],
            pixel_buffer,
            0..8 * 3,
        );
    }
}

#[test]
fn draw_buffer_from_name_table_1_with_scrolling_x() {
    let memory = memory!(
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
    let mut ppu = PPU::new(PPUMemory::wrap(
        SharedMemory::wrap(memory),
        Mirroring::NoMirroring,
    ));
    ppu.set_ppu_ctrl(0x08);
    load_ppu_patterns(&mut ppu, 0x0000);
    ppu.set_scroll(8); //x scroll
    ppu.set_scroll(0); //y scroll

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
                BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
                BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
            ],
            pixel_buffer,
            0..8 * 2,
        );
    }
}

fn load_ppu_patterns(ppu: &mut PPU, address: u16) {
    ppu.load(
        address,
        &[
            //Pattern table 0
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000000, //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000000, //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100,
            0b00000000,
        ],
    );
}
