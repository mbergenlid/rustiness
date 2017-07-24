

#[allow(dead_code)]
pub const BACK_DROP: (u8, u8, u8) = (0, 0, 0);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const ORANGE: (u8, u8, u8) = (0xCB,0x4F,0x0F);
#[allow(dead_code)]
pub const BROWN: (u8, u8, u8) = (0x00,0x3F,0x17);
#[allow(dead_code)]
pub const GREEN: (u8, u8, u8) = (0xB3,0xFF,0xCF);
#[allow(dead_code)]
pub const GRAY: (u8, u8, u8) = (0x75,0x75,0x75);

use std::ops::Range;
pub fn assert_pixels(expected: &[(u8, u8, u8)], screen_buffer: &[u8], range: Range<usize>) {
    let expected_buffer: Vec<u8> = expected.iter().flat_map(|p| vec!(p.0, p.1, p.2)).collect();
    assert_pixels_internal(&expected_buffer, &screen_buffer[range.start*3..range.end*3]);
}

fn assert_pixels_internal(expected: &[u8], actual: &[u8]) {
    assert_eq!(expected == actual, true, "Expected\n{}\nbut was\n{}\n", expected.debug(), actual.debug());
}

use std::fmt::format;
trait PixelDebug {
    fn debug(&self) -> String;
}
impl <'a> PixelDebug for &'a [u8] {
    fn debug(&self) -> String {
        let mut i = 0;
        let mut string = String::new();
        while i < self.len() {
            string = string + &format(format_args!("({},{},{})", self[i], self[i+1], self[i+2]));
            i += 3;
            if i % 32 == 0 {
                string = string + "\n";
            }
        }
        return string;
    }
}
