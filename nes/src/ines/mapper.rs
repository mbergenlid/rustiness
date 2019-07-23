use ines::INes;
use memory::{BasicMemory, Memory};
use ppu::ppumemory::PPUMemory;
use std::fs::File;

pub struct Mapper {
    pub cpu_memory: Box<dyn Memory>,
    pub ppu_memory: PPUMemory,
}

pub fn from_file(file_name: &str) -> Mapper {
    let ines = INes::read(&mut File::open(file_name).unwrap());

    Mapper {
        cpu_memory: {
            let mut cpu_memory = box BasicMemory::new();
            cpu_memory.set_slice(0x8000, ines.prg_rom(0));
            if ines.num_prg_roms == 1 {
                cpu_memory.set_slice(0xC000, ines.prg_rom(0));
            } else if ines.num_prg_roms == 2 {
                cpu_memory.set_slice(0xC000, ines.prg_rom(1));
            } else {
                panic!(".nes file contains more than 2 prg rom banks which is not allowed");
            }
            cpu_memory
        },
        ppu_memory: {
            let mut ppu_mem = PPUMemory::new(ines.mirroring);
            if ines.num_chr_roms > 0 {
                ppu_mem.set_slice(0x0000, ines.chr_rom(0));
            }
            ppu_mem
        },
    }
}
