pub struct LengthCounter {
    value: u8,
    cpu_cycles: u32,
    halted: bool
}
const APU_CYCLES_CLOCK_RATE: u32 = 14913;

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
    use sound::counter::ClockTester;

    #[test]
    fn count_down_every_14913_clock_cycles() {
        let length_counter = LengthCounter::new(10);
        assert_eq!(length_counter.value(), 60);
        let mut clock = ClockTester::new(length_counter, super::APU_CYCLES_CLOCK_RATE);
        let mut value = 60;
        for _ in 0..60 {
            clock.count_down(
                |counter, tick| counter.clock(tick),
                &|counter, cycles| assert_eq!(counter.value(), value, "Failed on clock {}", cycles),
                &|counter, _| assert_eq!(counter.value(), value-1),
            );
            value -= 1;
        }

        for _ in 0..10 {
            clock.count_down(
                |counter, tick| counter.clock(tick),
                &|counter, cycles| assert_eq!(counter.value(), 0, "Failed on clock {}", cycles),
                &|counter, _| assert_eq!(counter.value(), 0),
            );
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
}
