
pub trait Sprite {
    fn position_y(&self) -> u8;
    fn position_x(&self) -> u8;
    fn pattern_index(&self) -> u8;

    fn colour_palette(&self) -> u16;
    fn flip_horizontal(&self) -> bool;
    fn flip_vertical(&self) -> bool;
    fn is_back(&self) -> bool;
    fn is_front(&self) -> bool {
        !self.is_back()
    }
}

impl <'a> Sprite for &'a [u8] {
    fn position_y(&self) -> u8 { return self[0] + 1; }
    fn position_x(&self) -> u8 { return self[3]; }
    fn pattern_index(&self) -> u8 { return self[1]; }

    fn colour_palette(&self) -> u16 { return (self[2] as u16 & 0x3)*4 + 0x3F10; }
    fn flip_horizontal(&self) -> bool { return self[2] & 0x40 > 0; }
    fn flip_vertical(&self) -> bool { return self[2] & 0x80 > 0; }
    fn is_back(&self) -> bool { return self[2] & 0x20 != 0; }
}
