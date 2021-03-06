use memory::Memory;
use ppu::ppumemory::{Mirroring, PPUMemory};
use std::fs::File;
use std::io::Read;

pub type ROM = [u8];

pub struct INes {
    buffer: Vec<u8>,
    pub num_prg_roms: u8,
    pub num_chr_roms: u8,
    pub mirroring: Mirroring,
}

impl<'a> INes {
    pub fn from_file(mut file: File) -> INes {
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let num_prg_roms = buffer[4];
        let num_chr_roms = buffer[5];
        let mirroring = if buffer[6] & 0x01 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };
        INes {
            buffer: buffer,
            num_prg_roms: num_prg_roms,
            num_chr_roms: num_chr_roms,
            mirroring: mirroring,
        }
    }

    pub fn read(file: &mut dyn Read) -> INes {
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let num_prg_roms = buffer[4];
        let num_chr_roms = buffer[5];
        let mirroring = if buffer[6] & 0x01 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };
        INes {
            buffer: buffer,
            num_prg_roms: num_prg_roms,
            num_chr_roms: num_chr_roms,
            mirroring: mirroring,
        }
    }

    pub fn prg_rom(&self, index: usize) -> &ROM {
        let rom_base: usize = 0x10 + index * 0x4000;
        &self.buffer[rom_base..(rom_base + 0x4000)]
    }

    pub fn chr_rom(&self, index: usize) -> &ROM {
        let chr_base = 0x10 + (self.num_prg_roms as usize) * 0x4000;
        let rom_base: usize = chr_base + index * 0x2000;
        &self.buffer[rom_base..(rom_base + 0x2000)]
    }

    pub fn load(&self, cpu_memory: &mut dyn Memory) {
        cpu_memory.set_slice(0x8000, self.prg_rom(0));
        if self.num_prg_roms == 1 {
            cpu_memory.set_slice(0xC000, self.prg_rom(0));
        } else if self.num_prg_roms == 2 {
            cpu_memory.set_slice(0xC000, self.prg_rom(1));
        } else {
            panic!(".nes file contains more than 2 prg rom banks which is not allowed");
        }
    }

    pub fn ppu_memory(&self) -> PPUMemory {
        let mut ppu_mem = PPUMemory::new(self.mirroring);
        ppu_mem.set_slice(0x0000, self.chr_rom(0));
        return ppu_mem;
    }
}

#[cfg(test)]
mod test {

    use memory;
    use memory::Memory;
    use std::fs::File;

    #[test]
    fn test() {
        let file = File::open("src/ines/donkey_kong.nes").unwrap();
        let ines = super::INes::from_file(file);

        assert_eq!(1, ines.num_prg_roms);
        assert_eq!(1, ines.num_chr_roms);

        assert_eq!(ines.buffer[0x10..0x4010], *(ines.prg_rom(0)));

        let mut memory = memory::BasicMemory::new();
        ines.load(&mut memory);
        assert_eq!(ines.buffer[0x10], memory.get(0x8000, 0));

        //should mirror 0xC0000 - 0xFFFF onto 0x8000-0xBFFF
        for i in 0x8000..0xC000 {
            assert_eq!(memory.get(i, 0), memory.get(i + 0x4000, 0));
        }
    }
}
