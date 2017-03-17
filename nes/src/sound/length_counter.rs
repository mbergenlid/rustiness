pub struct LengthCounter {
    value: u8,
    cpu_cycles: u32,
    halted: bool
}
const APU_CYCLES_CLOCK_RATE: u32 = 149113;

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];

impl LengthCounter {
    pub fn new(length: u8) -> LengthCounter {
        LengthCounter {
            value: LENGTH_TABLE[length as usize],
            cpu_cycles: 0,
            halted: false,
        }
    }

    pub fn clock(&mut self, cpu_cycles: u8) {
        if self.value > 0 && !self.halted {
            self.cpu_cycles += cpu_cycles as u32;
            if self.cpu_cycles >= APU_CYCLES_CLOCK_RATE {
                self.value -= 1;
                self.cpu_cycles -= APU_CYCLES_CLOCK_RATE;
            }
        }
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[cfg(test)]
mod test {
    use super::LengthCounter;

    #[test]
    fn count_down_every_14913_clock_cycles() {
        let mut length_counter = LengthCounter::new(10);
        let mut cpu_clock = 0;
        assert_eq!(length_counter.value(), 60);
        for _ in 0..60 {
            count_down_1_step(&mut cpu_clock, &mut length_counter);
        }
        assert_eq!(length_counter.value(), 0);

        for _ in 0..super::APU_CYCLES_CLOCK_RATE {
            length_counter.clock(1);
            assert_eq!(length_counter.value(), 0);
        }
    }

    #[test]
    fn length_counter_should_be_haltable() {
        let mut length_counter = LengthCounter::new(10);
        length_counter.halt();

        for _ in 0..(super::APU_CYCLES_CLOCK_RATE*2) {
            length_counter.clock(1);
            assert_eq!(length_counter.value(), 60);
        }
    }

    extern crate rand;
    fn count_down_1_step(cpu_clock: &mut u64, length_counter: &mut LengthCounter) {
        let value = length_counter.value();
        let clocks_required = *cpu_clock + (super::APU_CYCLES_CLOCK_RATE as u64 - (*cpu_clock % super::APU_CYCLES_CLOCK_RATE as u64));
        while *cpu_clock < clocks_required {
            let tick = rand::random::<u8>();
            length_counter.clock(tick);
            *cpu_clock += tick as u64;
            if *cpu_clock >= clocks_required {
                assert_eq!(length_counter.value(), value-1);
            } else {
                assert_eq!(length_counter.value(), value, "Failed on clock {}", *cpu_clock);
            }
        }
    }
}
