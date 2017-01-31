#![feature(box_syntax)]
#[macro_use]
extern crate nes;

use nes::ppu::screen::{Pattern, Tile, Screen};
use nes::memory::BasicMemory;
use nes::ppu::PPU;

#[test]
fn write_to_name_table() {
    let expected_tiles: Vec<(usize, usize, Tile)> = vec!(
        (0, 0, Tile { pattern_index: 0, palette_index: 0}),
        (1, 0, Tile { pattern_index: 0x10, palette_index: 0}),
        (2, 0, Tile { pattern_index: 0x20, palette_index: 1}),
        (3, 0, Tile { pattern_index: 0, palette_index: 1}),
        (2, 1, Tile { pattern_index: 0, palette_index: 1}),
        (3, 1, Tile { pattern_index: 0, palette_index: 1}),

        (0, 30, Tile { pattern_index: 0, palette_index: 0}),
        (1, 30, Tile { pattern_index: 0x10, palette_index: 0}),
        (2, 30, Tile { pattern_index: 0x20, palette_index: 1}),
        (3, 30, Tile { pattern_index: 0, palette_index: 1}),
        (2, 31, Tile { pattern_index: 0, palette_index: 1}),
        (3, 31, Tile { pattern_index: 0, palette_index: 1}),
    );
    let screen = PPUTestScreen::new();
    let mut ppu = PPU::new(box BasicMemory::new(), box screen.clone());
    ppu.draw();
    ppu.set_ppu_mask(0x08);

    ppu.set_vram(0x20);
    ppu.set_vram(0x00);

    ppu.write_to_vram(0x00);
    ppu.write_to_vram(0x10);
    ppu.write_to_vram(0x20);
    //        //0x23C0 => 0b00_00_01_00
    ppu.set_vram(0x23);
    ppu.set_vram(0xC0);
    ppu.write_to_vram(0b00_00_01_00);


    ppu.set_vram(0x28);
    ppu.set_vram(0x00);
    ppu.write_to_vram(0x00);
    ppu.write_to_vram(0x10);
    ppu.write_to_vram(0x20);

    ppu.set_vram(0x2B);
    ppu.set_vram(0xC0);
    ppu.write_to_vram(0b00_00_01_00);

    ppu.draw();

    let ref tiles = screen.data().tiles;

    for &tile in tiles.iter() {
        let expected_tile = expected_tiles.iter()
            .find(|t| t.0 == tile.0 && t.1 == tile.1)
            .map(|t| t.2)
            .unwrap_or(Tile { pattern_index: 0, palette_index: 0})
        ;
        assert_eq!(expected_tile, tile.2, "Tile x: {}, y: {} is not what was expected", tile.0, tile.1);
    }
}

#[test]
fn write_palettes() {
    let expected_background = 0x0E;
    let expected_palettes: Vec<Vec<u8>> = vec!(
        vec!(0x0A, 0x0B, 0x0C),
        vec!(0x1A, 0x1B, 0x1C),
        vec!(0x2A, 0x2B, 0x2C),
        vec!(0x3A, 0x3B, 0x3C),
    );
    let screen = PPUTestScreen::new();
    let mut ppu = PPU::new(box BasicMemory::new(), box screen.clone());
    ppu.set_vram(0x3F);
    ppu.set_vram(0x00);

    ppu.write_to_vram(0x0E);
    ppu.write_to_vram(0x0A);
    ppu.write_to_vram(0x0B);
    ppu.write_to_vram(0x0C);

    ppu.write_to_vram(0xFF);
    ppu.write_to_vram(0x1A);
    ppu.write_to_vram(0x1B);
    ppu.write_to_vram(0x1C);

    ppu.write_to_vram(0xFF);
    ppu.write_to_vram(0x2A);
    ppu.write_to_vram(0x2B);
    ppu.write_to_vram(0x2C);

    ppu.write_to_vram(0xFF);
    ppu.write_to_vram(0x3A);
    ppu.write_to_vram(0x3B);
    ppu.write_to_vram(0x3C);

    ppu.set_ppu_mask(0x08);
    ppu.draw();

    assert_eq!(expected_background, screen.background().unwrap());

    let ref palettes = screen.data().palettes;
    assert_eq!(expected_palettes, *palettes);
}

#[test]
fn test_patterns() {
    let expected_patterns: [Pattern; 3] = [
        Pattern { data: [
            [0,0,0,3,3,3,0,0],
            [0,0,3,3,0,0,3,0],
            [0,0,3,3,3,0,0,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,3,3,3,0],
            [0,0,3,0,0,3,3,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,0,0,0,0],
        ]},
        Pattern { data: [
            [0,0,0,1,1,1,0,0],
            [0,0,2,2,0,0,2,0],
            [0,0,3,3,3,0,0,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,3,3,3,0],
            [0,0,3,0,0,3,3,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,0,0,0,0],
        ]},
        Pattern { data: [
            [0,0,0,3,3,3,0,0],
            [0,0,1,1,0,0,1,0],
            [0,0,2,2,2,0,0,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,3,3,3,0],
            [0,0,3,0,0,3,3,0],
            [0,0,0,3,3,3,0,0],
            [0,0,0,0,0,0,0,0],
        ]},
    ];

    let screen = PPUTestScreen::new();
    let mut ppu = PPU::new(box BasicMemory::new(), box screen.clone());
    ppu.draw();
    ppu.set_ppu_mask(0x08);

    ppu.load(
        0,
        &[
            //Pattern table 1
            //Layer 1
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00011100, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,

            //Pattern table 2
            //Layer 1
            0b00011100, 0b00000000, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00000000, 0b00110010, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,

            //Pattern table 3
            //Layer 1
            0b00011100, 0b00110010, 0b00000000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000,
            //Layer 2
            0b00011100, 0b00000000, 0b00111000, 0b00011100, 0b00001110, 0b00100110, 0b00011100, 0b00000000
            //Pattern table end
        ]
    );
    ppu.draw();

    let ref patterns = screen.data().patterns;
    assert_eq!(256, patterns.len());

    for i in 0..expected_patterns.len() {
        assert_eq!(expected_patterns[i], patterns[i], "Pattern {} differs", i);
    }
}

struct PPUTestScreenData {
    tiles: Vec<(usize, usize, Tile)>,
    patterns: Vec<Pattern>,
    background: Option<u8>,
    palettes: Vec<Vec<u8>>
}

use std::rc::Rc;
use std::cell::{RefCell, Ref};

#[derive(Clone)]
struct PPUTestScreen {
    data: Rc<RefCell<PPUTestScreenData>>,
}

impl PPUTestScreen {
    pub fn new() -> PPUTestScreen {
        PPUTestScreen {
            data: Rc::new(RefCell::new(PPUTestScreenData {
                tiles: vec!(),
                patterns: vec!(),
                background: None,
                palettes: vec!(
                    vec!(0, 0, 0),
                    vec!(0, 0, 0),
                    vec!(0, 0, 0),
                    vec!(0, 0, 0)
                ),
            })),
        }
    }

    pub fn background(&self) -> Option<u8> {
        self.data.borrow().background
    }

    pub fn data(&self) -> Ref<PPUTestScreenData> {
        self.data.borrow()
    }
}

impl Screen for PPUTestScreen {
    fn update_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        self.data.borrow_mut().tiles.push((x, y, *tile))
    }

    fn update_patterns(&mut self, pattern: &[Pattern]) {
        let ref mut patterns = self.data.borrow_mut().patterns;
        for &p in pattern {
            patterns.push(p);
        }
    }

    fn set_universal_background(&mut self, background_value: u8) {
        self.data.borrow_mut().background = Some(background_value);
    }

    fn update_palette(&mut self, palette: u8, index: u8, palette_value: u8) {
        self.data.borrow_mut().palettes[palette as usize][index as usize] = palette_value;
    }

    fn draw(&mut self) {
        //            unimplemented!()
    }
    fn set_background_offset(&mut self, _: usize, _: usize) {
        unimplemented!()
    }
}