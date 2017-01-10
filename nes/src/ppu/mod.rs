pub mod screen;

use memory::Memory;
use ppu::screen::Screen;
use ppu::screen::COLOUR_PALETTE;

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
        let attribute_row = tile_row >> 2;
        let attribute_col = tile_col >> 2;
        let row_inside_attribute = (tile_row & 0x03) >> 1;
        let col_inside_attribute = (tile_col & 0x03) >> 1;
        let quadrant = (row_inside_attribute << 1) | col_inside_attribute;
        let attribute_address = self.address + (attribute_row*8 + attribute_col);

        let value = self.memory.get(attribute_address);
        let palette_index = value >> (quadrant << 1) & 0x03;

        COLOUR_PALETTE_BASE_ADDRESS + (palette_index as u16)*4
    }
}

pub struct PPU<'a> {
    control_register: PPUCtrl,
    memory: Box<Memory>,
    screen: &'a mut Screen
}

impl<'a> PPU<'a> {
    pub fn new(memory: Box<Memory>, screen: &'a mut Screen) -> PPU<'a> {
        PPU {
            control_register: PPUCtrl::new(),
            memory: memory,
            screen: screen,
        }
    }

    pub fn set_ppu_ctrl(&mut self, value: u8) {
        self.control_register.value = value;
    }

    pub fn draw(&mut self) {
        let pattern_table_base_address = self.control_register.background_pattern_table();
        let mut name_table = self.control_register.base_name_table();

        for row in 0..32 {
            for col in 0..30 {
                let pattern_table_address = self.memory.get(name_table) as u16 + pattern_table_base_address;
                let colour_palette_address = {
                    let attribute_table = AttributeTable {
                        memory: &(*self.memory),
                        address: 0x23C0,
                    };
                    attribute_table.get_palette_address(row, col)
                };
                for pattern_row in 0..8 {
                    self.draw_tile_row(
                        (col*8) as usize,
                        (row*8 + pattern_row) as usize,
                        pattern_table_address + pattern_row as u16,
                        colour_palette_address
                    );
                }

                name_table += 1;
            }
        }
    }

    fn draw_tile_row(
        &mut self,
        base_x: usize,
        base_y: usize,
        pattern_table_address: u16,
        colour_palette_address: u16
    ) {
        let mut low_bits = self.memory.get(pattern_table_address);
        let mut high_bits = self.memory.get(pattern_table_address+8);

        for bit_index in 0..8 {
            let pixel = ((high_bits & 0x01) << 1) | (low_bits & 0x01);
            let colour_index =
                if pixel != 0 {
                    self.memory.get(colour_palette_address + pixel as u16)
                } else {
                    self.memory.get(COLOUR_PALETTE_BASE_ADDRESS)
                };
            let color = COLOUR_PALETTE[colour_index as usize];
            self.screen.set_color(base_x+bit_index, base_y, color);

            low_bits >>= 1;
            high_bits >>= 1;
        }

        self.screen.draw();
    }
}

#[cfg(test)]
mod tests {
    use memory::Memory;
    use super::screen::Color;
    use super::screen::Screen;
    use super::PPU;

    use super::screen::BLACK;
    use super::screen::WHITE;

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

    struct ScreenMock {
        colors: [[Color; 240]; 256],
    }

    impl Screen for ScreenMock {
        fn set_color(&mut self, x: usize, y: usize, color: Color) {
            self.colors[y][x] = color
        }
        fn draw(&mut self) {

        }
    }

    #[test]
    fn test() {
        let mut screen = ScreenMock {colors: [[[0.0, 0.0, 0.0]; 240]; 256]};
        {
            //0b00011100
            //0b00011100
            //  00 00 00 11 11 11 00 00
            let mut ppu = PPU::new(
                Box::new(memory!(
                    //Pattern table 1
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

                    //Pattern table 2
                        //Layer 1
                    0x0010 => 0b00011100,
                    0x0011 => 0b00000000,
                    0x0012 => 0b00111000,
                    0x0013 => 0b00011100,
                    0x0014 => 0b00001110,
                    0x0015 => 0b00100110,
                    0x0016 => 0b00011100,
                    0x0017 => 0b00000000,
                        //Layer 2
                    0x0018 => 0b00000000,
                    0x0019 => 0b00110010,
                    0x001A => 0b00111000,
                    0x001B => 0b00011100,
                    0x001C => 0b00001110,
                    0x001D => 0b00100110,
                    0x001E => 0b00011100,
                    0x001F => 0b00000000,

                    //Pattern table 3
                        //Layer 1
                    0x0020 => 0b00011100,
                    0x0021 => 0b00110010,
                    0x0022 => 0b00000000,
                    0x0023 => 0b00011100,
                    0x0024 => 0b00001110,
                    0x0025 => 0b00100110,
                    0x0026 => 0b00011100,
                    0x0027 => 0b00000000,
                        //Layer 2
                    0x0028 => 0b00011100,
                    0x0029 => 0b00000000,
                    0x002A => 0b00111000,
                    0x002B => 0b00011100,
                    0x002C => 0b00001110,
                    0x002D => 0b00100110,
                    0x002E => 0b00011100,
                    0x002F => 0b00000000,
                    //Pattern table end

                    //Name table
                    0x2000 => 0x00, //points to pattern table
                    0x2001 => 0x10, //points to pattern table
                    0x2002 => 0x20, //points to pattern table
                        //Attribute table
                    0x23C0 => 0b00_00_01_00,  //points to colour palette

                    //PPU Palettes
                    0x3F00 => 0x3F,
                    0x3F01 => 0x01,
                    0x3F02 => 0x10,
                    0x3F03 => 0x20,

                    0x3F04 => 0xFF, //invalid value (should not be referenced)
                    0x3F05 => 0x0A,
                    0x3F06 => 0x0B,
                    0x3F07 => 0x0C
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

        let color_1 = super::screen::COLOUR_PALETTE[0x01];
        let color_2 = super::screen::COLOUR_PALETTE[0x10];
        assert_eq!(screen.colors[0][8..16], [BLACK, BLACK, color_1, color_1, color_1, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[1][8..16], [BLACK, color_2, BLACK, BLACK, color_2, color_2, BLACK, BLACK]);

        let colour_1 = super::screen::COLOUR_PALETTE[0x0A];
        let colour_2 = super::screen::COLOUR_PALETTE[0x0B];
        let colour_3 = super::screen::COLOUR_PALETTE[0x0C];
        assert_eq!(screen.colors[0][16..24], [BLACK, BLACK, colour_3, colour_3, colour_3, BLACK, BLACK, BLACK]);
        assert_eq!(screen.colors[1][16..24], [BLACK, colour_1, BLACK, BLACK, colour_1, colour_1, BLACK, BLACK]);
        assert_eq!(screen.colors[2][16..24], [BLACK, BLACK, BLACK, colour_2, colour_2, colour_2, BLACK, BLACK]);
    }
}