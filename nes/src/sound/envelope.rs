
const APU_CYCLES_CLOCK_RATE: u32 = 149113;

pub struct Envelope {
    divider: u8,
    volume: u8,
    decay_level: u8,
    cpu_cycles: u32,
    constant_volume: bool
}

impl Envelope {
    pub fn decaying(volume: u8) -> Envelope {
        Envelope {
            volume: volume,
            divider: volume,
            decay_level: 15,
            cpu_cycles: 0,
            constant_volume: false
        }
    }

    pub fn constant(volume: u8) -> Envelope {
        Envelope {
            volume: volume,
            divider: 0,
            decay_level: volume,
            cpu_cycles: 0,
            constant_volume: true,
        }
    }

    pub fn clock(&mut self, cpu_cycles: u8) {
        if self.decay_level > 0 && !self.constant_volume {
            self.cpu_cycles += cpu_cycles as u32;
            if self.cpu_cycles >= APU_CYCLES_CLOCK_RATE {
                if self.divider == 0 {
                    self.decay_level -= 1;
                    self.divider = self.volume;
                } else {
                    self.divider -= 1;
                }
                self.cpu_cycles -= APU_CYCLES_CLOCK_RATE;
            }
        }
    }

    pub fn value(&self) -> u8 {
        self.decay_level
    }
}


#[cfg(test)]
mod test {
    use super::Envelope;
    use sound::counter::ClockTester;

    #[test]
    fn decaying_every_14913_clock_cycles() {
        let envelope = Envelope::decaying(10);
        assert_eq!(envelope.value(), 15);
        let mut value = 15;
        let mut clock = ClockTester::new(envelope, (10+1)*super::APU_CYCLES_CLOCK_RATE);
        for i in 0..15 {
            println!("Iteration {}", i);
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
    fn constant_volume() {
        let envelope = Envelope::constant(10);
        assert_eq!(envelope.value(), 10);
        let mut clock = ClockTester::new(envelope, (10+1)*super::APU_CYCLES_CLOCK_RATE);
        for i in 0..15 {
            clock.count_down(
                |counter, tick| counter.clock(tick),
                &|counter, cycles| assert_eq!(counter.value(), 10, "Failed on clock {}", cycles),
                &|counter, _| assert_eq!(counter.value(), 10),
            );
        }
    }
}
