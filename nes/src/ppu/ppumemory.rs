
use memory::{Memory, BasicMemory, Address};

pub enum Mirroring {
    Horizontal,
    Vertical,
}

pub struct PPUMemory {
    basic_memory: BasicMemory,
    name_table_mirror_mask: u16,
}

impl PPUMemory {
    pub fn new(mirroring: Mirroring) -> PPUMemory {
        PPUMemory {
            basic_memory: BasicMemory::new(),
            name_table_mirror_mask: match mirroring {
                Mirroring::Horizontal => 0xFBFF,
                Mirroring::Vertical => !0x0800,
            }

        }
    }
}

impl Memory for PPUMemory {
    fn get(&self, address: Address) -> u8 {
        if address >= 0x2000 && address < 0x3000 {
            self.basic_memory.get(address & self.name_table_mirror_mask)
        } else if address >= 0x3000 && address < 0x3F00 {
            self.get(address & 0xEFFF)
        } else {
            self.basic_memory.get(address)
        }
    }
    fn set(&mut self, address: Address, value: u8) {
        if address >= 0x2000 && address < 0x3000 {
            self.basic_memory.set(address & self.name_table_mirror_mask, value);
        } else if address >= 0x3000 && address < 0x3F00 {
            self.set(address & 0xEFFF, value);
        } else {
            self.basic_memory.set(address, value);
        }
    }
}

#[cfg(test)]
pub mod tests {
    extern crate rand;
    use memory::Memory;
    use super::{PPUMemory, Mirroring};

    #[test]
    fn test_horizontal_read_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::Horizontal);

        for address in 0x2000..0x2400 {
            let value = rand::random::<u8>() & (!0x80);
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address + 0x400), "Mirrored address is not written properly");
        }

        for address in 0x2800..0x2C00 {
            let value = rand::random::<u8>() | 0x80;
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address + 0x400), "Mirrored address is not written properly");
        }

        //First name table is not same as third name table
        for address in 0x2000..0x2400 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x800));
        }

        //Second name table is not same as fourth name table
        for address in 0x2400..0x2800 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x800));
        }
    }

    #[test]
    fn test_horizontal_write_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::Horizontal);

        //Fill second (i.e. first) name table
        for address in 0x2400..0x2800 {
            let value = rand::random::<u8>() & (!0x80);
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address - 0x400), "Mirrored address is not written properly");
        }

        //Fill fourth (i.e. third) name table
        for address in 0x2C00..0x3000 {
            let value = rand::random::<u8>() | 0x80;
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly for address {:02x}", address);
            assert_eq!(value, ppu_mem.get(address - 0x400), "Mirrored address is not written properly for address {:02x}", address);
        }

        //Third name table is not the same as first name table
        for address in 0x2800..0x2C00 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address - 0x800));
        }

        //Fourth name table is not the same as second name table
        for address in 0x2C00..0x3000 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address - 0x800));
        }
    }

    #[test]
    fn test_vertical_read_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::Vertical);

        //Fill first (and third) name table
        for address in 0x2000..0x2400 {
            let value = rand::random::<u8>() & (!0x80);
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address + 0x800), "Mirrored address is not written properly");
        }

        //Fill second (and fourth) name table
        for address in 0x2400..0x2800 {
            let value = rand::random::<u8>() | 0x80;
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address + 0x800), "Mirrored address is not written properly");
        }

        //First name table is not the same as second name table
        for address in 0x2000..0x2400 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x400), "Value {:02X} = {:02X}", address, address+0x400);
        }

        //Third name table is not the same as fourth name table
        for address in 0x2800..0x2C00 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x400), "Value {:02X} = {:02X}", address, address+0x400);
        }
    }

    #[test]
    fn test_vertical_write_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::Vertical);

        //Fill third (and first) name table
        for address in 0x2800..0x2C00 {
            let value = rand::random::<u8>() & (!0x80);
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address - 0x800), "Mirrored address is not written properly");
        }

        //Fill fourth (and second) name table
        for address in 0x2C00..0x3000 {
            let value = rand::random::<u8>() | 0x80;
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(value, ppu_mem.get(address - 0x800), "Mirrored address is not written properly");
        }

        //First name table is not the same as second name table
        for address in 0x2000..0x2400 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x400), "Value {:02X} = {:02X}", address, address+0x400);
        }

        //Third name table is not the same as fourth name table
        for address in 0x2800..0x2C00 {
            assert_ne!(ppu_mem.get(address), ppu_mem.get(address + 0x400), "Value {:02X} = {:02X}", address, address+0x400);
        }
    }

    #[test]
    fn test_entire_name_table_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::Horizontal);

        //0b0011
        //0x2197 -> 0x3197
        for _ in 0..100 {
            //Random value in range [0x2000,0x2EFF]
            let address = (rand::random::<u16>() % 0x0F00) + 0x2000;
            let value = rand::random::<u8>();
            ppu_mem.set(address, value);

            assert_eq!(value, ppu_mem.get(address), "Original address {:02x} is not written properly", address);
            assert_eq!(value, ppu_mem.get(address + 0x1000), "Mirrored address {:02x} is not written properly", address)
        }

        for address in 0x0000..0x2000 {
            assert_eq!(0, ppu_mem.get(address));
        }
        for address in 0x3F00..0x3FFF {
            assert_eq!(0, ppu_mem.get(address));
        }
    }

    #[test]
    fn test_palette_mirroring() {

    }
}