use memory::Memory;

const COLOUR_PALETTE_BASE_ADDRESS: u16 = 0x3F00;
pub struct AttributeTable<'a> {
    pub memory: &'a Memory,
    pub address: u16,
}

impl<'a> AttributeTable<'a> {
    pub fn get_palette_address(&self, tile_row: u16, tile_col: u16) -> u16 {
        let palette_index = self.get_palette_index(tile_row, tile_col);

        COLOUR_PALETTE_BASE_ADDRESS + (palette_index as u16) * 4
    }

    pub fn get_palette_index(&self, tile_row: u16, tile_col: u16) -> u8 {
        let attribute_row = tile_row >> 2;
        let attribute_col = tile_col >> 2;
        let row_inside_attribute = (tile_row & 0x03) >> 1;
        let col_inside_attribute = (tile_col & 0x03) >> 1;
        let quadrant = (row_inside_attribute << 1) | col_inside_attribute;
        let attribute_address = self.address + (attribute_row * 8 + attribute_col);

        let value = self.memory.get(attribute_address, 0);
        value >> (quadrant << 1) & 0x03
    }
}

#[cfg(test)]
mod test {
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
            address: 0x23C0,
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
