
use nes::ppu::screen::PixelBuffer;
use nes::ppu::vram_registers::VRAMRegisters;

struct PPU2 {
    pixel_buffer: PixelBuffer,
    vram_registers: VRAMRegisters,

    background_tiles_changed: bool,
}

impl PPU2 {

    pub fn write_to_vram(&mut self, value: u8) {
        self.vram_changed = true;
        self.memory.set(self.vram_registers.current, value);
        if self.vram_registers.current < 0x2000 {
            //Update pattern table
        } else if self.vram_registers.current < 0x3000 {
            //Update name table

        } else if self.vram_registers.current >= 0x3F00 {

        }
        self.vram_registers.current += self.control_register.vram_pointer_increment();
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

    pub fn update_screen(&mut self, screen: &mut T) where T: Screen + Sized {

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

}