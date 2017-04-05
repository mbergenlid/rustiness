
pub struct TileCache {
    tiles_changed: [[bool; 30*2]; 32*2],
}

use std::cmp::min;
impl TileCache {
    pub fn new() -> TileCache {
        TileCache {
            tiles_changed: [[true; 30*2]; 32*2]
        }
    }

    pub fn update(&mut self, address: u16) {
        let row_offset: usize = if address & 0x800 == 0 { 0 } else { 30 };
        let col_offset: usize = if address & 0x400 == 0 { 0 } else { 32 };
        if address & 0x3C0 == 0x3C0 {
            //Attribute table
            let row = (address as usize >> 3) & 0x7;
            let col = (address as usize & 0x7) + col_offset/4;
            for tc in col*4..(col*4 + 4) {
                for rc in (row_offset + row*4)..(min(row_offset + row*4 + 4, row_offset + 30)) {
                    self.tiles_changed[tc][rc] = true;
                }
            }
        } else {
            let row = ((address as usize >> 5) & 0x1F) + row_offset;
            let col = (address as usize & 0x1F) + col_offset;
            self.tiles_changed[col][row] = true;
        }
    }

    #[inline]
    pub fn is_modified(&self, row: usize, col: usize) -> bool {
        self.tiles_changed[col][row]
    }

    #[inline]
    pub fn clear(&mut self, row: usize, col: usize) {
        self.tiles_changed[col][row] = false;
    }
}

#[cfg(test)]
mod test {
    fn assert(address: u16, tiles: &[(u8, u8)]) {
        let mut tile_cache = super::TileCache::new();
        for col in 0..64 {
            for row in 0..60 {
                tile_cache.clear(row, col);
            }
        }
        tile_cache.update(address);
        for col in 0..64 {
            for row in 0..60 {
                if tiles.iter().find(|&&(r,c)| r == row && c == col).is_some() {
                    assert_eq!(tile_cache.tiles_changed[col as usize][row as usize], true, "Expected ({},{}) to be true", row, col);
                } else {
                    assert_eq!(tile_cache.tiles_changed[col as usize][row as usize], false, "Expected ({},{}) to be false", row, col);
                }

            }
        }
    }

    #[test]
    fn name_table_1() {
        assert(0x2001, &[(0,1)]);
        assert(0x2020, &[(1,0)]);
        assert(0x23BF, &[(29,31)]);
    }

    #[test]
    fn name_table_1_attribute_table() {
        assert(0x23C0, &[(0,0),(0,1),(0,2),(0,3),(1,0),(1,1),(1,2),(1,3),(2,0),(2,1),(2,2),(2,3),(3,0),(3,1),(3,2),(3,3)]);
        assert(0x23F8, &[(28,0),(28,1),(28,2),(28,3),(29,0),(29,1),(29,2),(29,3)]);
    }

    #[test]
    fn name_table_2() {
        assert(0x2401, &[(0,33)]);
        assert(0x2420, &[(1,32)]);
        assert(0x27BF, &[(29,63)]);
    }

    #[test]
    fn name_table_3() {
        assert(0x2801, &[(30,1)]);
        assert(0x2820, &[(31,0)]);
        assert(0x2BBF, &[(59,31)]);
    }

    #[test]
    fn name_table_3_attribute_table() {
        assert(0x2BC0, &[(30,0),(30,1),(30,2),(30,3),(31,0),(31,1),(31,2),(31,3),(32,0),(32,1),(32,2),(32,3),(33,0),(33,1),(33,2),(33,3)]);
       //assert(0x27F8, &[(28,32),(28,33),(28,34),(28,35),(29,32),(29,33),(29,34),(29,35)]);
    }

    #[test]
    fn name_table_2_attribute_table() {
        assert(0x27C0, &[(0,32),(0,33),(0,34),(0,35),(1,32),(1,33),(1,34),(1,35),(2,32),(2,33),(2,34),(2,35),(3,32),(3,33),(3,34),(3,35)]);
        assert(0x27F8, &[(28,32),(28,33),(28,34),(28,35),(29,32),(29,33),(29,34),(29,35)]);
    }

    #[test]
    fn name_table_4_attribute_table() {
        assert(0x2FC0, &[(30,32),(30,33),(30,34),(30,35),(31,32),(31,33),(31,34),(31,35),(32,32),(32,33),(32,34),(32,35),(33,32),(33,33),(33,34),(33,35)]);
        assert(0x2FF8, &[(58,32),(58,33),(58,34),(58,35),(59,32),(59,33),(59,34),(59,35)]);
        assert(0x2FFF, &[(58,60),(58,61),(58,62),(58,63),(59,60),(59,61),(59,62),(59,63)]);
    }

}
