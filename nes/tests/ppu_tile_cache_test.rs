#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod screen;
use nes::memory::SharedMemory;
use nes::ppu::ppumemory::{Mirroring, PPUMemory};
use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;

use screen::{BACK_DROP, BROWN, WHITE};

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

    load_ppu_patterns(&mut ppu);

    let mut screen = ScreenMock::new();
    ppu.update_screen(&mut screen);

    ppu.set_vram(0x20);
    ppu.set_vram(0x00);
    ppu.set_scroll(0);
    ppu.set_scroll(0);
    ppu.write_to_vram(0x01);
    ppu.write_to_vram(0x00);
    ppu.update_screen(&mut screen);
    let pixel_buffer = screen.screen_buffer.as_ref();

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, BROWN, BROWN, BROWN, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        0..8,
    );

    screen::assert_pixels(
        &[
            BACK_DROP, BACK_DROP, BACK_DROP, WHITE, WHITE, WHITE, BACK_DROP, BACK_DROP,
        ],
        pixel_buffer,
        8..16,
    );
}

fn load_ppu_patterns(ppu: &mut PPU) {
    ppu.load(
        0x0000,
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
