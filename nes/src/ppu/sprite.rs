
pub trait Sprite {
    fn position_y(&self) -> u8;
    fn position_x(&self) -> u8;
    fn pattern_index(&self) -> u8;

    fn colour_palette(&self) -> u8;
    fn flip_horizontal(&self) -> bool;
    fn flip_vertical(&self) -> bool;
    fn is_back(&self) -> bool;
    fn is_front(&self) -> bool {
        !self.is_back()
    }
}

impl <'a> Sprite for &'a [u8] {
    fn position_y(&self) -> u8 { return self[0]; }
    fn position_x(&self) -> u8 { return self[3]; }
    fn pattern_index(&self) -> u8 { return self[1]; }

    fn colour_palette(&self) -> u8 { return self[2] & 0x3; }
    fn flip_horizontal(&self) -> bool { return self[2] & 0x40 > 0; }
    fn flip_vertical(&self) -> bool { return self[2] & 0x80 > 0; }
    fn is_back(&self) -> bool { return self[2] & 0x20 != 0; }
}

use ppu::screen::{Screen, Rectangle};
pub struct Sprites {
    data: [u8; 64*4],
    address: u8,
}

impl Sprites {
    pub fn new() -> Sprites {
        Sprites{
            data: [0; 64*4],
            address: 0
        }
    }

    pub fn set_address(&mut self, value: u8) {
        self.address = value;
    }
    pub fn address(&self) -> u8 {
        self.address
    }

    pub fn write_byte(&mut self, value: u8) {
        let address = self.address;
        self.address = self.address.wrapping_add(1);
        self.data[address as usize] = value;
    }

    pub fn read_byte(&self) -> u8 {
        self.data[self.address as usize]
    }

    pub fn render_back_sprites<T>(&self, screen: &mut T) where T: Screen + Sized {
        for sprite_index in (0..64).rev() {
            let sprite = &self.data[(sprite_index*4)..(sprite_index*4+4)];
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
    pub fn render_front_sprites<T>(&self, screen: &mut T) where T: Screen + Sized {
        for sprite_index in (0..64).rev() {
            let sprite = &self.data[(sprite_index*4)..(sprite_index*4+4)];
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

    pub fn slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

use std::ops::Index;
impl Index<usize> for Sprites {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[(index*4)..(index*4+4)]
    }
}
