#[cfg(test)]
mod test {

    use cpu::opcodes::*;
    use cpu::CpuBuilder;
    use memory::Memory;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn absolute_indexed_addressing_should_do_dummy_read() {
        //LDA, LDX, LDY, EOR, AND, ORA, ADC, SBC, CMP
        for &op in [
            LDA_ABSOLUTE_X,
            LDA_ABSOLUTE_Y,
            LDX_ABSOLUTE_Y,
            LDY_ABSOLUTE_X,
            EOR_ABSOLUTE_X,
            EOR_ABSOLUTE_Y,
            AND_ABSOLUTE_X,
            AND_ABSOLUTE_Y,
            ORA_ABSOLUTE_X,
            ORA_ABSOLUTE_Y,
            ADC_ABSOLUTE_X,
            ADC_ABSOLUTE_Y,
            SBC_ABSOLUTE_X,
            SBC_ABSOLUTE_Y,
            CMP_ABSOLUTE_X,
            CMP_ABSOLUTE_Y,
        ]
        .iter()
        {
            let memory = memory!(
                0x8000 => op,
                0x8001 => 0xE0,
                0x8002 => 0x20
            );
            let (cycles, memory_spy) = set_up(Box::new(memory));
            assert_eq!(cycles, 5);
            assert_eq!(memory_spy.address_read_count(0x2102), 1);
            assert_eq!(memory_spy.address_read_count(0x2002), 1);

            let memory = memory!(
                0x8000 => op,
                0x8001 => 0x00,
                0x8002 => 0x20
            );
            let (cycles, memory_spy) = set_up(Box::new(memory));
            assert_eq!(cycles, 4);
            assert_eq!(memory_spy.address_read_count(0x2022), 1);
        }
    }

    #[test]
    fn indirect_indexed_addressing_should_do_dummy_read() {
        //LDA, EOR, AND, ORA, ADC, SBC, CMP
        for &op in [
            LDA_INDIRECT_Y,
            EOR_INDIRECT_Y,
            AND_INDIRECT_Y,
            ORA_INDIRECT_Y,
            ADC_INDIRECT_Y,
            SBC_INDIRECT_Y,
            CMP_INDIRECT_Y,
        ]
        .iter()
        {
            let memory = memory!(
                0x8000 => op,
                0x8001 => 0x05,

                0x0005 => 0xE0,
                0x0006 => 0x20
            );
            let (cycles, memory_spy) = set_up(Box::new(memory));
            assert_eq!(cycles, 6);
            assert_eq!(memory_spy.address_read_count(0x2102), 1);
            assert_eq!(memory_spy.address_read_count(0x2002), 1);

            let memory = memory!(
                0x8000 => op,
                0x8001 => 0x05,

                0x0005 => 0x00,
                0x0006 => 0x20
            );
            let (cycles, memory_spy) = set_up(Box::new(memory));
            assert_eq!(cycles, 5);
            assert_eq!(memory_spy.address_read_count(0x2022), 1);
        }
    }

    struct MemorySpy {
        memory: Box<Memory>,
        reads: RefCell<HashMap<u16, u32>>,
    }

    impl MemorySpy {
        fn new(memory: Box<Memory>) -> MemorySpy {
            MemorySpy {
                memory: memory,
                reads: RefCell::new(HashMap::new()),
            }
        }

        fn address_read_count(&self, address: u16) -> u32 {
            *self.reads.borrow().get(&address).unwrap_or(&0)
        }
    }

    impl Memory for MemorySpy {
        fn get(&self, address: u16, sub_cycle: u8) -> u8 {
            let mut reads = self.reads.borrow_mut();
            let reads_at_address = reads.entry(address).or_insert(0);
            *reads_at_address += 1;

            self.memory.get(address, sub_cycle)
        }
        fn set(&mut self, address: u16, value: u8, sub_cycle: u8) {
            self.memory.set(address, value, sub_cycle)
        }
    }

    fn set_up(memory: Box<Memory>) -> (u8, MemorySpy) {
        let mut memory_spy = MemorySpy::new(memory);
        let opcodes = OpCodes::new();
        let mut cpu = CpuBuilder::new().register_x(0x22).register_y(0x22).build();
        let cycles = opcodes.execute_instruction(&mut cpu, &mut memory_spy);
        return (cycles, memory_spy);
    }
}
