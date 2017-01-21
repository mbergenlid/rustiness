
use ppu::PPU;
pub type Address = u16;

pub trait Memory {
    fn get(&self, address: Address) -> u8;
    fn set(&mut self, address: Address, value: u8);

    fn set_slice(&mut self, start: Address, data: &[u8]) {
        let mut address = start-1;
        for &d in data {
            address += 1;
            self.set(address, d);
        }
    }
}

pub struct BasicMemory {
    data: [u8; 65_536]
}

impl BasicMemory {
    pub fn new() -> BasicMemory {
        return BasicMemory { data: [0; 65_536]};
    }
}

impl Memory for BasicMemory {
    fn get(&self, address: Address) -> u8 {
        return self.data[address as usize];
    }

    fn set(&mut self, address: Address, value: u8) {
        self.data[address as usize] = value;
    }
}

pub struct CPUMemory<'a> {
    data: &'a mut BasicMemory,
    ppu: &'a mut PPU,
}

impl <'a> CPUMemory<'a> {
    pub fn new(ppu: &'a mut PPU, memory: &'a mut BasicMemory) -> CPUMemory<'a> {
        CPUMemory {
            data: memory,
            ppu: ppu,
        }
    }
}

impl <'a> Memory for CPUMemory<'a> {
    fn get(&self, address: Address) -> u8 {
        self.data.get(address)
    }

    fn set(&mut self, address: Address, value: u8) {
        if address == 0x2000 {
            self.ppu.set_ppu_ctrl(value);
        } else if address == 0x2006 {
            self.ppu.set_vram(value);
        } else if address == 0x2007 {
            self.ppu.write_to_vram(value);
        } else {
            self.data.set(address, value);
        }
    }
}

macro_rules! memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use memory;
            let mut temp_memory = memory::BasicMemory::new();
            $(
                temp_memory.set($x, $y);
            )*
            temp_memory
        }
    };
}

#[macro_export]
macro_rules! external_memory {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            use nes::memory::Memory;
            let mut temp_memory = nes::memory::BasicMemory::new();
            $(
                temp_memory.set($x, $y);
            )*
            temp_memory
        }
    };
}

#[cfg(test)]
mod test {
    use ppu::screen::{Color, Screen};
    use super::BasicMemory;
    use super::Memory;
    use ppu::PPU;

    struct ScreenMock {
        colors: [[Color; 240]; 256],
    }

    impl Screen for ScreenMock {
        fn set_color(&mut self, x: usize, y: usize, color: Color) {
            self.colors[y][x] = color
        }
        fn draw(&mut self) {

        }
        fn get_row(&self, _: usize) -> &[Color] {
            unimplemented!()
        }
    }

    #[test]
    fn test_write_to_vram() {
        let mut ppu = PPU::new(
                box (BasicMemory::new()),
                box (ScreenMock {colors: [[[0.1, 0.1, 0.1]; 240]; 256]})
            );

        let mut basic_memory = BasicMemory::new();
        {
            let mut memory = super::CPUMemory::new(&mut ppu, &mut basic_memory);

            memory.set(0x2006, 0xFF); //High byte of vram pointer
            memory.set(0x2006, 0x01); //Low byte of vram pointer

            memory.set(0x2007, 0xA5); //write 0xA5 to PPU-MEM 0xFF01
        }
        assert_eq!(0xA5, ppu.memory().get(0xFF01));

        {
            let mut memory = super::CPUMemory::new(&mut ppu, &mut basic_memory);
            memory.set(0x2007, 0x3B); //vram pointer should have been increased
        }
        assert_eq!(0x3B, ppu.memory().get(0xFF02));
    }
}