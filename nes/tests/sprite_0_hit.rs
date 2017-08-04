#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::ScreenMock;
use nes::ppu::PPU;
use nes::ppu::ppumemory::{PPUMemory, Mirroring};
use nes::memory::SharedMemory;
use std::rc::Rc;
use std::cell::RefCell;
use nes::memory::{CPUMemory, Memory};
use nes::sound::APU;

#[test]
fn test_basic_sprite_rendering() {
    let ppu = create_ppu();
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

    //Sprite 0 should hit on pixel 4
    //i.e first scanline dot 2+4=6 => 6 PPU_CYCLES = 6/3 = 2 CPU_CYCLES
    ppu.borrow_mut().update(1, &mut screen);
    assert_eq!(0x0, ppu.borrow_mut().status() & 0x40);
    ppu.borrow_mut().update(2, &mut screen);
    assert_eq!(0x40, ppu.borrow_mut().status() & 0x40);
}

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
            0x3F17 => 0x3B, //(0xB3,0xFF,0xCF)

            0x2000 => 0x01
        );
    let mut ppu = PPU::new(PPUMemory::wrap(SharedMemory::wrap(memory), Mirroring::NoMirroring));
    ppu.set_ppu_ctrl(0x08);

    ppu.load(
        0x10,
        &[
            //Pattern table 0
            //Layer 1
            0b00001100,
            0b00110010,
            0b00111000,
            0b00011100,
            0b00001110,
            0b00100110,
            0b00011100,
            0b00000000,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ]
    );
    ppu.load(
        0x1010,
        &[
            //Pattern table 0
            //Layer 1
            0b11111111,
            0b11111111,
            0b11000011,
            0b11000011,
            0b11000011,
            0b11000011,
            0b11111111,
            0b11111111,
            //Layer 2
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        ]
    );
    return Rc::new(RefCell::new(ppu));
}
