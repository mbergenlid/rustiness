pub mod screen;
pub mod ppumemory;
pub mod vram_registers;
pub mod attributetable;

use memory::Memory;
use ppu::screen::{Screen, COLOUR_PALETTE, PixelBuffer, Rectangle};
use ppu::vram_registers::VRAMRegisters;
use ppu::attributetable::AttributeTable;

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }

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

trait Sprite {
    fn position_y(&self) -> u8;
    fn position_x(&self) -> u8;
    fn pattern_index(&self) -> u8;
    fn colour_palette(&self) -> u16;
}

impl <'a> Sprite for &'a [u8] {
    fn position_y(&self) -> u8 { return self[0]; }
    fn position_x(&self) -> u8 { return self[3]; }
    fn pattern_index(&self) -> u8 { return self[1]; }
    fn colour_palette(&self) -> u16 { return (self[2] as u16 & 0x3)*4 + 0x3F10; }
}

pub struct PPU {
    control_register: PPUCtrl,
    mask_register: PPUMask,
    status_register: u8,
    memory: Box<Memory>,
    vram_registers: VRAMRegisters,

    vram_changed: bool,

    cycle_count: u32,
    mirroring: ppumemory::Mirroring,

    sprites: [u8; 64*4],
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
            format_args!("\tVRAM Temp Pointer:     0x{:08x}\n", self.vram_registers.temporary)).unwrap();
        formatter.write_fmt(
            format_args!("\tStatus register:  0b{:08b}\t\n", self.status_register)).unwrap();
        formatter.write_fmt(
            format_args!("\tscroll (x, y):    ({}, {})\t\n", self.vram_registers.temporary_x_scroll(), self.vram_registers.temporary_y_scroll()))
    }
}

const PPU_CYCLES_PER_CPU_CYCLE: u32 = 3;
const PPU_CYCLES_PER_SCANLINE: u32 = 341;
const SCANLINES_PER_VBLANK: u32 = 20;
const SCANLINES_PER_FRAME: u32 = 262;
const PPU_CYCLES_PER_VISIBLE_FRAME: u32 = (SCANLINES_PER_FRAME-SCANLINES_PER_VBLANK)*PPU_CYCLES_PER_SCANLINE;

impl PPU {
    pub fn new(memory: Box<Memory>) -> PPU {
        PPU::with_mirroring(memory, ppumemory::Mirroring::NoMirroring)
    }

    pub fn with_mirroring(memory: Box<Memory>, mirroring: ppumemory::Mirroring) -> PPU {
        PPU {
            control_register: PPUCtrl::new(),
            mask_register: PPUMask { value: 0 },
            status_register: 0,
            memory: memory,
            vram_registers: VRAMRegisters::new(),

            vram_changed: true,

            cycle_count: 0,
            mirroring: mirroring,

            sprites: [0; 64*4],
        }
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.control_register.value = value;
        self.vram_registers.write_name_table(value);
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
        self.vram_registers.reset_write_toggle();
        return status_register;
    }

    pub fn set_vram(&mut self, value: u8) {
        self.vram_registers.set_vram(value);
    }

    pub fn vram(&self) -> u16 {
        self.vram_registers.current
    }

    pub fn set_scroll(&mut self, value: u8) {
        self.vram_registers.write_scroll(value);
    }

    pub fn write_to_vram(&mut self, value: u8) {
        self.vram_changed = true;
        self.memory.set(self.vram_registers.current, value);
        self.vram_registers.current += self.control_register.vram_pointer_increment();
    }

    pub fn load(&mut self, base_address: u16, rom: &[u8]) {
        let current_vram = self.vram_registers.current;
        self.vram_registers.current = base_address;

        for &byte in rom {
            self.write_to_vram(byte);
        }

        self.vram_registers.current = current_vram;
    }

    pub fn load_sprites(&mut self, oam_data: &[u8]) {
        self.sprites.copy_from_slice(oam_data);
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
            if self.mask_register.is_drawing_enabled() {
                self.update_screen(screen);
            }
            return false
        } else {
            return false;
        }
    }

    pub fn update_screen<T>(&mut self, screen: &mut T) where T: Screen + Sized {
        if self.vram_changed {
            self.vram_changed = false;
            screen.update_buffer(|buffer| self.draw_buffer(buffer));
        }
        self.vram_registers.copy_temporary_bits();
        let screen_width: usize = 256;
        let screen_height: usize = 240;
        let left: usize = self.vram_registers.current_absolute_x_scroll() as usize;
        let top: usize = self.vram_registers.current_absolute_y_scroll() as usize;
        let (area_width, area_height): (usize, usize) = match self.mirroring {
            ppumemory::Mirroring::Horizontal => (256, 480),
            ppumemory::Mirroring::Vertical => (512, 240),
            ppumemory::Mirroring::NoMirroring => (512, 480),
        };
        use std::cmp::min;
        screen.render(
            Rectangle { x: left as i32, y: top as i32, width: min(screen_width, area_width-left) as u32, height: min(screen_height, area_height-top) as u32 },
            0, 0
        );
        //Do we need to patch to the right?
        if area_width-left < screen_width {
            screen.render(
                Rectangle { x: 0, y: top as i32, width: (screen_width-(area_width-left)) as u32, height: min(screen_height, area_height-top) as u32 },
                area_width-left, 0
            );
        }
        //Do we need to patch at the bottom?
        if area_height-top < screen_height {
            screen.render(
                Rectangle { x: left as i32, y: 0, width: min(screen_width, area_width-left) as u32, height: (screen_height-(area_height-top)) as u32 },
                0, area_height-top as usize
            );
        }
        //Do we need to patch at the bottom right?
        if area_width-left < screen_width && area_height-top < screen_height {
            screen.render(
                Rectangle { x: 0, y: 0, width: (screen_width-(area_width-left)) as u32, height: (screen_height-(area_height-top)) as u32 },
                area_width-left as usize, area_height-top as usize
            );
        }

        //Update sprites
        self.draw_sprite(screen);

        
        screen.present();
    }

    pub fn draw_sprite<T>(&mut self, screen: &mut T) where T: Screen + Sized {
        let sprite = &self.sprites[0..4];
        screen.update_sprites(|buffer| {
            let pattern_table_base_address = 0x1000;
            let colour_palette = sprite.colour_palette();
            let mut pattern_table_address = pattern_table_base_address | ((sprite.pattern_index() as u16) << 4);
            for pattern_row in 0..8 {
                let mut low_bits = self.memory.get(pattern_table_address);
                let mut high_bits = self.memory.get(pattern_table_address+8);
                for bit_index in 0..8 {
                    let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);
                    let colour = 
                        if pixel == 0 { (0,0,0,0) } 
                        else { 
                            let colour_address = colour_palette + pixel as u16;
                            let colour = COLOUR_PALETTE[self.memory.get(colour_address) as usize];
                            (255, colour.2, colour.1, colour.0)
                        };

                    buffer.set_pixel(
                        (7-bit_index) as usize,
                        pattern_row as usize,
                        colour
                    );
                    low_bits >>= 1;
                    high_bits >>= 1;
                }
                pattern_table_address += 1;
            }
        });
        screen.render_sprite(
            Rectangle { x: 0, y: 0, width: 8, height: 8 },
            sprite.position_x() as usize,
            sprite.position_y() as usize,
        );
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &mut PixelBuffer) {
        match self.mirroring {
            ppumemory::Mirroring::Horizontal => {
                self.update_tile_for_nametable(pixel_buffer, 0);
                self.update_tile_for_nametable(pixel_buffer, 2);
            },
            ppumemory::Mirroring::Vertical => {
                self.update_tile_for_nametable(pixel_buffer, 0);
                self.update_tile_for_nametable(pixel_buffer, 1);
            },
            ppumemory::Mirroring::NoMirroring => {
                self.update_tile_for_nametable(pixel_buffer, 0);
                self.update_tile_for_nametable(pixel_buffer, 1);
                self.update_tile_for_nametable(pixel_buffer, 2);
                self.update_tile_for_nametable(pixel_buffer, 3);
            },
        };
    }

    fn update_tile_for_nametable(&mut self, pixel_buffer: &mut PixelBuffer, name_table_index: u16) {
        let pattern_table_base_address = self.control_register.background_pattern_table();
        let name_table_base = 0x2000 + name_table_index*0x400;
        let mut name_table = name_table_base;
        let x_offset: usize = ((name_table_index & 0x1)*256) as usize;
        let y_offset: usize = (((name_table_index & 0x2) >> 1)*240) as usize;
        for row in 0..30 {
            for col in 0..32 {
                let pattern_table_address = self.memory.get(name_table) as u16;
                let colour_palette = {
                    let attribute_table = AttributeTable {
                        memory: &(*self.memory),
                        address: name_table_base + 0x3C0,
                    };
                    attribute_table.get_palette_address(row, col)
                };
                let mut pattern_table_address = pattern_table_base_address | (pattern_table_address << 4);
                for pattern_row in 0..8 {
                    let mut low_bits = self.memory.get(pattern_table_address);
                    let mut high_bits = self.memory.get(pattern_table_address+8);
                    for bit_index in 0..8 {
                        let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);
                        let colour_address: u16 = if pixel == 0 { 0x3F00 } else { colour_palette + pixel as u16 };

                        let colour = COLOUR_PALETTE[self.memory.get(colour_address) as usize];
                        pixel_buffer.set_pixel(
                            x_offset + (col*8 + (7-bit_index)) as usize,
                            y_offset + (row*8 + pattern_row) as usize,
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
    use memory::BasicMemory;
    use super::screen::ScreenMock;
    use super::{PPU, PPUStatus};

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
}
