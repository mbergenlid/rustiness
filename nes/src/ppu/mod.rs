pub mod screen;
pub mod ppumemory;

use memory::Memory;
use ppu::screen::{Screen, COLOUR_PALETTE, PixelBuffer};

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }

    fn name_table_base(&self) -> u16 {
        0x2000 + 0x400*((self.value & 0x03) as u16)
    }

    fn background_pattern_table(&self) -> u16 {
        ((self.value & 0x10) as u16) << 8
    }

    fn nmi_enabled(&self) -> bool {
        (self.value & 0x80) != 0
    }

    fn vram_pointer_increment(&self) -> u16 {
        if self.value & 0x04 == 0 { 1 } else { 32 }
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
    pub memory: &'a Memory,
    pub address: u16
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
    vram_pointer: u16,
    vram_high_byte: bool,

    x_scroll: u8,
    y_scroll: u8,

    vram_changed: bool,

    cycle_count: u32,
}

use std::fmt::{Formatter, Error, Display};
impl Display for PPU {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_str("PPU:\n").unwrap();
        formatter.write_fmt(
            format_args!("\tControl register: 0b{:08b}\n", self.ppu_ctrl())).unwrap();
        formatter.write_fmt(
            format_args!("\tVRAM Pointer:     0x{:08x}\t\n", self.vram())).unwrap();
        formatter.write_fmt(
            format_args!("\tStatus register:  0b{:08b}\t\n", self.status_register)).unwrap();
        formatter.write_fmt(
            format_args!("\tscroll (x, y):    ({}, {})\t\n", self.x_scroll, self.y_scroll))
    }
}

const PPU_CYCLES_PER_CPU_CYCLE: u32 = 3;
const PPU_CYCLES_PER_SCANLINE: u32 = 341;
const SCANLINES_PER_VBLANK: u32 = 20;
const SCANLINES_PER_FRAME: u32 = 262;
const PPU_CYCLES_PER_VISIBLE_FRAME: u32 = (SCANLINES_PER_FRAME-SCANLINES_PER_VBLANK)*PPU_CYCLES_PER_SCANLINE;

impl PPU {
    pub fn new(memory: Box<Memory>) -> PPU {
        PPU {
            control_register: PPUCtrl::new(),
            mask_register: PPUMask { value: 0 },
            status_register: 0,
            memory: memory,
            vram_pointer: 0,
            x_scroll: 0,
            y_scroll: 0,
            vram_high_byte: true, //Big Endian

            vram_changed: true,

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
        self.vram_high_byte = true;
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

    pub fn set_scroll(&mut self, value: u8) {
        if self.vram_high_byte {
            self.x_scroll = value;
            self.vram_high_byte = false;
        } else {
            self.y_scroll = value;
            self.vram_high_byte = true;
        }
    }

    pub fn write_to_vram(&mut self, value: u8) {
        self.vram_changed = true;
        self.memory.set(self.vram_pointer, value);
        self.vram_pointer += self.control_register.vram_pointer_increment();
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
    pub fn update<T>(&mut self, cpu_cycle_count: u32, screen: &mut T) -> bool
        where T: Screen + Sized
    {
        self.cycle_count += cpu_cycle_count * PPU_CYCLES_PER_CPU_CYCLE;
        if !self.status_register.is_vblank() && self.cycle_count >= PPU_CYCLES_PER_VISIBLE_FRAME {
            self.status_register = self.status_register | 0x80;
            return self.control_register.nmi_enabled();
        } else if self.cycle_count >= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE {
            self.status_register = self.status_register & 0x7F;
            self.cycle_count -= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE;
            if self.mask_register.is_drawing_enabled() && self.vram_changed {
                self.vram_changed = false;
                screen.draw(|buffer| self.draw_buffer(buffer));
            }
            return false
        } else {
            return false;
        }
    }


    pub fn draw_buffer(&self, pixel_buffer: &mut PixelBuffer) {
        let pattern_table_base_address = self.control_register.background_pattern_table();
        let name_table_base = self.control_register.name_table_base();
        let mut name_table = name_table_base;
        for tile_y in 0..30 {
            for tile_x in 0..32 {
                let pattern_table_address = self.memory.get(name_table) as u16;
                let colour_palette = {
                    let attribute_table = AttributeTable {
                        memory: &(*self.memory),
                        address: name_table_base + 0x3C0,
                    };
                    attribute_table.get_palette_address(tile_y, tile_x)
                };

                let mut pattern_table_address = pattern_table_base_address | (pattern_table_address << 4);
                for pattern_row in 0..8 {
                    let mut low_bits = self.memory.get(pattern_table_address);
                    let mut high_bits = self.memory.get(pattern_table_address+8);
                    for bit_index in 0..8 {
                        let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);
                        let colour_address = if pixel == 0 { 0x3F00 } else { colour_palette + pixel as u16 };

                        let colour = COLOUR_PALETTE[self.memory.get(colour_address) as usize];
                        pixel_buffer.set_pixel(
                            (tile_x*8 + (7-bit_index)) as usize,
                            (tile_y*8 + pattern_row) as usize,
                            colour
                        );
                        low_bits >>= 1;
                        high_bits >>= 1;
                    }
                    pattern_table_address += 1;
                }

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
    fn reading_status_register_should_reset_vram_high_byte() {
        let mut ppu = PPU::new(box BasicMemory::new());

        ppu.set_vram(1);
        assert_eq!(false, ppu.vram_high_byte);
        ppu.status();
        assert_eq!(true, ppu.vram_high_byte);
    }

    #[test]
    fn reading_status_register_should_clear_vblank() {
        let mut ppu = PPU::new(box BasicMemory::new());
        ppu.status_register = 0b1100_0000;

        assert_eq!(true, ppu.status().is_vblank());
        assert_eq!(0b0100_0000, ppu.status_register);
    }

    #[test]
    fn should_not_cause_nmi_if_disabled() {
        let mut ppu = PPU::new(box BasicMemory::new());
        ppu.set_ppu_ctrl(0x00); //Disable NMI

        assert_eq!(false, ppu.update(29_000, &mut ScreenMock::new()));
    }

    #[test]
    fn test_vblank() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(box BasicMemory::new());
        ppu.set_ppu_ctrl(0x80);
        assert_eq!(false, ppu.update(45, screen)); //cycle count = 135
        assert_eq!(true, ppu.update(27_508-45, screen)); //cycle count = 82_524

        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50, screen)); //cycle count = 82 674
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_223, screen));  //cycle count = 89 343
        assert_eq!(false, ppu.status_register.is_vblank());

        //89 342 ppu cycles per frame
        //Total cpu cycles 29_781 = 89_343 ppu cycles
        assert_eq!(false, ppu.update(45, screen)); // cycle count = 136

        assert_eq!(true, ppu.update(27_462, screen)); //cycle count = 82 522
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50, screen)); //cycle count = 82 672
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_223, screen)); //cycle count = 89 341
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(1, screen));
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
