pub mod screen;
pub mod ppumemory;

use memory::Memory;
use ppu::screen::{Screen, Pattern, Tile};

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }

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

trait PPUStatus {
    fn is_vblank(&self) -> bool;
}

impl PPUStatus for u8 {
    fn is_vblank(&self) -> bool {
        return self & 0x80 != 0;
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
    status_register: u8,
    memory: Box<Memory>,
    screen: Box<Screen>,
    vram_pointer: u16,
    vram_high_byte: bool,

    pattern_tables_changed: bool,
    name_tables_changed: bool,

    cycle_count: u32,
}

const PPU_CYCLES_PER_CPU_CYCLE: u32 = 3;
const PPU_CYCLES_PER_SCANLINE: u32 = 341;
const SCANLINES_PER_VBLANK: u32 = 20;
const SCANLINES_PER_FRAME: u32 = 262;
const PPU_CYCLES_PER_VISIBLE_FRAME: u32 = (SCANLINES_PER_FRAME-SCANLINES_PER_VBLANK)*PPU_CYCLES_PER_SCANLINE;

impl PPU {
    pub fn new(memory: Box<Memory>, screen: Box<Screen>) -> PPU {
        PPU {
            control_register: PPUCtrl::new(),
            mask_register: PPUMask { value: 0 },
            status_register: 0,
            memory: memory,
            screen: screen,
            vram_pointer: 0,
            vram_high_byte: true, //Big Endian

            pattern_tables_changed: true,
            name_tables_changed: true,

            cycle_count: 0,
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

    pub fn status(&mut self) -> u8 {
        let status_register = self.status_register;
        self.status_register &= 0x7F;
        return status_register;
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

    /**
     * Returns true if a VBLANK should be generated.
     */
    pub fn update(&mut self, cpu_cycle_count: u32) -> bool {
        self.cycle_count += cpu_cycle_count * PPU_CYCLES_PER_CPU_CYCLE;
        if !self.status_register.is_vblank() && self.cycle_count >= PPU_CYCLES_PER_VISIBLE_FRAME {
            self.status_register = self.status_register | 0x80;
            return true;
        } else if self.cycle_count >= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE {
            self.status_register = self.status_register & 0x7F;
            self.cycle_count -= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE;
            self.draw();
            return false
        } else {
            return false;
        }
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
                self.update_tile_for_nametable(0);
                self.update_tile_for_nametable(1);
                self.update_tile_for_nametable(2);
                self.update_tile_for_nametable(3);
                self.name_tables_changed = false;
            }
            self.screen.draw();
        }
    }

    fn update_tile_for_nametable(&mut self, name_table_index: u16) {
        let name_table_base = 0x2000 + name_table_index*0x400;
        let mut name_table = name_table_base;
        for row in 0..30 {
            for col in 0..32 {
                let pattern_table_address = self.memory.get(name_table) as u32;
                let colour_palette_index = {
                    let attribute_table = AttributeTable {
                        memory: &(*self.memory),
                        address: name_table_base + 0x3C0,
                    };
                    attribute_table.get_palette_index(row, col)
                };
                self.screen.update_tile(
                    (32*(name_table_index % 2) + col) as usize,
                    (30*(name_table_index / 2) + row) as usize,
                    &Tile {
                        pattern_index: pattern_table_address,
                        palette_index: colour_palette_index
                    }
                );

                name_table += 1;
            }
        }
    }

    pub fn memory(&self) -> &Memory {
        self.memory.as_ref()
    }
}

#[cfg(test)]
pub mod tests {
    use memory::{Memory, BasicMemory};
    use super::screen::ScreenMock;
    use super::{PPU, PPUStatus};

    use super::AttributeTable;

    #[test]
    fn reading_status_register_should_clear_vblank() {
        let mut ppu = PPU::new(box BasicMemory::new(), box ScreenMock::new());
        ppu.status_register = 0b1100_0000;

        assert_eq!(true, ppu.status().is_vblank());
        assert_eq!(0b0100_0000, ppu.status_register);
    }

    #[test]
    fn test_vblank() {
        let mut ppu = PPU::new(box BasicMemory::new(), box ScreenMock::new());
        assert_eq!(false, ppu.update(45)); //cycle count = 135
        assert_eq!(true, ppu.update(27_508-45)); //cycle count = 82_524

        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50)); //cycle count = 82 674
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_223));  //cycle count = 89 343
        assert_eq!(false, ppu.status_register.is_vblank());

        //89 342 ppu cycles per frame
        //Total cpu cycles 29_781 = 89_343 ppu cycles
        assert_eq!(false, ppu.update(45)); // cycle count = 136

        assert_eq!(true, ppu.update(27_462)); //cycle count = 82 522
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50)); //cycle count = 82 672
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_223)); //cycle count = 89 341
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(1));
        assert_eq!(false, ppu.status_register.is_vblank());
    }

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
}
