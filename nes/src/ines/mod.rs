
use std::fs::File;
use std::io::Read;
use memory::Memory;

pub type ROM = [u8];

pub struct INes {
    buffer: Vec<u8>,
    pub num_prg_roms: u8,
    pub num_chr_roms: u8,
}

impl <'a> INes  {
    pub fn from_file(mut file: File) -> INes {
        let mut buffer: Vec<u8> = vec!();
        file.read_to_end(&mut buffer).unwrap();
        let num_prg_roms = buffer[4];
        let num_chr_roms = buffer[5];
        INes {
            buffer: buffer,
            num_prg_roms: num_prg_roms,
            num_chr_roms: num_chr_roms,
        }
    }

    pub fn prg_rom(&self, index: usize) -> &ROM {
        let rom_base: usize = 0x10 + index*0x400;
        &self.buffer[rom_base..(rom_base+0x4000)]
    }

    pub fn load(&self, memory: &mut Memory) {
        memory.set_slice(0x8000, self.prg_rom(0));
        memory.set_slice(0xC000, self.prg_rom(0));
    }
}


#[cfg(test)]
mod test {

    use std::fs::File;
    use memory;
    use memory::Memory;

    #[test]
    fn test() {
        let file = File::open("src/ines/donkey_kong.nes").unwrap();
        let ines = super::INes::from_file(file);

        assert_eq!(1, ines.num_prg_roms);
        assert_eq!(1, ines.num_chr_roms);

        assert_eq!(ines.buffer[0x10..0x4010], *(ines.prg_rom(0)));

        let mut memory = memory::BasicMemory::new();
        ines.load(&mut memory);
        assert_eq!(ines.buffer[0x10], memory.get(0x8000));

        //should mirror 0xC0000 - 0xFFFF onto 0x8000-0xBFFF
        for i in 0x8000..0xC000 {
            assert_eq!(memory.get(i), memory.get(i + 0x4000));
        }
    }
}