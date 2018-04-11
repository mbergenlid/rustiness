use memory::{Memory, Address};
use ppu::attributetable::AttributeTable;
use ppu::screen::PixelBuffer;
use std::cell::Cell;

#[derive(Clone)]
struct Tile {
    pattern_and_colour: (u8, u8),
    modified: Cell<bool>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            pattern_and_colour: (0,0),
            modified: Cell::new(true),
        }
    }

    fn colour(&mut self, value: u8) {
        self.pattern_and_colour.1 = value;
        self.modified.set(true);
    }

    fn pattern(&mut self, value: u8) {
        self.pattern_and_colour.0 = value;
        self.modified.set(true);
    }
}

pub struct NameTable {
    tiles: Vec<Vec<Tile>>,
    raw_data: Vec<u8>
}
use ppu::ppumemory::Palette;
use ppu::pattern::Pattern;

impl NameTable {
    pub fn from_memory(memory: &Memory) -> NameTable {
        let mut tiles = vec!(vec!(Tile::new(); 64); 60);
        for name_table_index in 0..4 {
            let name_table_base = (0x2000 + name_table_index*0x400) as u16;
            let mut name_table = name_table_base;
            let x_offset_multiplier = name_table_index & 0x01;
            let y_offset_multiplier = (name_table_index & 0x2) >> 1;
            for row in 0..30 {
                for col in 0..32 {
                    let absolute_row = y_offset_multiplier*30 + row;
                    let absolute_col = x_offset_multiplier*32 + col;
                    let pattern_table_address = memory.get(name_table, 0);
                    let colour_palette = {
                        let attribute_table = AttributeTable {
                            memory: memory,
                            address: name_table_base + 0x3C0,
                        };
                        attribute_table.get_palette_index(row as u16, col as u16)
                    };
                    tiles[absolute_row][absolute_col].pattern_and_colour = (pattern_table_address, colour_palette);
                    name_table += 1;
                }
            }
        }

        let mut raw_data = vec!(0; 0x1000);
        memory.dma(0x2000..0x3000, &mut raw_data);
        NameTable {
            tiles: tiles,
            raw_data: raw_data,
        }
    }

    pub fn tile(&self, row: u16, col: u16) -> (u8, u8) {
        self.tiles[row as usize][col as usize].pattern_and_colour
    }

    pub fn invalidate_tile_cache(&mut self) {
        for tiles in self.tiles.iter_mut() {
            for tile in tiles {
                tile.modified.set(true);
            }
        }
    }

    pub fn update_tile_for_nametable(
        &self,
        pixel_buffer: &mut PixelBuffer,
        name_table_index: usize,
        patterns: &[Pattern],
        palettes: &[Palette],
    ) {
        let x_offset_multiplier = name_table_index & 0x01;
        let y_offset_multiplier = (name_table_index & 0x2) >> 1;
        for row in 0..30 {
            for col in 0..32 {
                let absolute_row = y_offset_multiplier*30 + row;
                let absolute_col = x_offset_multiplier*32 + col;
                let tile = &self.tiles[absolute_row][absolute_col];
                if tile.modified.get() {
                    let (pattern, colour_palette) = tile.pattern_and_colour;
                    let pattern = patterns[pattern as usize];
                    pattern.update_buffer(
                        pixel_buffer,
                        &palettes[colour_palette as usize],
                        absolute_col*8,
                        absolute_row*8
                    );
                    tile.modified.set(false);
                }
            }
        }
    }
}

impl Memory for NameTable {
    fn get(&self, address: Address, _: u8) -> u8 {
        self.raw_data[(address as usize) - 0x2000]
    }

    fn set(&mut self, address: Address, value: u8, _: u8) {
        let row_offset: usize = if address & 0x800 == 0 { 0 } else { 30 };
        let col_offset: usize = if address & 0x400 == 0 { 0 } else { 32 };
        if address & 0x3C0 == 0x3C0 {
            //Attribute table
            let row = row_offset + ((address as usize >> 3) & 0x7)*4;
            let col = col_offset + (address as usize & 0x7)*4;
            let mut value = value;
            let rows = if row == 29 || row == 58 { 1 } else { 2 };
            for i in 0..rows {
                for j in 0..2 {
                    self.tiles[row+i*2][col+j*2].colour(value & 0b11);
                    self.tiles[row+i*2][col+j*2 + 1].colour(value & 0b11);
                    self.tiles[row+i*2 + 1][col+j*2].colour(value & 0b11);
                    self.tiles[row+i*2 + 1][col+j*2 + 1].colour(value & 0b11);
                    value >>= 2;
                }
            }
        } else {
            let row = ((address as usize >> 5) & 0x1F) + row_offset;
            let col = (address as usize & 0x1F) + col_offset;
            self.tiles[row][col].pattern(value);
        }
        self.raw_data[(address as usize) - 0x2000] = value;
    }
}


#[cfg(test)]
mod test {
    use super::NameTable;
    use memory::{Memory, BasicMemory};

    #[test]
    fn name_table_1() {
        name_table(0, 0);
    }

    #[test]
    fn name_table_2() {
        name_table(0, 32);
    }
    #[test]
    fn name_table_3() {
        name_table(30, 0);
    }
    #[test]
    fn name_table_4() {
        name_table(30, 32);
    }

    #[test]
    fn name_table_1_initialized_from_memory() {
        name_table_from_memory(0, 0);
    }

    #[test]
    fn name_table_2_initialized_from_memory() {
        name_table_from_memory(0, 32);
    }

    #[test]
    fn name_table_3_initialized_from_memory() {
        name_table_from_memory(30, 0);
    }

    #[test]
    fn name_table_4_initialized_from_memory() {
        name_table_from_memory(30, 32);
    }

    extern crate rand;
    #[test]
    fn get_set_memory() {
        let mut name_table = NameTable::from_memory(&BasicMemory::new());
        for a in 0x2000..0x3000 {
            let value = rand::random::<u8>();
            name_table.set(a, value, 0);
            assert_eq!(
                value,
                name_table.get(a, 0),
                "Failed for address {:x}, Expected: {}, Was: {}",
                a,
                value,
                name_table.get(a, 0)
            );
        }
    }

    fn name_table(row_offset: u16, col_offset: u16) {
        let mut name_table = NameTable::from_memory(&BasicMemory::new());
        populate(&mut name_table);

        assert_name_table(&name_table, row_offset, col_offset);

    }

    fn name_table_from_memory(row_offset: u16, col_offset: u16) {
        let mut memory = BasicMemory::new();
        populate(&mut memory);

        let name_table = NameTable::from_memory(&memory);
        assert_name_table(&name_table, row_offset, col_offset);
    }

    fn populate(memory: &mut Memory) {
        memory.set(0x23C0, 0b11_10_01_00, 0);
        memory.set(0x23C1, 0b00_01_10_11, 0);
        memory.set(0x23C9, 0b11_10_01_00, 0);

        memory.set(0x27C0, 0b11_10_01_00, 0);
        memory.set(0x27C1, 0b00_01_10_11, 0);
        memory.set(0x27C9, 0b11_10_01_00, 0);

        memory.set(0x2BC0, 0b11_10_01_00, 0);
        memory.set(0x2BC1, 0b00_01_10_11, 0);
        memory.set(0x2BC9, 0b11_10_01_00, 0);

        memory.set(0x2FC0, 0b11_10_01_00, 0);
        memory.set(0x2FC1, 0b00_01_10_11, 0);
        memory.set(0x2FC9, 0b11_10_01_00, 0);

        for a in 0x2000..0x23C0 {
            memory.set(a, (a & 0xFF) as u8, 0);
        }
        for a in 0x2400..0x27C0 {
            memory.set(a, (a & 0xFF) as u8, 0);
        }
        for a in 0x2800..0x2BC0 {
            memory.set(a, (a & 0xFF) as u8, 0);
        }
        for a in 0x2C00..0x2FC0 {
            memory.set(a, (a & 0xFF) as u8, 0);
        }
    }

    fn assert_name_table(name_table: &NameTable, row_offset: u16, col_offset: u16) {
        assert_eq!((0x00, 0x00), name_table.tile(row_offset+0,col_offset+0));
        assert_eq!((0x01, 0x00), name_table.tile(row_offset+0,col_offset+1));
        assert_eq!((0x02, 0x01), name_table.tile(row_offset+0,col_offset+2));
        assert_eq!((0x03, 0x01), name_table.tile(row_offset+0,col_offset+3));

        assert_eq!((0x20, 0x00), name_table.tile(row_offset+1,col_offset+0));
        assert_eq!((0x21, 0x00), name_table.tile(row_offset+1,col_offset+1));
        assert_eq!((0x22, 0x01), name_table.tile(row_offset+1,col_offset+2));
        assert_eq!((0x23, 0x01), name_table.tile(row_offset+1,col_offset+3));

        assert_eq!((0x40, 0x02), name_table.tile(row_offset+2,col_offset+0));
        assert_eq!((0x41, 0x02), name_table.tile(row_offset+2,col_offset+1));
        assert_eq!((0x42, 0x03), name_table.tile(row_offset+2,col_offset+2));
        assert_eq!((0x43, 0x03), name_table.tile(row_offset+2,col_offset+3));

        assert_eq!((0x60, 0x02), name_table.tile(row_offset+3,col_offset+0));
        assert_eq!((0x61, 0x02), name_table.tile(row_offset+3,col_offset+1));
        assert_eq!((0x62, 0x03), name_table.tile(row_offset+3,col_offset+2));
        assert_eq!((0x63, 0x03), name_table.tile(row_offset+3,col_offset+3));

        //Quadrants of 0x23C1
        assert_eq!((0x04, 0x03), name_table.tile(row_offset+0,col_offset+ 4));
        assert_eq!((0x05, 0x03), name_table.tile(row_offset+0,col_offset+ 5));
        assert_eq!((0x24, 0x03), name_table.tile(row_offset+1,col_offset+ 4));
        assert_eq!((0x25, 0x03), name_table.tile(row_offset+1,col_offset+ 5));

        assert_eq!((0x06, 0x02), name_table.tile(row_offset+0,col_offset+ 6));
        assert_eq!((0x07, 0x02), name_table.tile(row_offset+0,col_offset+ 7));
        assert_eq!((0x26, 0x02), name_table.tile(row_offset+1,col_offset+ 6));
        assert_eq!((0x27, 0x02), name_table.tile(row_offset+1,col_offset+ 7));

        assert_eq!((0x44, 0x01), name_table.tile(row_offset+2,col_offset+ 4));
        assert_eq!((0x45, 0x01), name_table.tile(row_offset+2,col_offset+ 5));
        assert_eq!((0x64, 0x01), name_table.tile(row_offset+3,col_offset+ 4));
        assert_eq!((0x65, 0x01), name_table.tile(row_offset+3,col_offset+ 5));

        assert_eq!((0x46, 0x00), name_table.tile(row_offset+2,col_offset+ 6));
        assert_eq!((0x47, 0x00), name_table.tile(row_offset+2,col_offset+ 7));
        assert_eq!((0x66, 0x00), name_table.tile(row_offset+3,col_offset+ 6));
        assert_eq!((0x67, 0x00), name_table.tile(row_offset+3,col_offset+ 7));


        //Quadrants of 0x23C9
        assert_eq!((0x84, 0x00), name_table.tile(row_offset+4,col_offset+ 4));
        assert_eq!((0x85, 0x00), name_table.tile(row_offset+4,col_offset+ 5));
        assert_eq!((0xA4, 0x00), name_table.tile(row_offset+5,col_offset+ 4));
        assert_eq!((0xA5, 0x00), name_table.tile(row_offset+5,col_offset+ 5));

        assert_eq!((0x86, 0x01), name_table.tile(row_offset+4,col_offset+ 6));
        assert_eq!((0x87, 0x01), name_table.tile(row_offset+4,col_offset+ 7));
        assert_eq!((0xA6, 0x01), name_table.tile(row_offset+5,col_offset+ 6));
        assert_eq!((0xA7, 0x01), name_table.tile(row_offset+5,col_offset+ 7));

        assert_eq!((0xC4, 0x02), name_table.tile(row_offset+6,col_offset+ 4));
        assert_eq!((0xC5, 0x02), name_table.tile(row_offset+6,col_offset+ 5));
        assert_eq!((0xE4, 0x02), name_table.tile(row_offset+7,col_offset+ 4));
        assert_eq!((0xE5, 0x02), name_table.tile(row_offset+7,col_offset+ 5));

        assert_eq!((0xC6, 0x03), name_table.tile(row_offset+6,col_offset+ 6));
        assert_eq!((0xC7, 0x03), name_table.tile(row_offset+6,col_offset+ 7));
        assert_eq!((0xE6, 0x03), name_table.tile(row_offset+7,col_offset+ 6));
        assert_eq!((0xE7, 0x03), name_table.tile(row_offset+7,col_offset+ 7));
    }
}

