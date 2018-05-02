use std::cell::RefCell;
use std::rc::Rc;
use memory::MemoryMappedIO;
use ppu::PPU;
use memory::Memory;

pub struct PPUCtrl(pub Rc<RefCell<PPU>>);
pub struct PPUMask(pub Rc<RefCell<PPU>>);
pub struct PPUStatus(pub Rc<RefCell<PPU>>);
pub struct PPUScroll(pub Rc<RefCell<PPU>>);
pub struct PPUAddress(pub Rc<RefCell<PPU>>);
pub struct PPUData(pub Rc<RefCell<PPU>>);
pub struct OAMAddress(pub Rc<RefCell<PPU>>);
pub struct OAMData(pub Rc<RefCell<PPU>>);
pub struct OAMDMA(pub Rc<RefCell<PPU>>);

impl MemoryMappedIO for PPUCtrl {
    fn read(&self, _: &Memory) -> u8 {
        self.0.borrow().ppu_ctrl()
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().set_ppu_ctrl_at_cycle(value, 0);
    }

    fn write_at_cycle(&mut self, _: &mut Memory, value: u8, sub_cycle: u8) {
        self.0.borrow_mut().set_ppu_ctrl_at_cycle(value, sub_cycle);
    }
}

impl MemoryMappedIO for PPUMask {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().set_ppu_mask(value, 0);
    }

    fn write_at_cycle(&mut self, _: &mut Memory, value: u8, sub_cycle: u8) {
        self.0.borrow_mut().set_ppu_mask(value, sub_cycle);
    }
}
impl MemoryMappedIO for PPUStatus {
    fn read(&self, _: &Memory) -> u8 {
        self.0.borrow_mut().status(0)
    }
    fn write(&mut self, _: &mut Memory, _: u8) {
        //Do nothing
    }
    fn read_at_cycle(&self, _: &Memory, sub_cycle: u8) -> u8 {
        self.0.borrow_mut().status(sub_cycle)
    }
}
impl MemoryMappedIO for PPUScroll {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().set_scroll(value);
    }
}
impl MemoryMappedIO for PPUAddress {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().set_vram(value);
    }
}

impl MemoryMappedIO for PPUData {
    fn read(&self, _: &Memory) -> u8 {
        self.0.borrow_mut().read_from_vram()
    }

    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().write_to_vram(value);
    }
}

impl MemoryMappedIO for OAMDMA {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, memory: &mut Memory, value: u8) {
        let dma_address: u16 = (value as u16) << 8;
        let mut ppu = self.0.borrow_mut();
        let sprites = ppu.sprites_mut();
        let oam_address = sprites.address() as usize;
        if oam_address == 0 {
            memory.dma(dma_address..(dma_address+256), sprites.slice());
        } else {
            let wrap_around_address: u16 = dma_address+(256-oam_address as u16);
            memory.dma(dma_address..wrap_around_address, &mut sprites.slice()[oam_address..(256-oam_address)+1]);
            memory.dma(wrap_around_address..(dma_address+256), &mut sprites.slice()[0..]);
        }
    }
}

impl MemoryMappedIO for OAMAddress {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().sprites_mut().set_address(value);
    }
}

impl MemoryMappedIO for OAMData {
    fn read(&self, _: &Memory) -> u8 {
        self.0.borrow().sprites().read_byte()
    }
    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().sprites_mut().write_byte(value);
    }
}

#[cfg(test)]
mod test {
    use memory::BasicMemory;
    use memory::Memory;
    use ppu::PPU;
    use ppu::ppumemory::PPUMemory;
    use super::{PPUAddress,PPUData, OAMAddress, OAMData, OAMDMA};
    use std::rc::Rc;
    use std::cell::RefCell;


    #[test]
    fn test_write_to_vram() { 
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::no_mirroring())));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );

            memory.set(0x2006, 0xFF, 0); //High byte of vram pointer
            memory.set(0x2006, 0x01, 0); //Low byte of vram pointer

            memory.set(0x2007, 0xA5, 0); //write 0xA5 to PPU-MEM 0xFF01
        }
        assert_eq!(0xA5, ppu.borrow().memory().get(0x3F01, 0));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );
            memory.set(0x2007, 0x3B, 0); //vram pointer should have been increased
        }
        assert_eq!(0x3B, ppu.borrow().memory().get(0x3F02, 0));
    }

    use memory::SharedMemory;
    use ppu::ppumemory::Mirroring;
    #[test]
    fn test_read_from_vram() { 
        let ppu_internal_memory = memory!(
            0x2000 => 0x05,
            0x2001 => 0x10
        );
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::wrap(SharedMemory::wrap(ppu_internal_memory), Mirroring::NoMirroring))));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );

            memory.set(0x2006, 0x20, 0); //High byte of vram pointer
            memory.set(0x2006, 0x00, 0); //Low byte of vram pointer

            memory.get(0x2007, 0); //Dummy read
            assert_eq!(0x05, memory.get(0x2007, 0));
            assert_eq!(0x10, memory.get(0x2007, 0));
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn read_from_vram_3Fxx_does_not_require_dummy_read() {
        let ppu_internal_memory = memory!(
            0x3F00 => 0x05,
            0x3F01 => 0x10
        );
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::wrap(SharedMemory::wrap(ppu_internal_memory), Mirroring::NoMirroring))));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );

            memory.set(0x2006, 0x3F, 0); //High byte of vram pointer
            memory.set(0x2006, 0x00, 0); //Low byte of vram pointer

            assert_eq!(0x05, memory.get(0x2007, 0));
            assert_eq!(0x10, memory.get(0x2007, 0));
        }
    }

    #[test]
    fn palette_read_should_also_read_vram_into_buffer() {
        let ppu_internal_memory = memory!(
            0x2f12 => 0x9A,
            0x3F12 => 0x05
        );
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::wrap(SharedMemory::wrap(ppu_internal_memory), Mirroring::NoMirroring))));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );

            memory.set(0x2006, 0x3F, 0); //High byte of vram pointer
            memory.set(0x2006, 0x12, 0); //Low byte of vram pointer

            assert_eq!(0x05, memory.get(0x2007, 0));

            memory.set(0x2006, 0x2F, 0);
            memory.set(0x2006, 0x12, 0);
            assert_eq!(0x9A, memory.get(0x2007, 0));
        }
    }

    #[test]
    fn oam_address_should_increase_on_write() {
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::no_mirroring())));
        let basic_memory = BasicMemory::new();
        let mut memory = cpu_memory!(
            box basic_memory,
            0x2003 => MutableRef::Box(box OAMAddress(ppu.clone())),
            0x2004 => MutableRef::Box(box OAMData(ppu.clone()))
        );

        memory.set(0x2003, 0x0, 0);
        memory.set(0x2004, 0x12, 0);
        memory.set(0x2004, 0x34, 0);

        memory.set(0x2003, 0x0, 0);
        assert_eq!(0x12, memory.get(0x2004, 0));
        memory.set(0x2003, 0x1, 0);
        assert_eq!(0x34, memory.get(0x2004, 0));
    }

    #[test]
    fn oam_address_should_not_increase_on_read() {
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::no_mirroring())));
        let basic_memory = BasicMemory::new();
        let mut memory = cpu_memory!(
            box basic_memory,
            0x2003 => MutableRef::Box(box OAMAddress(ppu.clone())),
            0x2004 => MutableRef::Box(box OAMData(ppu.clone()))
        );

        memory.set(0x2003, 0x0, 0);
        memory.set(0x2004, 0x12, 0);
        memory.set(0x2004, 0x34, 0);

        memory.set(0x2003, 0x0, 0);
        assert_eq!(0x12, memory.get(0x2004, 0));
        assert_eq!(0x12, memory.get(0x2004, 0));
    }

    #[test]
    fn dma_copy_should_start_at_oam_address_and_wrap() {
        let ppu = Rc::new(RefCell::new(PPU::new(PPUMemory::no_mirroring())));
        let basic_memory = memory!(
            0x0200 => 1,
            0x0201 => 2,
            0x0203 => 3,
            0x0204 => 4
        );
        let mut memory = cpu_memory!(
            box basic_memory,
            0x2003 => MutableRef::Box(box OAMAddress(ppu.clone())),
            0x2004 => MutableRef::Box(box OAMData(ppu.clone())),
            0x4014 => MutableRef::Box(box OAMDMA(ppu.clone()))
        );

        memory.set(0x2003, 0x1, 0);
        memory.set(0x4014, 0x02, 0);
        memory.set(0x2003, 0x1, 0);
        assert_eq!(1, memory.get(0x2004, 0));
    }
}
