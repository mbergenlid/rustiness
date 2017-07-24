#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod screen;

use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;
use nes::ppu::ppumemory::{PPUMemory, Mirroring};
use nes::memory::SharedMemory;
use std::rc::Rc;
use std::cell::RefCell;

fn create_ppu() -> Rc<RefCell<PPU>> {
    let memory = memory!(
            //BG colour palettes
            0x3F00 => 0x00, //Gray,
            0x3F01 => 0x20, //White

            //Sprite palettes
            0x3F10 => 0x1F, //Black
            0x3F11 => 0x20, //White
            0x3F13 => 0x0B, //(0x00,0x3F,0x17)

            0x3F14 => 0xFF, //Invalid
            0x3F15 => 0x17, //(0xCB,0x4F,0x0F)
            0x3F17 => 0x3B  //(0xB3,0xFF,0xCF)
        );
    let mut ppu = PPU::new(PPUMemory::wrap(SharedMemory::wrap(memory), Mirroring::NoMirroring));
    ppu.set_ppu_ctrl(0x08);

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

            //Pattern table 2
            //Layer 1
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            //Layer 2
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        ]
    );
    return Rc::new(RefCell::new(ppu));
}

use nes::memory::{CPUMemory, Memory};
use nes::sound::APU;

use screen::{BROWN, WHITE, GRAY, ORANGE};

#[test]
fn test_basic_sprite_rendering() {
    let ppu = create_ppu();
    let mut sprites = [0; 64*4];
    sprites[0..4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    let mut screen = ScreenMock::new();
    let basic_memory = memory!(
        0x0200 => 0x00,
        0x0201 => 0x01,
        0x0202 => 0x00,
        0x0203 => 0x00
    );

    let mut cpu_memory = CPUMemory::default(box basic_memory, ppu.clone(), &APU::new(Rc::new(RefCell::new(Vec::new())), 1), None);
    {
        cpu_memory.set(0x4014, 0x02);
    };

    ppu.borrow_mut().update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
    /* tile 1 */ GRAY, GRAY, GRAY, GRAY, GRAY, GRAY, GRAY, GRAY,
            ],
            pixel_buffer,
            0..8
        );
        screen::assert_pixels(
            &[
    /* tile 1 */ WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            ],
            pixel_buffer,
            256..256+8
        );
    }

    {
        cpu_memory.set(0x0200, 0);
        cpu_memory.set(0x0203, 8);
        cpu_memory.set(0x4014, 0x02);
    }
    ppu.borrow_mut().update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
    /* tile 1 */ GRAY,GRAY,GRAY,GRAY,GRAY,GRAY,GRAY,GRAY,
    /* tile 2 */ WHITE,WHITE,WHITE,WHITE,WHITE,WHITE,WHITE,WHITE,
            ],
            pixel_buffer,
            256..256+16
        );
    }

    {
        cpu_memory.set(0x0200, 5);
        cpu_memory.set(0x0203, 10);
        cpu_memory.set(0x4014, 0x02);
    }
    ppu.borrow_mut().update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
                WHITE, WHITE, GRAY, GRAY, GRAY, GRAY, WHITE, WHITE
            ],
            pixel_buffer,
            {
                let start = 8*256 + 10;
                start..start+8
            }
        );
    }
}

#[test]
fn test_multiple_sprite_rendering() {
    let ppu = create_ppu();
    let mut sprites = [0; 64*4];
    sprites[0..4].copy_from_slice(&[0x00, 0x01, 0x00, 0x00]);
    let mut screen = ScreenMock::new();
    let basic_memory = memory!(
        0x0200 => 0x00,
        0x0201 => 0x01,
        0x0202 => 0x00,
        0x0203 => 0x00,

        0x0210 => 0x08,
        0x0211 => 0x01,
        0x0212 => 0x01,
        0x0213 => 0x08

    );

    {
        let mut cpu_memory = CPUMemory::default(box basic_memory, ppu.clone(), &APU::new(Rc::new(RefCell::new(Vec::new())), 1), None);
        cpu_memory.set(0x4014, 0x02);
    };

    ppu.borrow_mut().update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
    /* tile 1 */ WHITE,WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            ],
            pixel_buffer,
            256..256+8
        );
        screen::assert_pixels(
            &[
    /* tile 1 */ ORANGE, ORANGE,ORANGE,ORANGE,ORANGE,ORANGE,ORANGE,ORANGE,
            ],
            pixel_buffer,
            {
                let start = 9*256 + 8;
                start..start+8
            }
        );
    }
}

#[test]
fn test_background_sprite() {
    let ppu = create_ppu();
    let mut screen = ScreenMock::new();
    let basic_memory = memory!(
        0x0200 => 0x00, //Position Y
        0x0201 => 0x03,
        0x0202 => 0x20, //Priority back
        0x0203 => 0x00 //Position X
    );
    ppu.borrow_mut().load(
        0x0,
        &[
            //Pattern table 0
            //Layer 1
            0b11111111, 0b11111111, 0b11000011, 0b11000011, 0b11000011, 0b11000011, 0b11111111, 0b11111111,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ]
    );

    {
        let mut cpu_memory = CPUMemory::default(box basic_memory, ppu.clone(), &APU::new(Rc::new(RefCell::new(Vec::new())), 1), None);
        cpu_memory.set(0x4014, 0x02);
    };

    ppu.borrow_mut().update_screen(&mut screen);
    {
        let pixel_buffer = screen.screen_buffer.as_ref();

        screen::assert_pixels(
            &[
    /* tile 1 */ WHITE,WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            ],
            pixel_buffer,
            0..8
        );
        screen::assert_pixels(
            &[
    /* tile 1 */ WHITE,WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE,
            ],
            pixel_buffer,
            {
                let start = 1*256;
                start..start+8
            }
        );
        screen::assert_pixels(
            &[
    /* tile 1 */ WHITE,WHITE, BROWN, BROWN, BROWN, BROWN, WHITE, WHITE,
            ],
            pixel_buffer,
            {
                let start = 2*256;
                start..start+8
            }
        );
    }
}
