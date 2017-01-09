mod screen;

use nes::memory::Memory;
use nes::ppu::screen::Screen;
use nes::ppu::screen::COLOUR_PALETTE;

struct PPUCtrl {
    value: u8,
}

impl PPUCtrl {
    fn new() -> PPUCtrl { PPUCtrl {value: 0} }
    fn base_name_table(&self) -> u16 {
        (self.value & 0x03) as u16
    }

    fn background_pattern_table(&self) -> u16 {
        (self.value & 0x10) as u16
    }
}

pub struct PPU<'a> {
    control_register: PPUCtrl,
    memory: Box<Memory>,
    screen: &'a mut Screen
}

impl<'a> PPU<'a> {
    fn new(memory: Box<Memory>, screen: &'a mut Screen) -> PPU<'a> {
        PPU {
            control_register: PPUCtrl::new(),
            memory: memory,
            screen: screen,
        }
    }

    fn set_ppu_ctrl(&mut self, value: u8) {
        self.control_register.value = value;
    }

    fn draw(&mut self) {
        let base_x = 0;
        let base_y = 0;

        let pattern_table_address = self.control_register.background_pattern_table();
        for row in 0..8 {
            self.draw_tile_row(base_x, base_y + row, pattern_table_address + row as u16);
        }
    }

    fn draw_tile_row(&mut self, base_x: usize, base_y: usize, pattern_table_address: u16) {
        let mut low_bits = self.memory.get(pattern_table_address);
        let mut high_bits = self.memory.get(pattern_table_address+8);
        let colour_palette_address: u16 = 0x3F00;

        for bit_index in 0..8 {
            let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);
            let colour_index = self.memory.get((colour_palette_address + pixel as u16));
            let color = COLOUR_PALETTE[colour_index as usize];
            println!("Pixel {} {:b} = {:x} {:x} {:?}", bit_index, pixel, colour_palette_address, colour_index, color);
            self.screen.set_color(base_x+bit_index, base_y, color);

            low_bits >>= 1;
            high_bits >>= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use nes::memory::Memory;
    use super::screen::Color;
    use super::screen::Screen;
    use super::PPU;

    use super::screen::BLACK;
    use super::screen::WHITE;

    struct ScreenMock {
        colors: [[Color; 30]; 32],
    }

    impl Screen for ScreenMock {
        fn set_color(&mut self, x: usize, y: usize, color: Color) {
            self.colors[y][x] = color
        }
    }

    #[test]
    fn test() {
        let mut screen = ScreenMock {colors: [[[0.0, 0.0, 0.0]; 30]; 32]};
        {
            //0b00011100
            //0b00011100
            //  00 00 00 11 11 11 00 00
            let mut ppu = PPU::new(
                Box::new(memory!(
                    //Pattern table
                        //Layer 1
                    0x0000 => 0b00011100,
                    0x0001 => 0b00110010,
                    0x0002 => 0b00111000,
                    0x0003 => 0b00011100,
                    0x0004 => 0b00001110,
                    0x0005 => 0b00100110,
                    0x0006 => 0b00011100,
                    0x0007 => 0b00000000,
                        //Layer 2
                    0x0008 => 0b00011100,
                    0x0009 => 0b00110010,
                    0x000A => 0b00111000,
                    0x000B => 0b00011100,
                    0x000C => 0b00001110,
                    0x000D => 0b00100110,
                    0x000E => 0b00011100,
                    0x000F => 0b00000000,
                    //Pattern table end

                    //Name table
                    0x2000 => 0x0000, //points to pattern table
                        //Attribute table
                    0x23C0 => 0x0000,  //points to colour palette

                    //PPU Palettes
                    0x3F00 => 0x3F,
                    0x3F01 => 0x00,
                    0x3F02 => 0x00,
                    0x3F03 => 0x20
                )),
                &mut screen
            );
            ppu.draw();
        }

        assert_eq!(screen.colors[0][0..8], [BLACK, BLACK, WHITE, WHITE, WHITE, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[1][0..8], [BLACK, WHITE, BLACK, BLACK, WHITE, WHITE, BLACK, BLACK]);
        assert_eq!(screen.colors[2][0..8], [BLACK, BLACK, BLACK, WHITE, WHITE, WHITE, BLACK, BLACK]);
        assert_eq!(screen.colors[3][0..8], [BLACK, BLACK, WHITE, WHITE, WHITE, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[4][0..8], [BLACK, WHITE, WHITE, WHITE, BLACK, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[5][0..8], [BLACK, WHITE, WHITE, BLACK, BLACK, WHITE, BLACK, BLACK]);
        assert_eq!(screen.colors[6][0..8], [BLACK, BLACK, WHITE, WHITE, WHITE, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[7][0..8], [BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK, BLACK]);
    }
}