use memory::{MemoryMappedIO, Memory};
use super::square::PulseGenerator;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Register1(pub Rc<RefCell<PulseGenerator>>);
pub struct Register3(pub Rc<RefCell<PulseGenerator>>);
pub struct Register4(pub Rc<RefCell<PulseGenerator>>);


impl MemoryMappedIO for Register1 {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }

    fn write(&mut self, _: &mut Memory, value: u8) {
        if value & 0x10 > 0 {
            self.0.borrow_mut().volume(value & 0xF);
        } else {
            self.0.borrow_mut().decaying_volume(value & 0xF);
        }
    }
}
impl MemoryMappedIO for Register3 {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }

    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().timer_low(value);
    }
}
impl MemoryMappedIO for Register4 {
    fn read(&self, _: &Memory) -> u8 {
        unimplemented!();
    }

    fn write(&mut self, _: &mut Memory, value: u8) {
        self.0.borrow_mut().length(value >> 3);
        self.0.borrow_mut().timer_high(value & 0x07);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use memory::{Memory, BasicMemory, CPUMemory};
    use sound::square::PulseGenerator;
    use std::cell::RefCell;
    use sound::AudioDevice;

    use sound::counter::ClockTester;

    #[test]
    fn simple_constant_square_wave() {
        let generator = Rc::new(RefCell::new(PulseGenerator::new()));
        let mut cpu_memory = cpu_memory(generator.clone());
        cpu_memory.set(0x4000, 0x1A);
        cpu_memory.set(0x4002, 0xAA);
        cpu_memory.set(0x4003, 0b0100_1001);

        assert_eq!(generator.borrow().pulse_value(), 0);
        let mut clock = ClockTester::new(generator, 426*2);
        {
            clock.count_down(
                |gen, tick| gen.borrow_mut().update(tick),
                &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 0, "After {} cycles", cycles),
                &|gen, _| assert_eq!(gen.borrow().pulse_value(), 10),
            );
        }

        for _ in 0..176 {
            execute_one_cycle(
                &mut clock,
                &|gen, cycles| assert_eq!(gen.pulse_value(), if cycles >= 14913*8 { 0 } else { 10 }, "After {} cycles", cycles)
            );
        }
    }

    #[test]
    fn simple_decaying_square_wave() {
        let generator = Rc::new(RefCell::new(PulseGenerator::new()));
        let mut cpu_memory = cpu_memory(generator.clone());
        cpu_memory.set(0x4000, 4);
        cpu_memory.set(0x4002, 0xAA);
        cpu_memory.set(0x4003, 0b0000_1001);
        let decaying_period = 14913*5;
        let mut clock = ClockTester::new(generator, 426*2);
        {
            clock.count_down(
                |gen, tick| gen.borrow_mut().update(tick),
                &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 0, "After {} cycles", cycles),
                &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 15 - (cycles/decaying_period) as i16),
            );
        }
        use std::cmp;
        for _ in 0..1500 {
            execute_one_cycle(
                &mut clock,
                &|gen, cycles| assert_eq!(gen.pulse_value(), cmp::max(0, 15 - (cycles/decaying_period) as i16), "After {} cycles", cycles)
            );
        }
    }

    impl AudioDevice for RefCell<Vec<i16>> {
        fn play(&self, pulse: &[i16]) {
            push_all(self.borrow_mut().as_mut(), pulse);
        }
    }

    fn cpu_memory(square1: Rc<RefCell<PulseGenerator>>) -> CPUMemory<'static> {
        cpu_memory!(
            box BasicMemory::new(),
            0x4000 => MutableRef::Box(box Register1(square1.clone())),
            0x4002 => MutableRef::Box(box Register3(square1.clone())),
            0x4003 => MutableRef::Box(box Register4(square1.clone()))
        )
    }

    fn push_all(vec: &mut Vec<i16>, slice: &[i16]) {
        for &d in slice.iter() {
            vec.push(d);
        }
    }

    fn execute_one_cycle<F>(clock: &mut ClockTester<Rc<RefCell<PulseGenerator>>>, assert_value_high: &F) where F: Fn(&PulseGenerator, u64) {
        {
            for _ in 0..3 {
                clock.count_down(
                    |gen, tick| gen.borrow_mut().update(tick),
                    &|gen, cycles| assert_value_high(&gen.borrow(), cycles),
                    &|gen, cycles| assert_value_high(&gen.borrow(), cycles),
                );
            }
            clock.count_down(
                |gen, tick| gen.borrow_mut().update(tick),
                &|gen, cycles| assert_value_high(&gen.borrow(), cycles),
                &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 0, "After {} cycles", cycles),
            );
        }
        {
            for _ in 0..3 {
                clock.count_down(
                    |gen, tick| gen.borrow_mut().update(tick),
                    &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 0, "After {} cycles", cycles),
                    &|gen, _| assert_eq!(gen.borrow().pulse_value(), 0),
                );
            }
            clock.count_down(
                |gen, tick| gen.borrow_mut().update(tick),
                &|gen, cycles| assert_eq!(gen.borrow().pulse_value(), 0, "After {} cycles", cycles),
                &|gen, cycles| assert_value_high(&gen.borrow(), cycles),
            );
        }
    }
}
