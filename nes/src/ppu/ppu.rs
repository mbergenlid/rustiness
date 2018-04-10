use memory::Memory;
use ppu::screen::{Screen, COLOUR_PALETTE, PixelBuffer, Rectangle};
use ppu::vram_registers::VRAMRegisters;
use ppu::ppumemory;
use ppu::ppumemory::PPUMemory;
use ppu::sprite::Sprite;

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }

    fn background_pattern_table(&self) -> u16 {
        ((self.value & 0x10) as u16) << 4
    }

    fn sprite_pattern_table(&self) -> u16 {
        ((self.value & 0x08) as u16) << 9
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

pub struct PPU {
    control_register: PPUCtrl,
    mask_register: PPUMask,
    status_register: u8,
    vblank_triggered: bool,
    vblank_cleared: bool,
    pending_nmi: u32,
    memory: PPUMemory,
    vram_registers: VRAMRegisters,
    temp_vram_read_buffer: u8,

    vram_changed: bool,

    cycle_count: u32,
    mirroring: ppumemory::Mirroring,

    sprites: [u8; 64*4],
    oam_address: u8,
}

use std::fmt::{Formatter, Error, Display};
impl Display for PPU {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_fmt(format_args!("PPU: {}\n", self.cycle_count)).unwrap();
        formatter.write_fmt(
            format_args!("\tControl register: 0b{:08b}\n", self.ppu_ctrl())).unwrap();
        formatter.write_fmt(
            format_args!("\tMask register: 0b{:08b}\n", self.mask_register.value)).unwrap();
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
const VISIBLE_SCANLINES: u32 = 240;
const POST_RENDER_LINES: u32 = 1;
const VBLANK_CYCLE: u32 = (VISIBLE_SCANLINES+POST_RENDER_LINES)*PPU_CYCLES_PER_SCANLINE; //82 181
const VBLANK_CLEAR_CYCLE: u32 = (VISIBLE_SCANLINES+POST_RENDER_LINES+SCANLINES_PER_VBLANK)*PPU_CYCLES_PER_SCANLINE; //89 001

impl PPU {
    pub fn new(memory: PPUMemory) -> PPU {
        let mirroring = memory.mirroring();
        PPU {
            control_register: PPUCtrl::new(),
            mask_register: PPUMask { value: 0 },
            status_register: 0,
            vblank_triggered: false,
            vblank_cleared: false,
            pending_nmi: 0,
            memory: memory,
            vram_registers: VRAMRegisters::new(),
            temp_vram_read_buffer: 0,

            vram_changed: true,

            cycle_count: 0,
            mirroring: mirroring,

            sprites: [0; 64*4],
            oam_address: 0,
        }
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        if !self.control_register.nmi_enabled() 
                && (value & 0x80 != 0) 
                && self.status_register.is_vblank() {
            self.pending_nmi = 2;
        }
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
        if self.cycle_count == VBLANK_CYCLE-1 {
            self.vblank_triggered = true;
        }
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

    pub fn read_from_vram(&mut self) -> u8 {
        let current_vram = self.vram_registers.current;
        let value = if current_vram >= 0x3F00 {
            self.temp_vram_read_buffer = self.memory.get(current_vram - 0x1000);
            self.memory.get(current_vram)
        } else {
            let value = self.temp_vram_read_buffer;
            self.temp_vram_read_buffer = self.memory.get(current_vram);
            value
        };
        self.vram_registers.current += self.control_register.vram_pointer_increment();
        return value;
    }

    pub fn load(&mut self, base_address: u16, rom: &[u8]) {
        let current_vram = self.vram_registers.current;
        self.vram_registers.current = base_address;

        for &byte in rom {
            self.write_to_vram(byte);
        }

        self.vram_registers.current = current_vram;
    }

    pub fn sprites(&mut self) -> &mut [u8] {
        &mut self.sprites
    }

    pub fn oam_address(&mut self, value: u8) {
        self.oam_address = value;
    }

    pub fn get_oam_address(&mut self) -> u8 {
        self.oam_address
    }

    pub fn oam_data(&mut self, value: u8) {
        let oam_address = self.oam_address;
        self.oam_address = self.oam_address.wrapping_add(1);
        self.sprites[oam_address as usize] = value;
    }

    pub fn get_oam_data(&self) -> u8 {
        self.sprites[self.oam_address as usize]
    }

    pub fn update<T>(&mut self, cpu_cycle_count: u32, screen: &mut T) -> bool
        where T: Screen + Sized
    {
        self.update_ppu_cycles(cpu_cycle_count*PPU_CYCLES_PER_CPU_CYCLE, screen)
    }

    /**
     * Returns true if a VBLANK should be generated.
     */
    pub fn update_ppu_cycles<T>(&mut self, ppu_cycle_count: u32, screen: &mut T) -> bool
        where T: Screen + Sized
    {
        self.cycle_count += ppu_cycle_count;
        if self.cycle_count >= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE {
            self.cycle_count -= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE;
            self.vblank_triggered = false;
            self.vblank_cleared = false;
        }
        if !self.vblank_triggered && self.cycle_count >= VBLANK_CYCLE {
            //VBLANK
            self.status_register = self.status_register | 0x80; //set vblank
            self.vblank_triggered = true;
            if cfg!(feature = "ppu") {
                if self.mask_register.is_drawing_enabled() {
                    self.update_screen(screen);
                }
            }
            return self.control_register.nmi_enabled();
        } else if !self.vblank_cleared && self.cycle_count >= VBLANK_CLEAR_CYCLE {
            //VBLANK is over
            self.status_register = self.status_register & 0x3F;
            self.vblank_cleared = true;
            return false;
        } else if self.pending_nmi > 0 {
            self.pending_nmi -= 1;
            return self.pending_nmi == 0;
        } else {
            return false;
        }
    }

    pub fn update_screen<T>(&mut self, screen: &mut T) where T: Screen + Sized {
        if self.vram_changed {
            self.vram_changed = false;
            screen.update_buffer(|buffer| self.draw_buffer(buffer));
        }
        screen.update_sprites(|buffer| self.update_sprites(buffer));
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
        screen.set_backdrop_color(COLOUR_PALETTE[self.memory.get(0x3F00) as usize]);
        self.render_back_sprites(screen);
        let area_width_minus_left = area_width.wrapping_sub(left);
        let area_height_minus_top = area_height.wrapping_sub(top);
        screen.render(
            Rectangle { x: left as i32, y: top as i32, width: min(screen_width, area_width_minus_left) as u32, height: min(screen_height, area_height_minus_top) as u32 },
            0, 0
        );
        //Do we need to patch to the right?
        if area_width_minus_left < screen_width {
            screen.render(
                Rectangle { x: 0, y: top as i32, width: (screen_width-area_width_minus_left) as u32, height: min(screen_height, area_height_minus_top) as u32 },
                area_width-left, 0
            );
        }
        //Do we need to patch at the bottom?
        if area_height_minus_top < screen_height {
            screen.render(
                Rectangle { x: left as i32, y: 0, width: min(screen_width, area_width_minus_left) as u32, height: (screen_height-area_height_minus_top) as u32 },
                0, area_height-top as usize
            );
        }
        //Do we need to patch at the bottom right?
        if area_width_minus_left < screen_width && area_height_minus_top < screen_height {
            screen.render(
                Rectangle { x: 0, y: 0, width: (screen_width-area_width_minus_left) as u32, height: (screen_height-area_height_minus_top) as u32 },
                area_width-left as usize, area_height-top as usize
            );
        }

        //Update sprites
        self.render_front_sprites(screen);

        screen.present();
    }

    fn update_sprites(&mut self, buffer: &mut PixelBuffer) {
        for sprite_index in 0..64 {
            let sprite = &self.sprites[(sprite_index*4)..(sprite_index*4+4)];
            let pattern_table_base_address = self.control_register.sprite_pattern_table();
            let pattern_table_address = pattern_table_base_address | ((sprite.pattern_index() as u16) << 4);
            let pattern = self.memory.patterns()[(pattern_table_address as usize) >> 4];
            pattern.update_buffer(
                buffer,
                self.memory.sprite_palette(sprite.colour_palette()),
                sprite_index*8,
                0
            );
        }
    }

    pub fn render_back_sprites<T>(&mut self, screen: &mut T) where T: Screen + Sized {
        for sprite_index in (0..64).rev() {
            let sprite = &self.sprites[(sprite_index*4)..(sprite_index*4+4)];
            let position_y = sprite.position_y();
            if sprite.is_back() && position_y < 0xFE {
                screen.render_sprite(
                    Rectangle { x: (sprite_index*8) as i32, y: 0, width: 8, height: 8 },
                    sprite.position_x() as usize,
                    (position_y + 1) as usize,
                    sprite.flip_horizontal(),
                    sprite.flip_vertical(),
                );
            }
        }

    }

    pub fn render_front_sprites<T>(&mut self, screen: &mut T) where T: Screen + Sized {
        for sprite_index in (0..64).rev() {
            let sprite = &self.sprites[(sprite_index*4)..(sprite_index*4+4)];
            let position_y = sprite.position_y();
            if sprite.is_front() && position_y < 0xFE {
                screen.render_sprite(
                    Rectangle { x: (sprite_index*8) as i32, y: 0, width: 8, height: 8 },
                    sprite.position_x() as usize,
                    (position_y + 1) as usize,
                    sprite.flip_horizontal(),
                    sprite.flip_vertical(),
                );
            }
        }
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &mut PixelBuffer) {
        let pattern_table = self.control_register.background_pattern_table() as usize;
        let patterns = &self.memory.patterns()[pattern_table..(pattern_table+0x100)];
        let palettes = self.memory.background_palette();
        let name_table = self.memory.name_table();
        match self.mirroring {
            ppumemory::Mirroring::Horizontal => {
                name_table.update_tile_for_nametable(pixel_buffer, 0, patterns, palettes);
                name_table.update_tile_for_nametable(pixel_buffer, 2, patterns, palettes);
            },
            ppumemory::Mirroring::Vertical => {
                name_table.update_tile_for_nametable(pixel_buffer, 0, patterns, palettes);
                name_table.update_tile_for_nametable(pixel_buffer, 1, patterns, palettes);
            },
            ppumemory::Mirroring::NoMirroring => {
                name_table.update_tile_for_nametable(pixel_buffer, 0, patterns, palettes);
                name_table.update_tile_for_nametable(pixel_buffer, 1, patterns, palettes);
                name_table.update_tile_for_nametable(pixel_buffer, 2, patterns, palettes);
                name_table.update_tile_for_nametable(pixel_buffer, 3, patterns, palettes);
            },
        };
    }

    pub fn invalidate_tile_cache(&mut self) {
        self.vram_changed = true;
        self.memory.name_table_mut().invalidate_tile_cache();
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }
}

#[cfg(test)]
pub mod tests {
    use ppu::screen::ScreenMock;
    use super::{PPU, PPUStatus};
    use ppu::ppumemory::PPUMemory;

    #[test]
    fn reading_status_register_should_clear_vblank() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.status_register = 0b1100_0000;

        assert_eq!(true, ppu.status().is_vblank());
        assert_eq!(0b0100_0000, ppu.status_register);
        assert_eq!(false, ppu.status().is_vblank());
    }

    #[test]
    fn should_not_cause_nmi_if_disabled() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x00); //Disable NMI

        assert_eq!(false, ppu.update(29_000, &mut ScreenMock::new()));
    }

    #[test]
    fn test_vblank() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x80);
        assert_eq!(false, ppu.update(45, screen)); //cycle count = 135
        assert_eq!(true, ppu.update(27_394-45, screen)); //cycle count = 82_182

        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50, screen)); //cycle count = 82 332
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_224, screen));  //cycle count = 89 004
        assert_eq!(false, ppu.status_register.is_vblank());

        //89 342 ppu cycles per frame
        //Total cpu cycles 29_781 = 89_343 ppu cycles
        assert_eq!(false, ppu.update(113+45, screen)); // cycle count = 136

        assert_eq!(true, ppu.update(27_462, screen)); //cycle count = 82 522
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(50, screen)); //cycle count = 82 672
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(2_223, screen)); //cycle count = 89 341
        assert_eq!(false, ppu.status_register.is_vblank());
    }

    #[test]
    fn test_vblank_cleared_manually() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x80);
        assert_eq!(true, ppu.update(27_508, screen)); //cycle count = 82_524
        assert_eq!(true, ppu.status_register.is_vblank());

        ppu.status(); //To clear vblank
        assert_eq!(false, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.update(5, screen));
    }

    #[test]
    #[allow(non_snake_case)]
    fn read_status_one_PPU_clock_before_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.update(27_393, screen); //82_179  VBL-2
        assert_eq!(0x00, ppu.status() & 0x80);
        ppu.update(4, screen); //82_191
        assert_eq!(0x80, ppu.status() & 0x80);

        ppu.update(29_781-4, screen); //82_180  VBL-1
        assert_eq!(0x00, ppu.status() & 0x80); //Reads one PPU clock before vbl suppresses vbl for this frame
        ppu.update(4, screen); //82_192
        assert_eq!(0x00, ppu.status() & 0x80); //VBL has been suppressed by previous read
    }

    #[test]
    fn nmi_should_occur_immediately_after_next_instruction_if_eabled_when_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.update(27_393, screen); //82_179  VBL-2
        ppu.update(200, screen); //82_779 (in VBL)
        assert_eq!(true, ppu.status_register.is_vblank());

        ppu.set_ppu_ctrl(0x80); //Enable NMI
        assert_eq!(false, ppu.update(1, screen));
        assert_eq!(true, ppu.update(1, screen));
    }
}
