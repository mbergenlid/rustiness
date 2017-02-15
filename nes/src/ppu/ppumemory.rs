
use memory::{Memory, BasicMemory, Address};

pub enum Mirroring {
    Horizontal,
    Vertical,
    NoMirroring,
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
                Mirroring::NoMirroring => 0xFFFF,
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
        } else if address == 0x3F10 || address == 0x3F14 || address == 0x3F18 || address == 0x3F1C {
            self.get(0x3F00)
        } else if address >= 0x3F20 && address < 0x4000 {
            self.get(address & 0xFF1F)
        } else {
            self.basic_memory.get(address)
        }
    }
    fn set(&mut self, address: Address, value: u8) {
        if address >= 0x2000 && address < 0x3000 {
            self.basic_memory.set(address & self.name_table_mirror_mask, value);
        } else if address >= 0x3000 && address < 0x3F00 {
            self.set(address & 0xEFFF, value);
        } else if address == 0x3F10 || address == 0x3F14 || address == 0x3F18 || address == 0x3F1C {
            self.basic_memory.set(0x3F00, value);
        } else if address >= 0x3F20 && address < 0x4000 {
            self.set(address & 0xFF1F, value);
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
    fn test_no_mirroring() {
        let mut ppu_mem = PPUMemory::new(Mirroring::NoMirroring);
        for address in 0x2000..0x2400 {
            let value = rand::random::<u8>() & (!0x80);
            ppu_mem.set(address, value);
            assert_eq!(value, ppu_mem.get(address), "Original address is not written properly");
            assert_eq!(0, ppu_mem.get(address + 0x400), "Address {} changed unexpectedly", address + 0x400);
            assert_eq!(0, ppu_mem.get(address + 0x800), "Address {} changed unexpectedly", address + 0x800);
            assert_eq!(0, ppu_mem.get(address + 0xC00), "Address {} changed unexpectedly", address + 0xC00);
        }
    }

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
        let mut ppu_mem = PPUMemory::new(Mirroring::Horizontal);

        //$3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
        assert_mirrored_addresses(&mut ppu_mem, 0x3F10, 0x3F00);
        assert_mirrored_addresses(&mut ppu_mem, 0x3F14, 0x3F00);
        assert_mirrored_addresses(&mut ppu_mem, 0x3F18, 0x3F00);
        assert_mirrored_addresses(&mut ppu_mem, 0x3F1C, 0x3F00);

        for address in 0x3F00..0x3F20 {
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*1);
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*2);
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*3);
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*4);
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*5);
            assert_mirrored_addresses(&mut ppu_mem, address, address+0x20*6);
        }
    }

    use memory::Address;
    fn assert_mirrored_addresses(ppu_mem: &mut PPUMemory, address1: Address, address2: Address) {
        let value1 = rand::random::<u8>();
        ppu_mem.set(address1, value1);
        assert_eq!(value1, ppu_mem.get(address1), "value is not written to original address 0x{:x}", address1);
        assert_eq!(value1, ppu_mem.get(address2), "value is not mirrored from 0x{:x} to 0x{:x}", address1, address2);

        let value2 = rand::random::<u8>();
        ppu_mem.set(address2, value2);
        assert_eq!(value2, ppu_mem.get(address2), "value is not written to original address 0x{:x}", address2);
        assert_eq!(value2, ppu_mem.get(address1), "value is not mirrored from 0x{:x} to 0x{:x}", address2, address1);

    }
}
