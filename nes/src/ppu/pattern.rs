
pub type Pixel = u8;

#[derive(Clone, Copy)]
pub struct Pattern {
    data: [[Pixel; 8]; 8],
    raw_data: [u8; 16],
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {
            data: [[0; 8]; 8],
            raw_data: [0; 16],
        }
    }

    pub fn pixel(&self, x: u8, y: u8) -> Pixel {
        self.data[y as usize][x as usize]
    }

}

use memory::{Memory, Address};
impl Memory for Pattern {
    fn get(&self, address: Address) -> u8 {
        return self.raw_data[(address as usize) & 0xF];
    }
    fn set(&mut self, address: Address, value: u8) {
        let address: usize = address as usize;
        self.raw_data[address & 0xF] = value;
        let row = address & 0x7;
        let mut value = value;
        let shift = (address & 0xF) >> 3;
        for bit_index in 0..8 {
            self.data[row][7-bit_index] |= (value & 0x1) << shift;
            value >>= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Pattern;
    use memory::Memory;

    #[test]
    fn setting_low_bits() {
        let mut pattern = Pattern::new();
        pattern.set(0x0, 0b01010101);

        assert_eq!(0, pattern.pixel(0,0));
        assert_eq!(1, pattern.pixel(1,0));
        assert_eq!(0, pattern.pixel(2,0));
        assert_eq!(1, pattern.pixel(3,0));
        assert_eq!(0, pattern.pixel(4,0));
        assert_eq!(1, pattern.pixel(5,0));
        assert_eq!(0, pattern.pixel(6,0));
        assert_eq!(1, pattern.pixel(7,0));

        assert_eq!(0b01010101, pattern.get(0x0));
    }

    #[test]
    fn setting_high_bits() {
        let mut pattern = Pattern::new();
        pattern.set(0x8, 0b01010101);

        assert_eq!(0, pattern.pixel(0,0));
        assert_eq!(2, pattern.pixel(1,0));
        assert_eq!(0, pattern.pixel(2,0));
        assert_eq!(2, pattern.pixel(3,0));
        assert_eq!(0, pattern.pixel(4,0));
        assert_eq!(2, pattern.pixel(5,0));
        assert_eq!(0, pattern.pixel(6,0));
        assert_eq!(2, pattern.pixel(7,0));

        assert_eq!(0b01010101, pattern.get(0x8));
    }

    #[test]
    fn setting_both_high_and_low_bits() {
        let mut pattern = Pattern::new();
        pattern.set(0x0, 0b01010101);
        pattern.set(0x8, 0b00110011);

        assert_eq!(0, pattern.pixel(0,0));
        assert_eq!(1, pattern.pixel(1,0));
        assert_eq!(2, pattern.pixel(2,0));
        assert_eq!(3, pattern.pixel(3,0));
        assert_eq!(0, pattern.pixel(4,0));
        assert_eq!(1, pattern.pixel(5,0));
        assert_eq!(2, pattern.pixel(6,0));
        assert_eq!(3, pattern.pixel(7,0));

        assert_eq!(0b01010101, pattern.get(0x0));
        assert_eq!(0b00110011, pattern.get(0x8));
    }

    #[test]
    fn getting_uninitialized_memory() {
        let pattern = Pattern::new();
        assert_eq!(0, pattern.get(0x40));
    }
}
