pub type Color = (u8, u8, u8);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile {
    pub pattern_index: u32,
    pub palette_index: u8,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub data: [[u8; 8]; 8],
}

use std::fmt;
use std::fmt::Debug;
impl Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            writeln!(f, "{:?}", row).expect("Failed to write");
        }
        Ok(())
    }
}

pub struct PixelBuffer<'a> {
    pub buffer: &'a mut [u8],
    pub pitch: usize,
    pub scale: u8,
}

impl<'a> PixelBuffer<'a> {
    pub fn set_pixel(&mut self, x: usize, y: usize, colour: (u8, u8, u8, u8)) {
        let scale = self.scale as usize;
        let mut offset = y * self.pitch * scale + x * 4 * scale;

        for _ in 0..scale {
            let mut i = 0;
            for _ in 0..scale {
                self.buffer[offset + i] = colour.3;
                self.buffer[offset + i + 1] = colour.2;
                self.buffer[offset + i + 2] = colour.1;
                self.buffer[offset + i + 3] = colour.0;
                i += 4;
            }
            offset += self.pitch;
        }
    }
}

pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub trait Screen {
    fn draw<T>(&mut self, func: T)
    where
        Self: Sized,
        T: FnOnce(&mut PixelBuffer);

    fn set_backdrop_color(&mut self, color: Color);

    fn upload_buffer(&mut self, rect: Option<Rectangle>, buffer: &[u8], pitch: usize);
    fn update_buffer<T>(&mut self, func: T)
    where
        T: FnOnce(&mut PixelBuffer);
    fn render(&mut self, src: Rectangle, dst_x: usize, dst_y: usize);

    fn render_sprite(
        &mut self,
        src: Rectangle,
        dst_x: usize,
        dst_y: usize,
        flip_horizontal: bool,
        flip_vertical: bool,
    );
    fn update_sprites<T>(&mut self, func: T)
    where
        T: FnOnce(&mut PixelBuffer);

    fn present(&mut self);
}

pub struct ScreenMock {
    pub temp_buffer: Box<[u8; 256 * 2 * 240 * 2 * 4]>,
    pub temp_sprite_buffer: Box<[u8; 64 * 8 * 4 * 8]>,
    pub screen_buffer: Box<[u8; 256 * 240 * 3]>,
}

impl ScreenMock {
    pub fn new() -> ScreenMock {
        ScreenMock {
            temp_buffer: box [0; 256 * 2 * 240 * 2 * 4],
            temp_sprite_buffer: box [0; 64 * 8 * 4 * 8],
            screen_buffer: box [0; 256 * 240 * 3],
        }
    }
}

impl Screen for ScreenMock {
    fn draw<T>(&mut self, _: T)
    where
        T: FnOnce(&mut PixelBuffer),
    {
    }

    fn set_backdrop_color(&mut self, colour: Color) {
        let mut index = 0;
        while index < self.screen_buffer.len() {
            self.screen_buffer[index + 0] = colour.2;
            self.screen_buffer[index + 1] = colour.1;
            self.screen_buffer[index + 2] = colour.0;
            index += 3;
        }
    }

    fn upload_buffer(&mut self, _: Option<Rectangle>, _: &[u8], _: usize) {
        unimplemented!()
    }

    fn update_buffer<T>(&mut self, func: T)
    where
        T: FnOnce(&mut PixelBuffer),
    {
        func(&mut PixelBuffer {
            buffer: self.temp_buffer.as_mut(),
            pitch: 256 * 2 * 4,
            scale: 1,
        });
    }

    fn render(&mut self, src: Rectangle, dst_x: usize, dst_y: usize) {
        let img_pitch = 256 * 2 * 4;
        let screen_pitch = 256 * 3;
        let mut y = dst_y;
        for row in src.y..src.y + (src.height as i32) {
            let mut x = dst_x;
            for col in src.x..src.x + (src.width as i32) {
                let row = row as usize;
                let col = col as usize;
                let screen_index = y * screen_pitch + x * 3;
                if self.temp_buffer[row * img_pitch + col * 4 + 3] == 255 {
                    self.screen_buffer[screen_index + 0] =
                        self.temp_buffer[row * img_pitch + col * 4 + 2];
                    self.screen_buffer[screen_index + 1] =
                        self.temp_buffer[row * img_pitch + col * 4 + 1];
                    self.screen_buffer[screen_index + 2] =
                        self.temp_buffer[row * img_pitch + col * 4 + 0];
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn render_sprite(&mut self, src: Rectangle, dst_x: usize, dst_y: usize, _: bool, _: bool) {
        let img_pitch = 64 * 8 * 4;
        let screen_pitch = 256 * 3;
        let mut y = dst_y;
        for row in src.y..src.y + (src.height as i32) {
            let mut x = dst_x;
            for col in src.x..src.x + (src.width as i32) {
                let row = row as usize;
                let col = col as usize;
                let screen_index = y * screen_pitch + x * 3;
                if screen_index + 2 < self.screen_buffer.len() {
                    if self.temp_sprite_buffer[row * img_pitch + col * 4 + 3] == 255 {
                        //Only add the pixel if alpha is 255.
                        self.screen_buffer[screen_index + 0] =
                            self.temp_sprite_buffer[row * img_pitch + col * 4 + 2];
                        self.screen_buffer[screen_index + 1] =
                            self.temp_sprite_buffer[row * img_pitch + col * 4 + 1];
                        self.screen_buffer[screen_index + 2] =
                            self.temp_sprite_buffer[row * img_pitch + col * 4 + 0];
                    }
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn update_sprites<T>(&mut self, func: T)
    where
        T: FnOnce(&mut PixelBuffer),
    {
        func(&mut PixelBuffer {
            buffer: self.temp_sprite_buffer.as_mut(),
            pitch: 64 * 8 * 4,
            scale: 1,
        });
    }

    fn present(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::PixelBuffer;

    #[test]
    fn pixel_buffer_with_scale_1() {
        let mut buffer = [0; 8 * 8 * 4];
        {
            let mut pixel_buffer = PixelBuffer {
                buffer: &mut buffer,
                pitch: 8 * 4,
                scale: 1,
            };
            for y in 0..8 {
                for x in 0..8 {
                    let colour = (x + y * 8) as u8;
                    pixel_buffer.set_pixel(x, y, (255, colour, colour, colour));
                }
            }
        }

        let expected: Vec<u8> = (0..64).flat_map(|i| vec![i, i, i, 255]).collect();
        let actual: Vec<u8> = buffer.iter().map(|&i| i).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pixel_buffer_with_scale_2() {
        let mut buffer = [0; 16 * 16 * 4];
        {
            let mut pixel_buffer = PixelBuffer {
                buffer: &mut buffer,
                pitch: 16 * 4,
                scale: 2,
            };
            for y in 0..8 {
                for x in 0..8 {
                    let colour = (x + y * 8) as u8;
                    pixel_buffer.set_pixel(x, y, (255, colour, colour, colour));
                }
            }
        }

        let expected: Vec<u8> = (0..8)
            .flat_map(|i| vec![i, i])
            .flat_map(|i: u8| -> Vec<u8> {
                (i * 8..i * 8 + 8)
                    .flat_map(|i| vec![i, i, i, 255, i, i, i, 255])
                    .collect()
            })
            .collect();
        let actual: Vec<u8> = buffer.iter().map(|&i| i).collect();
        assert_eq!(expected, actual);
    }
}

pub const BLACK: Color = (0, 0, 0);
pub const WHITE: Color = (1, 1, 1);

pub const COLOUR_PALETTE: [Color; 0x40] = [
    (0x75, 0x75, 0x75), //0x00
    (0x27, 0x1B, 0x8F), //0x01
    (0x00, 0x00, 0xAB), //0x02
    (0x47, 0x00, 0x9F), //0x03
    (0x8F, 0x00, 0x77), //0x04
    (0xAB, 0x00, 0x13), //0x05
    (0xA7, 0x00, 0x00), //0x06
    (0x7F, 0x0B, 0x00), //0x07
    (0x43, 0x2F, 0x00), //0x08
    (0x00, 0x47, 0x00), //0x09
    (0x00, 0x51, 0x00), //0x0A
    (0x00, 0x3F, 0x17), //0x0B
    (0x1B, 0x3F, 0x5F), //0x0C
    (0x00, 0x00, 0x00), //0x0D
    (0x00, 0x00, 0x00), //0x0E
    (0x00, 0x00, 0x00), //0x0F
    (0xBC, 0xBC, 0xBC), //0x10
    (0x00, 0x73, 0xEF), //0x11
    (0x23, 0x3B, 0xEF), //0x12
    (0x83, 0x00, 0xF3), //0x13
    (0xBF, 0x00, 0xBF), //0x14
    (0xE7, 0x00, 0x5B), //0x15
    (0xDB, 0x2B, 0x00), //0x16
    (0xCB, 0x4F, 0x0F), //0x17
    (0x8B, 0x73, 0x00), //0x18
    (0x00, 0x97, 0x00), //0x19
    (0x00, 0xAB, 0x00), //0x1A
    (0x00, 0x93, 0x3B), //0x1B
    (0x00, 0x83, 0x8B), //0x1C
    (0x00, 0x00, 0x00), //0x1D
    (0x00, 0x00, 0x00), //0x1E
    (0x00, 0x00, 0x00), //0x1F
    (0xFF, 0xFF, 0xFF), //0x20
    (0x3F, 0xBF, 0xFF), //0x21
    (0x5F, 0x97, 0xFF), //0x22
    (0xA7, 0x8B, 0xFD), //0x23
    (0xF7, 0x7B, 0xFF), //0x24
    (0xFF, 0x77, 0xB7), //0x25
    (0xFF, 0x77, 0x63), //0x26
    (0xFF, 0x9B, 0x3B), //0x27
    (0xF3, 0xBF, 0x3F), //0x28
    (0x83, 0xD3, 0x13), //0x29
    (0x4F, 0xDF, 0x4B), //0x2A
    (0x58, 0xF8, 0x98), //0x2B
    (0x00, 0xEB, 0xDB), //0x2C
    (0x00, 0x00, 0x00), //0x2D
    (0x00, 0x00, 0x00), //0x2E
    (0x00, 0x00, 0x00), //0x2F
    (0xFF, 0xFF, 0xFF), //0x30
    (0xAB, 0xE7, 0xFF), //0x31
    (0xC7, 0xD7, 0xFF), //0x32
    (0xD7, 0xCB, 0xFF), //0x33
    (0xFF, 0xC7, 0xFF), //0x34
    (0xFF, 0xC7, 0xDB), //0x35
    (0xFF, 0xBF, 0xB3), //0x36
    (0xFF, 0xDB, 0xAB), //0x37
    (0xFF, 0xE7, 0xA3), //0x38
    (0xE3, 0xFF, 0xA3), //0x39
    (0xAB, 0xF3, 0xBF), //0x3A
    (0xB3, 0xFF, 0xCF), //0x3B
    (0x9F, 0xFF, 0xF3), //0x3C
    (0x00, 0x00, 0x00), //0x3D
    (0x00, 0x00, 0x00), //0x3E
    (0x00, 0x00, 0x00), //0x3F
];
