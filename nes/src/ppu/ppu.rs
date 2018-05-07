use memory::Memory;
use ppu::screen::{Screen, COLOUR_PALETTE, PixelBuffer, Rectangle};
use ppu::vram_registers::VRAMRegisters;
use ppu::ppumemory;
use ppu::ppumemory::PPUMemory;
use ppu::sprite::{Sprites,Sprite,SpritePattern,HorizontalPattern, VerticalPattern};

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
    fn is_rendering_enabled(&self) -> bool {
        self.value & 0x18 > 0
    }
    fn is_background_enabled(&self) -> bool {
        self.value & 0x08 > 0
    }
    fn is_sprite_enabled(&self) -> bool {
        self.value & 0x10 > 0
    }
    fn is_left_clipping_enabled(&self) -> bool {
        self.value & 0x06 != 0x06
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
    nmi_triggered: bool,
    nmi_active: bool,
    frame_skipped: bool,
    memory: PPUMemory,
    vram_registers: VRAMRegisters,
    temp_vram_read_buffer: u8,

    vram_changed: bool,

    cycle_count: u32,
    cycles_already_executed: u32,
    should_update_screen: bool,
    mirroring: ppumemory::Mirroring,

    sprites: Sprites,

    odd_flag: bool,
}

use std::fmt::{Formatter, Error, Display};
impl Display for PPU {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        formatter.write_fmt(format_args!("PPU: {}\t{}\n", self.cycle_count, if self.odd_flag { "Odd" } else { "Even" })).unwrap();
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
const VBLANK_CYCLE: u32 = (VISIBLE_SCANLINES+POST_RENDER_LINES)*PPU_CYCLES_PER_SCANLINE+1; //82 182
const NMI_CYCLE: u32 = VBLANK_CYCLE+3; //82 185
const VBLANK_CLEAR_CYCLE: u32 = (VISIBLE_SCANLINES+POST_RENDER_LINES+SCANLINES_PER_VBLANK)*PPU_CYCLES_PER_SCANLINE+1; //89 002

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
            nmi_triggered: false,
            nmi_active: false,
            frame_skipped: false,
            memory: memory,
            vram_registers: VRAMRegisters::new(),
            temp_vram_read_buffer: 0,

            vram_changed: true,

            cycle_count: 0,
            cycles_already_executed: 0,
            should_update_screen: false,
            mirroring: mirroring,

            sprites: Sprites::new(),

            odd_flag: false,
        }
    }

    pub fn set_ppu_ctrl_at_cycle(&mut self, value: u8, sub_cycle: u8) {
        self.partially_update((sub_cycle as u32)*PPU_CYCLES_PER_CPU_CYCLE+3);

        if !self.control_register.nmi_enabled()
                && (value & 0x80 != 0)
                && self.status_register.is_vblank() {
            self.pending_nmi = 2;
        }
        self.control_register.value = value;
        self.vram_registers.write_name_table(value);
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.set_ppu_ctrl_at_cycle(value, 0)
    }

    pub fn ppu_ctrl(&self) -> u8 {
        self.control_register.value
    }

    pub fn set_ppu_mask(&mut self, value: u8, sub_cycle: u8) {
        self.partially_update((sub_cycle as u32)*PPU_CYCLES_PER_CPU_CYCLE+3);
        self.mask_register.value = value;
    }

    pub fn status(&mut self, sub_cycle: u8) -> u8 {
        self.partially_update((sub_cycle as u32)*PPU_CYCLES_PER_CPU_CYCLE+2);
        let status_register = self.status_register;
        self.status_register &= 0x7F;
        self.vram_registers.reset_write_toggle();
        if self.cycle_count == VBLANK_CYCLE-1 {
            self.vblank_triggered = true;
            self.nmi_triggered = true;
        } else if self.cycle_count == VBLANK_CYCLE || self.cycle_count == VBLANK_CYCLE+1 {
            self.nmi_triggered = true;
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
        self.memory.set(self.vram_registers.current, value, 0);
        self.vram_registers.current += self.control_register.vram_pointer_increment();
    }

    pub fn read_from_vram(&mut self) -> u8 {
        let current_vram = self.vram_registers.current;
        let value = if current_vram >= 0x3F00 {
            self.temp_vram_read_buffer = self.memory.get(current_vram - 0x1000, 0);
            self.memory.get(current_vram, 0)
        } else {
            let value = self.temp_vram_read_buffer;
            self.temp_vram_read_buffer = self.memory.get(current_vram, 0);
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

    pub fn sprites_mut(&mut self) -> &mut Sprites {
        &mut self.sprites
    }

    pub fn sprites(&self) -> &Sprites {
        &self.sprites
    }

    fn partially_update(&mut self, ppu_cycles: u32) {
        self.update(ppu_cycles);
        self.cycles_already_executed = ppu_cycles;
    }

    fn update(&mut self, ppu_cycle_count: u32) {
        self.cycle_count += ppu_cycle_count;
        if !self.vblank_triggered && self.cycle_count >= VBLANK_CYCLE {
            //VBLANK
            self.status_register = self.status_register | 0x80; //set vblank
            self.vblank_triggered = true;
            self.should_update_screen = true;
            if !self.nmi_triggered && self.cycle_count >= NMI_CYCLE {
                self.nmi_triggered = true;
                self.nmi_active = self.control_register.nmi_enabled();
            }
        } else if !self.nmi_triggered && self.cycle_count >= NMI_CYCLE {
            self.nmi_triggered = true;
            self.nmi_active = self.control_register.nmi_enabled();
        } else if !self.vblank_cleared && self.cycle_count >= VBLANK_CLEAR_CYCLE {
            //VBLANK is over
            self.status_register = self.status_register & 0x3F;
            self.vblank_cleared = true;
        } else if !self.frame_skipped && self.cycle_count >= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE-2 {
            self.odd_flag = !self.odd_flag;
            self.frame_skipped = true;
            if self.mask_register.is_rendering_enabled() && self.odd_flag {
                self.cycle_count += 1;
            }
        }
        if self.cycle_count >= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE {
            self.cycle_count -= SCANLINES_PER_FRAME*PPU_CYCLES_PER_SCANLINE;
            self.vblank_triggered = false;
            self.vblank_cleared = false;
            self.nmi_triggered = false;
            self.frame_skipped = false;

            if self.determine_sprite_0_hit_cycle() < 0xFFFFFFFF {
                if self.mask_register.is_background_enabled()
                        && self.mask_register.is_sprite_enabled() {
                    self.status_register |= 0x40;
                }
            }
        }
    }

    fn determine_sprite_0_hit_cycle(&self) -> u64 {
        let sprite_0 = &self.sprites[0];

        let pattern_base_index = (self.control_register.sprite_pattern_table() >> 4) as usize;
        let sprite_pattern_0  =
            &self.memory.patterns()[pattern_base_index + sprite_0.pattern_index() as usize];
        if sprite_0.flip_horizontal() && sprite_0.flip_vertical() {
            self.determine_sprite_0_hit_cycle_from_sprite(
                &sprite_0,
                &HorizontalPattern::new(VerticalPattern::new(sprite_pattern_0))
            )
        } else if sprite_0.flip_horizontal() {
            self.determine_sprite_0_hit_cycle_from_sprite(
                &sprite_0,
                &HorizontalPattern::new(sprite_pattern_0)
            )
        } else if sprite_0.flip_vertical() {
            self.determine_sprite_0_hit_cycle_from_sprite(
                &sprite_0,
                &VerticalPattern::new(sprite_pattern_0)
            )
        } else {
            self.determine_sprite_0_hit_cycle_from_sprite(
                &sprite_0,
                sprite_pattern_0
            )
        }
    }

    fn determine_sprite_0_hit_cycle_from_sprite(
        &self,
        sprite_0: &Sprite,
        sprite_pattern: &SpritePattern
    ) -> u64 {
        let name_table = self.memory.name_table();
        let bg_pattern_base_index = self.control_register.background_pattern_table() as usize;
        let bg_patterns =
            &self.memory.patterns()[bg_pattern_base_index..(bg_pattern_base_index+0x100)];

        let mut absolute_y = sprite_0.position_y().wrapping_add(1) as u16;
        for py in 0..8 {
            let absolute_x = sprite_0.position_x() as u16;
            let px_start =
                if sprite_0.position_x() < 8 && self.mask_register.is_left_clipping_enabled() {
                    8 - sprite_0.position_x()
                } else {
                    0
                };
            for px in px_start..8 {
                if sprite_pattern.pixel(px,py) != 0
                    && name_table.pixel(absolute_x+(px as u16),absolute_y,&bg_patterns) != 0 {

                        return 0;
                }
            }
            absolute_y += 1;
        }
        return 0xFFFFFFFF;
    }

    /**
     * Returns true if a VBLANK should be generated.
     */
    pub fn sync<T>(&mut self, cpu_cycle_count: u32, screen: &mut T) -> bool
        where T: Screen + Sized
    {
        let remaining_cycles = cpu_cycle_count*PPU_CYCLES_PER_CPU_CYCLE - self.cycles_already_executed;
        self.cycles_already_executed = 0;
        self.update(remaining_cycles);
        if self.should_update_screen {
            if cfg!(feature = "ppu") {
                if self.mask_register.is_rendering_enabled() {
                    self.update_screen(screen);
                }
            }
            self.should_update_screen = false;
        }
        if self.nmi_active {
            self.nmi_active = false;
            return true;
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
        screen.set_backdrop_color(COLOUR_PALETTE[self.memory.get(0x3F00, 0) as usize]);
        self.sprites.render_back_sprites(screen);
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
        self.sprites.render_front_sprites(screen);

        screen.present();
    }

    fn update_sprites(&mut self, buffer: &mut PixelBuffer) {
        for sprite_index in 0..64 {
            let sprite = &self.sprites[sprite_index];
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

        assert_eq!(true, ppu.status(0).is_vblank());
        assert_eq!(0b0100_0000, ppu.status_register);
        assert_eq!(false, ppu.status(0).is_vblank());
    }

    #[test]
    fn should_not_cause_nmi_if_disabled() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x00); //Disable NMI

        assert_eq!(false, ppu.sync(29_000, &mut ScreenMock::new()));
    }

    #[test]
    fn test_vblank() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x80);
        assert_eq!(false, ppu.sync(45, screen)); //cycle count = 135
        assert_eq!(false, ppu.sync(27_394-45, screen)); //cycle count = 82_182

        assert_eq!(true, ppu.status_register.is_vblank());
        assert_eq!(true, ppu.sync(1, screen)); //cycle count = 82_185

        assert_eq!(false, ppu.sync(49, screen)); //cycle count = 82 332
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.sync(2_224, screen));  //cycle count = 89 004
        assert_eq!(false, ppu.status_register.is_vblank());

        //89 342 ppu cycles per frame
        //Total cpu cycles 29_781 = 89_343 ppu cycles
        assert_eq!(false, ppu.sync(113+45, screen)); // cycle count = 136

        assert_eq!(true, ppu.sync(27_462, screen)); //cycle count = 82 522
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.sync(50, screen)); //cycle count = 82 672
        assert_eq!(true, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.sync(2_223, screen)); //cycle count = 89 341
        assert_eq!(false, ppu.status_register.is_vblank());
    }

    #[test]
    fn test_vblank_cleared_manually() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());
        ppu.set_ppu_ctrl(0x80);
        assert_eq!(true, ppu.sync(27_508, screen)); //cycle count = 82_524
        assert_eq!(true, ppu.status_register.is_vblank());

        ppu.status(0); //To clear vblank
        assert_eq!(false, ppu.status_register.is_vblank());

        assert_eq!(false, ppu.sync(5, screen));
    }

    #[test]
    #[allow(non_snake_case)]
    fn read_status_one_PPU_clock_before_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.set_ppu_ctrl(0x80);
        update_ppu(27393, &mut ppu); //82_179 = VBL-3
        //The following 'status' read will happen 2 ppu cycles later (i.e at 82_181)
        assert_eq!(0x00, ppu.status(0) & 0x80); //Reads one PPU clock before vbl suppresses vbl for this frame
        assert_eq!(false, ppu.sync(4, screen)); //82_191
        assert_eq!(0x00, ppu.status(0) & 0x80); //VBL has been suppressed by previous read
    }

    #[test]
    #[allow(non_snake_case)]
    fn read_status_on_the_same_PPU_clock_as_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.set_ppu_ctrl(0x80);
        update_ppu(29781+27393, &mut ppu); //82_180 = VBL-2
        //The following 'status' read will happen 2 ppu cycles later (i.e at 82_182)
        assert_eq!(0x80, ppu.status(0) & 0x80); //Reads status exactly on vbl suppresses nmi
        assert_eq!(false, ppu.sync(2, screen));
    }

    #[test]
    #[allow(non_snake_case)]
    fn read_status_one_PPU_clock_after_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.set_ppu_ctrl(0x80);
        update_ppu(27392+29781*4, &mut ppu); //82_180 = VBL-1
        //The following 'status' read will happen 2 ppu cycles later (i.e at 82_182)
        assert_eq!(0x80, ppu.status(0) & 0x80); //Reads status exactly on vbl suppresses nmi
        assert_eq!(false, ppu.sync(2, screen));
    }

    #[test]
    fn nmi_should_occur_immediately_after_next_instruction_if_enabled_when_vbl_is_set() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.sync(27_393, screen); //82_179  VBL-2
        ppu.sync(200, screen); //82_779 (in VBL)
        assert_eq!(true, ppu.status_register.is_vblank());

        ppu.set_ppu_ctrl(0x80); //Enable NMI
        assert_eq!(false, ppu.sync(1, screen));
        assert_eq!(true, ppu.sync(1, screen));
    }

    #[test]
    fn nmi_should_not_occur_immediately_after_next_instruction_if_enabled_outside_of_vbl() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        update_ppu(29_781, &mut ppu);
        update_ppu(29_665, &mut ppu); //88_995 (in VBL)
        assert_eq!(true, ppu.status_register.is_vblank());

        ppu.set_ppu_ctrl_at_cycle(0x80, 1); //Enable NMI
        assert_eq!(false, ppu.sync(2, screen));
        assert_eq!(false, ppu.sync(1, screen));
    }

    #[test]
    fn read_status_occuring_mid_instruction() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        update_ppu(29_781+27_393, &mut ppu); //82_180
        assert_eq!(0x80, ppu.status(0) & 0x80); //82_182
    }

    fn update_ppu(cycles: u32, ppu: &mut PPU) {
        let screen = &mut ScreenMock::new();
        for _ in 0..cycles {
            ppu.sync(1, screen);
        }
    }

    #[test]
    fn the_update_after_a_status_read_should_subtract_the_cycles_already_consumed_by_the_status_read() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.sync(27_390, screen); //82_170
        assert_eq!(0x00, ppu.status(2) & 0x80); //82_178
        ppu.sync(3, screen); //82_179
        assert_eq!(false, ppu.status_register.is_vblank());
    }

    #[test]
    fn cycles_already_executed_must_be_cleared() {
        let screen = &mut ScreenMock::new();
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.sync(27_389, screen); //82_167
        assert_eq!(0x00, ppu.status(2) & 0x80); //82_175
        println!("{}", ppu);
        ppu.sync(3, screen); //82_176
        println!("{}", ppu);
        ppu.sync(2, screen); //82_182
        println!("{}", ppu);
        assert_eq!(true, ppu.status_register.is_vblank());
    }

    #[test]
    fn even_odd_frames() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.set_ppu_mask(0x18, 0);
        update_ppu(29781+27394, &mut ppu); //82_183
        assert_eq!(true, ppu.status_register.is_vblank());

        update_ppu(29780, &mut ppu); //82_182 (82_181 but odd frame should skip 1 cycle)
        assert_eq!(true, ppu.status_register.is_vblank());
    }

    #[test]
    fn odd_frames_should_not_skip_one_cycle_if_rendering_is_disabled() {
        let mut ppu = PPU::new(PPUMemory::no_mirroring());

        ppu.set_ppu_mask(0x00, 0);
        update_ppu(27394, &mut ppu); //82_182
        assert_eq!(true, ppu.status_register.is_vblank());

        update_ppu(29780, &mut ppu); //82_180
        assert_eq!(false, ppu.status_register.is_vblank());
    }
}
