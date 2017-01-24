pub mod screen;

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
            memory: memory,
            screen: screen,
            vram_pointer: 0,
            vram_high_byte: true, //Big Endian

            pattern_tables_changed: false,
            name_tables_changed: false,
        }
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.control_register.value = value;
    }

    pub fn ppu_ctrl(&self) -> u8 {
        self.control_register.value
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

                        pattern.data[pattern_row][bit_index] = pixel;
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
                    self.screen.update_tile(col as usize, row as usize, &Tile { pattern_index: pattern_table_address, palette_index: colour_palette_index});

                    name_table += 1;
                }
            }
            self.name_tables_changed = false;
        }
        self.screen.draw();
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

    struct PPUTestScreen<'a> {
        initiated: bool,

        expected_tiles: &'a [(usize, usize, Tile)],
        expected_patterns: &'a [Pattern],
        expected_background: Option<u8>,
        expected_palettes: Option<[[u8;3]; 4]>
    }

    impl <'a> PPUTestScreen<'a> {
        pub fn with_expected_tiles<'b>(tiles: &'b [(usize, usize, Tile)]) -> PPUTestScreen<'b> {
            PPUTestScreen {
                initiated: false,
                expected_tiles: tiles,
                expected_patterns: &[],
                expected_background: None,
                expected_palettes: None,
            }
        }

        pub fn with_expected_patterns<'b>(patterns: &'b [Pattern]) -> PPUTestScreen<'b> {
            PPUTestScreen {
                initiated: false,
                expected_tiles: &[],
                expected_patterns: patterns,
                expected_background: None,
                expected_palettes: None,
            }
        }

        pub fn with_expected_palettes<'b>(background: u8, palettes: [[u8; 3]; 4]) -> PPUTestScreen<'b> {
            PPUTestScreen {
                initiated: false,
                expected_tiles: &[],
                expected_patterns: &[],
                expected_background: Some(background),
                expected_palettes: Some(palettes),
            }
        }
    }

    impl <'a> Screen for PPUTestScreen<'a> {
        fn update_tile(&mut self, x: usize, y: usize, tile: &Tile) {
            if self.initiated {
                if self.expected_tiles.len() == 0 {
                    panic!("Unexpected call to method screen.update_tile");
                } else {
                    let tile_optional = self.expected_tiles.iter().find(|t| t.0 == x && t.1 == y);
                    match tile_optional {
                        Some(t) => assert_eq!(t.2, *tile),
                        None => assert_eq!(Tile { pattern_index: 0, palette_index: 0}, *tile),
                    }
                }
            }
        }

        fn update_patterns(&mut self, patterns: &[Pattern]) {
            if self.initiated {
                if self.expected_patterns.len() == 0 {
                    panic!("Unexpected call to method screen.update_patterns");
                } else {
                    assert_eq!(256, patterns.len());

                    for i in 0..self.expected_patterns.len() {
                        assert_eq!(self.expected_patterns[i], patterns[i], "Pattern {} is differs", i);
                    }
                }
            }
        }

        fn set_universal_background(&mut self, background: u8) {
            match self.expected_background {
                Some(bg) => assert_eq!(bg, background),
                None => ()
            }
        }

        fn update_palette(&mut self, palette: u8, index: u8, palette_value: u8) {
            match self.expected_palettes {
                Some(palettes) => {
                    let expected_value = palettes[palette as usize][index as usize];
                    assert_eq!(expected_value, palette_value, "Failed on palette {}, {}", palette, index);
                },
                None => ()
            }
        }

        fn draw(&mut self) {
            self.initiated = true;
        }
    }



    #[test]
    fn write_to_name_table() {
        static EXPECTED_TILES: [(usize, usize, Tile); 6] = [
            (0, 0, Tile { pattern_index: 0, palette_index: 0}),
            (1, 0, Tile { pattern_index: 0x10, palette_index: 0}),
            (2, 0, Tile { pattern_index: 0x20, palette_index: 1}),
            (3, 0, Tile { pattern_index: 0, palette_index: 1}),
            (2, 1, Tile { pattern_index: 0, palette_index: 1}),
            (3, 1, Tile { pattern_index: 0, palette_index: 1}),

        ];
        let screen = PPUTestScreen::with_expected_tiles(&EXPECTED_TILES);
        let mut ppu = PPU::new(box BasicMemory::new(), box screen);
        ppu.draw();

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
    }

    #[test]
    fn write_palettes() {
        let screen = PPUTestScreen::with_expected_palettes(0x0E, [[0x0A, 0x0B, 0x0C], [0x1A, 0x1B, 0x1C], [0x2A, 0x2B, 0x2C], [0x3A, 0x3B, 0x3C]]);
        let mut ppu = PPU::new(box BasicMemory::new(), box screen);
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

        ppu.draw();

    }

    #[test]
    fn test() {
        static EXPECTED_PATTERNS: [Pattern; 3] = [
            Pattern { data: [
                [0,0,3,3,3,0,0,0],
                [0,3,0,0,3,3,0,0],
                [0,0,0,3,3,3,0,0],
                [0,0,3,3,3,0,0,0],
                [0,3,3,3,0,0,0,0],
                [0,3,3,0,0,3,0,0],
                [0,0,3,3,3,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]},
            Pattern { data: [
                [0,0,1,1,1,0,0,0],
                [0,2,0,0,2,2,0,0],
                [0,0,0,3,3,3,0,0],
                [0,0,3,3,3,0,0,0],
                [0,3,3,3,0,0,0,0],
                [0,3,3,0,0,3,0,0],
                [0,0,3,3,3,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]},
            Pattern { data: [
                [0,0,3,3,3,0,0,0],
                [0,1,0,0,1,1,0,0],
                [0,0,0,2,2,2,0,0],
                [0,0,3,3,3,0,0,0],
                [0,3,3,3,0,0,0,0],
                [0,3,3,0,0,3,0,0],
                [0,0,3,3,3,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]},
        ];
        let screen = PPUTestScreen::with_expected_patterns(&EXPECTED_PATTERNS);
        let mut ppu = PPU::new(box BasicMemory::new(), Box::new(screen));
        ppu.draw();

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
    }
}
