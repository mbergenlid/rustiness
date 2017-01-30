pub mod screen;
pub mod ppumemory;

use memory::Memory;
use ppu::screen::{Screen, Pattern, Tile};

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }
    fn base_name_table(&self) -> u16 {
        0x2000 + ((self.value & 0x03) as u16)*1024
    }

    fn background_pattern_table(&self) -> u16 {
        (self.value & 0x10) as u16
    }

}

struct PPUMask {
    value: u8,
}

impl PPUMask {
    fn is_drawing_enabled(&self) -> bool {
        self.value & 0x08 > 0
    }
}

const COLOUR_PALETTE_BASE_ADDRESS: u16 = 0x3F00;
pub struct AttributeTable<'a> {
    memory: &'a Memory,
    address: u16
}

impl<'a> AttributeTable<'a> {
    pub fn get_palette_address(&self, tile_row: u16, tile_col: u16) -> u16 {
        let palette_index = self.get_palette_index(tile_row, tile_col);

        COLOUR_PALETTE_BASE_ADDRESS + (palette_index as u16)*4
    }

    pub fn get_palette_index(&self, tile_row: u16, tile_col: u16) -> u8 {
        let attribute_row = tile_row >> 2;
        let attribute_col = tile_col >> 2;
        let row_inside_attribute = (tile_row & 0x03) >> 1;
        let col_inside_attribute = (tile_col & 0x03) >> 1;
        let quadrant = (row_inside_attribute << 1) | col_inside_attribute;
        let attribute_address = self.address + (attribute_row*8 + attribute_col);

        let value = self.memory.get(attribute_address);
        value >> (quadrant << 1) & 0x03
    }
}

pub struct PPU {
    control_register: PPUCtrl,
    mask_register: PPUMask,
    memory: Box<Memory>,
    screen: Box<Screen>,
    vram_pointer: u16,
    vram_high_byte: bool,

    pattern_tables_changed: bool,
    name_tables_changed: bool,
}

impl PPU {
    pub fn new(memory: Box<Memory>, screen: Box<Screen>) -> PPU {
        PPU {
            control_register: PPUCtrl::new(),
            mask_register: PPUMask { value: 0 },
            memory: memory,
            screen: screen,
            vram_pointer: 0,
            vram_high_byte: true, //Big Endian

            pattern_tables_changed: true,
            name_tables_changed: true,
        }
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.control_register.value = value;
    }

    pub fn ppu_ctrl(&self) -> u8 {
        self.control_register.value
    }

    pub fn set_ppu_mask(&mut self, value: u8) {
        self.mask_register.value = value;
    }

    pub fn set_vram(&mut self, value: u8) {
        if self.vram_high_byte {
            self.vram_pointer = 0;
            self.vram_pointer = (value as u16) << 8;
            self.vram_high_byte = false;
        } else {
            self.vram_pointer = self.vram_pointer | (value as u16);
            self.vram_high_byte = true;
        }
    }

    pub fn vram(&self) -> u16 {
        self.vram_pointer
    }

    pub fn write_to_vram(&mut self, value: u8) {
        if self.vram_pointer >= 0x2000 && self.vram_pointer < 0x3000 {
            self.name_tables_changed = true;
        } else if self.vram_pointer < 0x200 {
            self.pattern_tables_changed = true;
        } else if self.vram_pointer >= 0x3F00 && self.vram_pointer < 0x3F20 {
            self.pattern_tables_changed = true;
        }
        self.memory.set(self.vram_pointer, value);
        self.vram_pointer += 1;
    }

    pub fn load(&mut self, base_address: u16, rom: &[u8]) {
        let current_vram = self.vram_pointer;
        self.vram_pointer = base_address;

        for &byte in rom {
            self.write_to_vram(byte);
        }

        self.vram_pointer = current_vram;
    }

    pub fn draw(&mut self) {
        if self.mask_register.is_drawing_enabled() {
            let pattern_table_base_address = self.control_register.background_pattern_table();

            //Patterns
            if self.pattern_tables_changed {
                self.screen.set_universal_background(self.memory.get(0x3F00));
                self.screen.update_palette(0, 0, self.memory.get(0x3F01));
                self.screen.update_palette(0, 1, self.memory.get(0x3F02));
                self.screen.update_palette(0, 2, self.memory.get(0x3F03));

                self.screen.update_palette(1, 0, self.memory.get(0x3F05));
                self.screen.update_palette(1, 1, self.memory.get(0x3F06));
                self.screen.update_palette(1, 2, self.memory.get(0x3F07));

                self.screen.update_palette(2, 0, self.memory.get(0x3F09));
                self.screen.update_palette(2, 1, self.memory.get(0x3F0A));
                self.screen.update_palette(2, 2, self.memory.get(0x3F0B));

                self.screen.update_palette(3, 0, self.memory.get(0x3F0D));
                self.screen.update_palette(3, 1, self.memory.get(0x3F0E));
                self.screen.update_palette(3, 2, self.memory.get(0x3F0F));

                let mut patterns = Vec::with_capacity(256);
                for p_table in 0x00..0x100 {

                    let mut pattern_table_address = pattern_table_base_address | (p_table << 4);
                    let mut pattern: Pattern = Pattern { data: [[0; 8]; 8] };
                    for pattern_row in 0..8 {
                        let mut low_bits = self.memory.get(pattern_table_address);
                        let mut high_bits = self.memory.get(pattern_table_address+8);
                        for bit_index in 0..8 {
                            let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);

                            pattern.data[pattern_row][7-bit_index] = pixel;
                            low_bits >>= 1;
                            high_bits >>= 1;
                        }
                        pattern_table_address += 1;
                    }
                    patterns.push(pattern);
                }
                self.screen.update_patterns(&patterns);
                self.pattern_tables_changed = false;
            }

            //Tiles
            if self.name_tables_changed {
                let mut name_table = self.control_register.base_name_table();
                for row in 0..30 {
                    for col in 0..32 {
                        let pattern_table_address = self.memory.get(name_table) as u32;
                        let colour_palette_index = {
                            let attribute_table = AttributeTable {
                                memory: &(*self.memory),
                                address: 0x23C0,
                            };
                            attribute_table.get_palette_index(row, col)
                        };
                        self.screen.update_tile(
                            col as usize,
                            row as usize,
                            &Tile {
                                pattern_index: pattern_table_address,
                                palette_index: colour_palette_index
                            }
                        );

                        name_table += 1;
                    }
                }
                self.name_tables_changed = false;
            }
            self.screen.draw();
        }
    }

    pub fn memory(&self) -> &Memory {
        self.memory.as_ref()
    }
}

#[cfg(test)]
pub mod tests {
    use memory::{Memory, BasicMemory};
    use super::screen::{Pattern, Screen, Tile};
    use super::PPU;

    use super::AttributeTable;

    #[test]
    fn test_attribute_table() {
        let memory = memory!(
            0x23C0 => 0b11_10_01_00,
            0x23C1 => 0b00_01_10_11,
            0x23C9 => 0b11_10_01_00
        );
        let attribute_table = AttributeTable {
            memory: &memory,
            address: 0x23C0
        };

        //Quadrants of 0x23C0
        assert_eq!(attribute_table.get_palette_address(0, 0), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(0, 1), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(1, 0), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(1, 1), 0x3F00);

        assert_eq!(attribute_table.get_palette_address(0, 2), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(0, 3), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(1, 2), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(1, 3), 0x3F04);

        assert_eq!(attribute_table.get_palette_address(2, 0), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(2, 1), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(3, 0), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(3, 1), 0x3F08);

        assert_eq!(attribute_table.get_palette_address(2, 2), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(2, 3), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(3, 2), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(3, 3), 0x3F0C);


        //Quadrants of 0x23C1
        assert_eq!(attribute_table.get_palette_address(0, 4), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(0, 5), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(1, 4), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(1, 5), 0x3F0C);

        assert_eq!(attribute_table.get_palette_address(0, 6), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(0, 7), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(1, 6), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(1, 7), 0x3F08);

        assert_eq!(attribute_table.get_palette_address(2, 4), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(2, 5), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(3, 4), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(3, 5), 0x3F04);

        assert_eq!(attribute_table.get_palette_address(2, 6), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(2, 7), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(3, 6), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(3, 7), 0x3F00);


        //Quadrants of 0x23C9
        assert_eq!(attribute_table.get_palette_address(4, 4), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(4, 5), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(5, 4), 0x3F00);
        assert_eq!(attribute_table.get_palette_address(5, 5), 0x3F00);

        assert_eq!(attribute_table.get_palette_address(4, 6), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(4, 7), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(5, 6), 0x3F04);
        assert_eq!(attribute_table.get_palette_address(5, 7), 0x3F04);

        assert_eq!(attribute_table.get_palette_address(6, 4), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(6, 5), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(7, 4), 0x3F08);
        assert_eq!(attribute_table.get_palette_address(7, 5), 0x3F08);

        assert_eq!(attribute_table.get_palette_address(6, 6), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(6, 7), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(7, 6), 0x3F0C);
        assert_eq!(attribute_table.get_palette_address(7, 7), 0x3F0C);
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
    }

    #[test]
    fn write_to_name_table() {
        let expected_tiles: Vec<(usize, usize, Tile)> = vec!(
            (0, 0, Tile { pattern_index: 0, palette_index: 0}),
            (1, 0, Tile { pattern_index: 0x10, palette_index: 0}),
            (2, 0, Tile { pattern_index: 0x20, palette_index: 1}),
            (3, 0, Tile { pattern_index: 0, palette_index: 1}),
            (2, 1, Tile { pattern_index: 0, palette_index: 1}),
            (3, 1, Tile { pattern_index: 0, palette_index: 1}),
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
}
