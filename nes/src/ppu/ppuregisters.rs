use std::cell::RefCell;
use std::rc::Rc;
use memory::MemoryMappedIO;
use ppu::PPU;
use memory::BasicMemory;

pub struct PPUCtrl(pub Rc<RefCell<PPU>>);
pub struct PPUMask(pub Rc<RefCell<PPU>>);
pub struct PPUStatus(pub Rc<RefCell<PPU>>);
pub struct PPUScroll(pub Rc<RefCell<PPU>>);
pub struct PPUAddress(pub Rc<RefCell<PPU>>);
pub struct PPUData(pub Rc<RefCell<PPU>>);
pub struct OAMAddress(pub Rc<RefCell<PPU>>);

impl MemoryMappedIO for PPUCtrl {
    fn read(&self, _: &BasicMemory) -> u8 {
        self.0.borrow().ppu_ctrl()
    }
    fn write(&mut self, _: &mut BasicMemory, value: u8) {
        self.0.borrow_mut().set_ppu_ctrl(value);
    }
}

impl MemoryMappedIO for PPUMask {
    fn read(&self, _: &BasicMemory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut BasicMemory, value: u8) {
        self.0.borrow_mut().set_ppu_mask(value);
    }
}
impl MemoryMappedIO for PPUStatus {
    fn read(&self, _: &BasicMemory) -> u8 {
        self.0.borrow_mut().status()
    }
    fn write(&mut self, _: &mut BasicMemory, _: u8) {
        unimplemented!();
    }
}
impl MemoryMappedIO for PPUScroll {
    fn read(&self, _: &BasicMemory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut BasicMemory, value: u8) {
        self.0.borrow_mut().set_scroll(value);
    }
}
impl MemoryMappedIO for PPUAddress {
    fn read(&self, _: &BasicMemory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, _: &mut BasicMemory, value: u8) {
        self.0.borrow_mut().set_vram(value);
    }
}

impl MemoryMappedIO for PPUData {
    fn read(&self, _: &BasicMemory) -> u8 {
        self.0.borrow_mut().read_from_vram()
    }

    fn write(&mut self, _: &mut BasicMemory, value: u8) {
        self.0.borrow_mut().write_to_vram(value);
    }
}

impl MemoryMappedIO for OAMAddress {
    fn read(&self, _: &BasicMemory) -> u8 {
        unimplemented!();
    }
    fn write(&mut self, memory: &mut BasicMemory, value: u8) {
        let dma_address: usize = (value as usize) << 8;
        self.0.borrow_mut().load_sprites(&memory[dma_address..(dma_address+256)]);
    }
}

#[cfg(test)]
mod test {
    use memory::BasicMemory;
    use memory::Memory;
    use ppu::PPU;
    use super::{PPUAddress,PPUData};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_write_to_vram() { let ppu = Rc::new(RefCell::new(PPU::new(box BasicMemory::new())));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );

            memory.set(0x2006, 0xFF); //High byte of vram pointer
            memory.set(0x2006, 0x01); //Low byte of vram pointer

            memory.set(0x2007, 0xA5); //write 0xA5 to PPU-MEM 0xFF01

            memory.set(0x2006, 0xFF); //High byte of vram pointer
            memory.set(0x2006, 0x01); //Low byte of vram pointer
            assert_eq!(0xA5, memory.get(0x2007));
        }
        assert_eq!(0xA5, ppu.borrow().memory().get(0x3F01));

        {
            let basic_memory = BasicMemory::new();
            let mut memory = cpu_memory!(
                box basic_memory,
                0x2006 => MutableRef::Box(box PPUAddress(ppu.clone())),
                0x2007 => MutableRef::Box(box PPUData(ppu.clone()))
            );
            memory.set(0x2007, 0x3B); //vram pointer should have been increased
        }
        assert_eq!(0x3B, ppu.borrow().memory().get(0x3F02));
    }

}
