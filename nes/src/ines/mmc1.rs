use memory::{Address, Memory, BasicMemory};
use ines::INes;

pub struct MMC1MemoryMapper {
    internal_memory: BasicMemory,
    prg_rom_banks: Vec<[u8; 0x4000]>,
    low_prg_rom_bank: usize,
    high_prg_rom_bank: usize,

    shift_register: u8,
}

impl MMC1MemoryMapper {
    pub fn new(ines: &INes) -> MMC1MemoryMapper {
        let num_prg_banks = ines.num_prg_roms as usize;
        let mut prg_rom: Vec<[u8; 0x4000]> = Vec::with_capacity(num_prg_banks);
        for rom_bank in 0..num_prg_banks {
            let mut bank = [0; 0x4000];
            bank.clone_from_slice(ines.prg_rom(rom_bank));
            prg_rom.push(bank);
        }
        MMC1MemoryMapper {
            internal_memory: BasicMemory::new(),
            prg_rom_banks: prg_rom,
            low_prg_rom_bank: 0,
            high_prg_rom_bank: 15,

            shift_register: 0b1_0000,
        }
    }
}

impl Memory for MMC1MemoryMapper {
    fn get(&self, address: Address) -> u8 {
        if address >= 0xC000 {
            self.prg_rom_banks[self.high_prg_rom_bank][address as usize - 0xC000]
        } else if address >= 0x8000 {
            self.prg_rom_banks[self.low_prg_rom_bank][address as usize - 0x8000]
        } else {
            self.internal_memory.get(address)
        }
    }
    fn set(&mut self, _: Address, value: u8) {
        let new_shift_register = (value & 0x01) << 4 | (self.shift_register >> 1);
        if self.shift_register & 0x01 == 0 {
            self.shift_register = new_shift_register;
        } else {
            self.low_prg_rom_bank = new_shift_register as usize;
            self.shift_register = 0x10;
        }
    }
}


#[cfg(test)]
mod test {
    use super::MMC1MemoryMapper;
    use ines::INes;
    use memory::Memory;

    const INES_HEADER: [u8; 16] = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    #[test]
    fn initial_state() {
        let rom = fill_banks();
        let mapper = MMC1MemoryMapper::new(&INes::read(&mut ArrayStream(rom.iter())));
        assert_eq!(1, mapper.get(0x8000)); //8000 -> switched to first bank.
        assert_eq!(16, mapper.get(0xC000)); //C000 -> fixed to last bank
        assert_eq!(0, mapper.get(0x1000)); //1000 -> mapped to NES internal memory
    }

    #[test]
    fn should_be_able_to_switch_the_log_bank() {
        let mut mapper = MMC1MemoryMapper::new(&INes::read(&mut ArrayStream(fill_banks().iter())));
        assert_eq!(1, mapper.get(0x8000)); //8000 -> switched to first bank.
        write_to_prg_bank_register(&mut mapper, 1);
        assert_eq!(2, mapper.get(0x8000)); //8000 -> switched to second bank.
        write_to_prg_bank_register(&mut mapper, 5);
        assert_eq!(6, mapper.get(0x8000));
    }

    #[test]
    fn writing_only_four_times_to_load_register_should_not_switch() {
        let mut mapper = MMC1MemoryMapper::new(&INes::read(&mut ArrayStream(fill_banks().iter())));
        for _ in 0..4 { mapper.set(0xE000, 1); }
        assert_eq!(1, mapper.get(0x8000)); //8000 -> switched to first bank.
    }

    fn write_to_prg_bank_register(mapper: &mut MMC1MemoryMapper, value: u8) {
        let mut index = value;
        for _ in 0..5 {
            mapper.set(0xE000, index);
            index = index >> 1;
        }
    }

    fn fill_banks() -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(16*0x4001);
        for &d in INES_HEADER.iter() {
            vec.push(d);
        }
        for x in 1..17 {
            for _ in 0..0x4000 {
                vec.push(x);
            }
        }
        return vec;
    }

    struct ArrayStream<'a>(Iter<'a, u8>);

    use std::slice::Iter;
    use std::io::Read;
    use std::io::Result;
    impl <'a> Read for ArrayStream<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut index = 0;
            let mut next = self.0.next();
            while next.is_some() {
                buf[index] = *next.unwrap();
                index += 1;
                if index >= buf.len() {
                    return Ok(index);
                } 
                next = self.0.next();
            }
            Ok(index)
        }
    }
}
