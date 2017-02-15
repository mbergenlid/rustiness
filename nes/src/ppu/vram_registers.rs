
pub struct VRAMRegisters {
    pub temporary: u16,
    pub current: u16,
    fine_x: u8,
    write_toggle: bool,
}

impl VRAMRegisters {
    pub fn new() -> VRAMRegisters {
        VRAMRegisters {
            temporary: 0,
            current: 0,
            fine_x: 0,
            write_toggle: false,
        }
    }

    pub fn write_scroll(&mut self, x: u8) {
        if self.write_toggle {
            let high_bits = (x as u16 & 0xF8) << 2;
            let low_bits = (x as u16 & 0x07) << 12;
            self.temporary = (self.temporary & 0b000_1100_0001_1111) | high_bits | low_bits;
            self.write_toggle = false;
        } else {
            let high_bits = ((x & 0xF8) >> 3) as u16;
            self.fine_x = x & 0x7;
            self.temporary = (self.temporary & 0b111_1111_1110_0000) | high_bits;
            self.write_toggle = true;
        }
    }

    pub fn horizontal_increment(&mut self) {
        if self.current & 0x1F == 0x1F {
            let name_table = (self.current & 0x04_00) ^ 0x0400;
            self.current = (self.current & 0b111_1011_1110_0000) | name_table;
        } else {
            self.current += 1;
        }
    }

    pub fn vertical_increment(&mut self) {
        if self.current & 0x7000 != 0x7000 {
            self.current += 0x1000;
        } else {
            self.current = self.current & !0x7000; //reset fine y
            if self.current & 0x3E0 == (29 << 5) {
                self.current = self.current & !0x3E0;
                self.current = self.current ^ 0x0800;
            } else if self.current & 0x3E0 == 0x3E0 {
                self.current = self.current & !0x3E0;
            } else {
                self.current += 0x0020;
            }
        }
    }

    pub fn copy_horizontal_bits(&mut self) {
        let horizontal_bits = self.temporary & 0b000_0100_0001_1111;
        self.current = (self.current & 0b111_1011_1110_0000) | horizontal_bits;
    }

    pub fn write_name_table(&mut self, value: u8) {
        let name_table = (value & 0x03) as u16;
        self.temporary = self.temporary & 0xF3FF | (name_table << 10);
    }

    pub fn set_vram(&mut self, value: u8) {
        if self.write_toggle {
            self.temporary = (self.temporary & 0xFF00) | (value as u16);
            self.current = self.temporary;
            self.write_toggle = false;
        } else {
            self.temporary = (self.temporary & 0x00FF) | (((value as u16) & 0x3F) << 8);
            self.write_toggle = true;
        }
    }

    pub fn reset_write_toggle(&mut self) {
        self.write_toggle = false;
    }

    pub fn copy_temporary_bits(&mut self) {
        self.current = self.temporary;
    }

    pub fn fine_y(&self) -> u8 {
        (self.current >> 12) as u8
    }

    pub fn temporary_x_scroll(&self) -> u8 {
        (self.temporary & 0x1F) as u8
    }

    pub fn temporary_y_scroll(&self) -> u8 {
        ((self.temporary & 0x7000) >> 12) as u8 | ((self.temporary & 0x3E0) >> 2) as u8
    }

    pub fn current_x_scroll(&self) -> u8 {
        (((self.current & 0x1F) as u8) << 3) | self.fine_x
    }

    pub fn current_y_scroll(&self) -> u8 {
        ((self.current & 0x7000) >> 12) as u8 | ((self.current & 0x3E0) >> 2) as u8
    }

    pub fn current_absolute_x_scroll(&self) -> u16 {
        ((self.current & 0x400) >> 2) + self.current_x_scroll() as u16
    }
}

#[cfg(test)]
mod tests {

    impl VRAMRegisters {
        fn with_current(value: u16) -> VRAMRegisters {
            VRAMRegisters {
                temporary: 0,
                current: value,
                fine_x: 0,
                write_toggle: false,
            }
        }

        fn with_temp(value: u16) -> VRAMRegisters {
            VRAMRegisters {
                temporary: value,
                current: 0,
                fine_x: 0,
                write_toggle: false,
            }
        }
    }

    use super::VRAMRegisters;
    #[test]
    fn write_scroll() {
        let mut registers = VRAMRegisters::new();
        registers.write_scroll(8);
        assert_eq!(0b000_0000_0000_0001, registers.temporary);

        registers.write_scroll(8);
        assert_eq!(0b000_0000_0010_0001, registers.temporary);

        registers.write_scroll(16);
        assert_eq!(0b000_0000_0010_0010, registers.temporary);

        registers.write_scroll(16);
        assert_eq!(0b000_0000_0100_0010, registers.temporary);
    }

    #[test]
    fn write_fine_y_scroll() {
        let mut registers = VRAMRegisters::new();
        registers.write_scroll(0);
        assert_eq!(0b000_0000_0000_0000, registers.temporary);

        registers.write_scroll(9);
        assert_eq!(0b001_0000_0010_0000, registers.temporary);
    }

    #[test]
    fn horizontal_increment_top_name_tables() {
        let mut registers = VRAMRegisters::new();

        {
            let mut x_scroll = 0;
            for _ in 1..32 {
                registers.horizontal_increment();
                x_scroll += 1;
                assert_eq!(x_scroll, registers.current);
            }
        }
        registers.horizontal_increment(); //This should carry over to name-table
        assert_eq!(0b000_0100_0000_0000, registers.current);

        {
            let mut x_scroll = 0b000_0100_0000_0000;
            for _ in 1..32 {
                registers.horizontal_increment();
                x_scroll += 1;
                assert_eq!(x_scroll, registers.current);
            }
        }
        registers.horizontal_increment(); //This should wrap around the name-table
        assert_eq!(0b000_0000_0000_0000, registers.current);
    }

    #[test]
    fn horizontal_increment_bottom_name_tables() {
        let mut registers = VRAMRegisters::with_current(0b000_1000_0000_0000);

        {
            let mut x_scroll = 0b000_1000_0000_0000;
            for _ in 1..32 {
                registers.horizontal_increment();
                x_scroll += 1;
                assert_eq!(x_scroll, registers.current);
            }
        }
        registers.horizontal_increment(); //This should carry over to name-table
        assert_eq!(0b000_1100_0000_0000, registers.current);

        {
            let mut x_scroll = 0b000_1100_0000_0000;
            for _ in 1..32 {
                registers.horizontal_increment();
                x_scroll += 1;
                assert_eq!(x_scroll, registers.current);
            }
        }
        registers.horizontal_increment(); //This should wrap around the name-table
        assert_eq!(0b000_1000_0000_0000, registers.current);
    }

    #[test]
    fn vertical_increment_let_name_tables() {
        let mut registers = VRAMRegisters::with_current(0b000_0000_0000_0000);
        {
            let mut coarse_y_scroll = 0;
            for _ in 1..30 {
                let mut fine_y_scroll = coarse_y_scroll;
                for fine in 1..8 {
                    registers.vertical_increment();
                    fine_y_scroll += 0b001_0000_0000_0000;
                    assert_eq!(
                        fine_y_scroll,
                        registers.current,
                        "Fine increment: {}, Expected: {:015b}, Was: {:015b}", fine, fine_y_scroll, registers.current
                    );
                }
                registers.vertical_increment();
                coarse_y_scroll += 0b000_000_0010_0000;
                assert_eq!(coarse_y_scroll, registers.current);
            }
            let mut fine_y_scroll = coarse_y_scroll;
            for fine in 1..8 {
                registers.vertical_increment();
                fine_y_scroll += 0b001_0000_0000_0000;
                assert_eq!(
                    fine_y_scroll,
                    registers.current,
                    "Fine increment: {}, Expected: {:015b}, Was: {:015b}", fine, fine_y_scroll, registers.current
                );
            }
            registers.vertical_increment();
            coarse_y_scroll = 0b000_1000_0000_0000;
            assert_eq!(
                coarse_y_scroll,
                registers.current,
                "Expected: {:015b}, Was: {:015b}", coarse_y_scroll, registers.current
            );
        }

        {
            let mut coarse_y_scroll = 0b000_1000_0000_0000;
            for _ in 1..30 {
                let mut fine_y_scroll = coarse_y_scroll;
                for fine in 1..8 {
                    registers.vertical_increment();
                    fine_y_scroll += 0b001_0000_0000_0000;
                    assert_eq!(
                        fine_y_scroll,
                        registers.current,
                        "Fine increment: {}, Expected: {:015b}, Was: {:015b}", fine, fine_y_scroll, registers.current
                    );
                }
                registers.vertical_increment();
                coarse_y_scroll += 0b000_000_0010_0000;
                assert_eq!(coarse_y_scroll, registers.current);
            }
            let mut fine_y_scroll = coarse_y_scroll;
            for fine in 1..8 {
                registers.vertical_increment();
                fine_y_scroll += 0b001_0000_0000_0000;
                assert_eq!(
                    fine_y_scroll,
                    registers.current,
                    "Fine increment: {}, Expected: {:015b}, Was: {:015b}", fine, fine_y_scroll, registers.current
                );
            }
            registers.vertical_increment();
            coarse_y_scroll = 0;
            assert_eq!(
                coarse_y_scroll,
                registers.current,
                "Expected: {:015b}, Was: {:015b}", coarse_y_scroll, registers.current
            );
        }
    }

    extern crate rand;
    #[test]
    fn copy_horizontal_bits() {
        for _ in 0..100 {
            let temp_vram = rand::random::<u16>();
            let mut vram = VRAMRegisters::with_temp(temp_vram);

            vram.copy_horizontal_bits();
            assert_eq!(
                temp_vram & 0b000_0100_0001_1111,
                vram.current,
                "Copy temp value: {:04x} -> {:04x}", temp_vram, vram.current
            )
        }
    }

    #[test]
    fn write_name_table() {
        for _ in 0..100 {
            let mut vram = VRAMRegisters::new();

            let value = rand::random::<u8>() & 0xFC;
            vram.write_name_table(value | 0x00);
            assert_eq!(0x0000, vram.temporary);

            vram.write_name_table(value | 0x01);
            assert_eq!(0x0400, vram.temporary);

            vram.write_name_table(value | 0x02);
            assert_eq!(0x0800, vram.temporary);

            vram.write_name_table(value | 0x03);
            assert_eq!(0x0C00, vram.temporary);
        }
    }

    #[test]
    fn set_vram() {
        let mut vram = VRAMRegisters::new();

        vram.set_vram(0x05);
        assert_eq!(0x0500, vram.temporary);
        assert_eq!(0x0, vram.current);

        vram.set_vram(0x1A);
        assert_eq!(0x051A, vram.temporary);
        assert_eq!(0x051A, vram.current);
    }
}
