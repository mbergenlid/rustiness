
pub type Color = [f32; 3];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile {
    pub pattern_index: u32,
    pub palette_index: u8
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub data: [[u8; 8]; 8],
}

use std::fmt;
use std::fmt::Debug;
impl Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            writeln!(f, "{:?}", row).expect("Failed to write");
        }
        Ok(())
    }
}

pub trait Screen {
    fn update_tile(&mut self, x: usize, y: usize, tile: &Tile);
    fn update_patterns(&mut self, pattern: &[Pattern]);

    fn set_universal_background(&mut self, background_value: u8);
    fn update_palette(&mut self, palette: u8, index: u8, palette_value: u8);

    fn draw(&mut self);
}

pub struct ScreenMock {}

impl ScreenMock {
    pub fn new() -> ScreenMock {
        ScreenMock {}
    }
}

impl Screen for ScreenMock {
    fn update_tile(&mut self, _: usize, _: usize, _: &Tile) {
    }

    fn update_patterns(&mut self, _: &[Pattern]) {
    }

    fn set_universal_background(&mut self, _: u8) {
    }

    fn update_palette(&mut self, _: u8, _: u8, _: u8) {
    }

    fn draw(&mut self) {
    }
}

pub const BLACK: Color = [0.0, 0.0, 0.0];
pub const WHITE: Color = [1.0, 1.0, 1.0];

pub const COLOUR_PALETTE: [Color; 0x40] = [
    [(0x75 as f32)/ (0xFF as f32), (0x75 as f32 / 0xFF as f32), 0x75 as f32 / 0xFF as f32], //0x00
    [(0x27 as f32)/ (0xFF as f32), (0x1B as f32 / 0xFF as f32), 0x8F as f32 / 0xFF as f32], //0x01
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0xAB as f32 / 0xFF as f32], //0x02
    [(0x47 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x9F as f32 / 0xFF as f32], //0x03
    [(0x8F as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x77 as f32 / 0xFF as f32], //0x04
    [(0xAB as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x13 as f32 / 0xFF as f32], //0x05
    [(0xA7 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x06
    [(0x7F as f32)/ (0xFF as f32), (0x0B as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x07
    [(0x43 as f32)/ (0xFF as f32), (0x2F as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x08
    [(0x00 as f32)/ (0xFF as f32), (0x47 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x09
    [(0x00 as f32)/ (0xFF as f32), (0x51 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x0A
    [(0x00 as f32)/ (0xFF as f32), (0x3F as f32 / 0xFF as f32), 0x17 as f32 / 0xFF as f32], //0x0B
    [(0x1B as f32)/ (0xFF as f32), (0x3F as f32 / 0xFF as f32), 0x5F as f32 / 0xFF as f32], //0x0C
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x0D
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x0E
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x0F
    [(0xBC as f32)/ (0xFF as f32), (0xBC as f32 / 0xFF as f32), 0xBC as f32 / 0xFF as f32], //0x10
    [(0x00 as f32)/ (0xFF as f32), (0x73 as f32 / 0xFF as f32), 0xEF as f32 / 0xFF as f32], //0x11
    [(0x23 as f32)/ (0xFF as f32), (0x3B as f32 / 0xFF as f32), 0xEF as f32 / 0xFF as f32], //0x12
    [(0x83 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0xF3 as f32 / 0xFF as f32], //0x13
    [(0xBF as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0xBF as f32 / 0xFF as f32], //0x14
    [(0xE7 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x5B as f32 / 0xFF as f32], //0x15
    [(0xDB as f32)/ (0xFF as f32), (0x2B as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x16
    [(0xCB as f32)/ (0xFF as f32), (0x4F as f32 / 0xFF as f32), 0x0F as f32 / 0xFF as f32], //0x17
    [(0x8B as f32)/ (0xFF as f32), (0x73 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x18
    [(0x00 as f32)/ (0xFF as f32), (0x97 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x19
    [(0x00 as f32)/ (0xFF as f32), (0xAB as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x1A
    [(0x00 as f32)/ (0xFF as f32), (0x93 as f32 / 0xFF as f32), 0x3B as f32 / 0xFF as f32], //0x1B
    [(0x00 as f32)/ (0xFF as f32), (0x83 as f32 / 0xFF as f32), 0x8B as f32 / 0xFF as f32], //0x1C
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x1D
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x1E
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x1F
    [(0xFF as f32)/ (0xFF as f32), (0xFF as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x20
    [(0x3F as f32)/ (0xFF as f32), (0xBF as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x21
    [(0x5F as f32)/ (0xFF as f32), (0x97 as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x22
    [(0xA7 as f32)/ (0xFF as f32), (0x8B as f32 / 0xFF as f32), 0xFD as f32 / 0xFF as f32], //0x23
    [(0xF7 as f32)/ (0xFF as f32), (0x7B as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x24
    [(0xFF as f32)/ (0xFF as f32), (0x77 as f32 / 0xFF as f32), 0xB7 as f32 / 0xFF as f32], //0x25
    [(0xFF as f32)/ (0xFF as f32), (0x77 as f32 / 0xFF as f32), 0x63 as f32 / 0xFF as f32], //0x26
    [(0xFF as f32)/ (0xFF as f32), (0x9B as f32 / 0xFF as f32), 0x3B as f32 / 0xFF as f32], //0x27
    [(0xF3 as f32)/ (0xFF as f32), (0xBF as f32 / 0xFF as f32), 0x3F as f32 / 0xFF as f32], //0x28
    [(0x83 as f32)/ (0xFF as f32), (0xD3 as f32 / 0xFF as f32), 0x13 as f32 / 0xFF as f32], //0x29
    [(0x4F as f32)/ (0xFF as f32), (0xDF as f32 / 0xFF as f32), 0x4B as f32 / 0xFF as f32], //0x2A
    [(0x58 as f32)/ (0xFF as f32), (0xF8 as f32 / 0xFF as f32), 0x98 as f32 / 0xFF as f32], //0x2B
    [(0x00 as f32)/ (0xFF as f32), (0xEB as f32 / 0xFF as f32), 0xDB as f32 / 0xFF as f32], //0x2C
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x2D
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x2E
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x2F
    [(0xFF as f32)/ (0xFF as f32), (0xFF as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x30
    [(0xAB as f32)/ (0xFF as f32), (0xE7 as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x31
    [(0xC7 as f32)/ (0xFF as f32), (0xD7 as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x32
    [(0xD7 as f32)/ (0xFF as f32), (0xCB as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x33
    [(0xFF as f32)/ (0xFF as f32), (0xC7 as f32 / 0xFF as f32), 0xFF as f32 / 0xFF as f32], //0x34
    [(0xFF as f32)/ (0xFF as f32), (0xC7 as f32 / 0xFF as f32), 0xDB as f32 / 0xFF as f32], //0x35
    [(0xFF as f32)/ (0xFF as f32), (0xBF as f32 / 0xFF as f32), 0xB3 as f32 / 0xFF as f32], //0x36
    [(0xFF as f32)/ (0xFF as f32), (0xDB as f32 / 0xFF as f32), 0xAB as f32 / 0xFF as f32], //0x37
    [(0xFF as f32)/ (0xFF as f32), (0xE7 as f32 / 0xFF as f32), 0xA3 as f32 / 0xFF as f32], //0x38
    [(0xE3 as f32)/ (0xFF as f32), (0xFF as f32 / 0xFF as f32), 0xA3 as f32 / 0xFF as f32], //0x39
    [(0xAB as f32)/ (0xFF as f32), (0xF3 as f32 / 0xFF as f32), 0xBF as f32 / 0xFF as f32], //0x3A
    [(0xB3 as f32)/ (0xFF as f32), (0xFF as f32 / 0xFF as f32), 0xCF as f32 / 0xFF as f32], //0x3B
    [(0x9F as f32)/ (0xFF as f32), (0xFF as f32 / 0xFF as f32), 0xF3 as f32 / 0xFF as f32], //0x3C
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x3D
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x3E
    [(0x00 as f32)/ (0xFF as f32), (0x00 as f32 / 0xFF as f32), 0x00 as f32 / 0xFF as f32], //0x3F

];