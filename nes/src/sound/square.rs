use super::sound::Pulse;
use sound::length_counter::LengthCounter;
use sound::envelope::Envelope;

pub struct SquarePulse {
    pulse: Vec<i16>
}

impl Pulse for SquarePulse {
    fn get(&self) -> &[i16] {
        &self.pulse
    }
}

struct CircularBuffer {
    buffer: [i16; 8],
    index: usize,
}

impl CircularBuffer {
    fn next(&mut self) {
        self.index += 1;
        if self.index >= 8 {
            self.index = 0;
        }
    }

    fn get(&self) -> i16 {
        self.buffer[self.index]
    }
}

pub struct PulseGenerator {
    volume_scale: i16,
    envelope: Envelope,
    timer_set: u32,
    timer: u32,
    sequencer: CircularBuffer,
    length: LengthCounter,
}

impl PulseGenerator {
    pub fn new(volume_scale: i16) -> PulseGenerator {
        PulseGenerator {
            volume_scale: volume_scale,
            envelope: Envelope::constant(0),
            timer_set: 0,
            timer: 0,
            length: LengthCounter::new(0),

            sequencer: CircularBuffer {
                buffer: [0,1,1,1,1,0,0,0],
                index: 0,
            },
        }
    }

    pub fn volume(&mut self, volume: u8) {
        self.envelope = Envelope::constant(volume);
    }

    pub fn decaying_volume(&mut self, volume: u8) {
        self.envelope = Envelope::decaying(volume);
    }

    pub fn timer(&mut self, timer: u32) {
        let adjusted_timer = timer*2;
        self.timer_set = adjusted_timer;
        self.timer = 0;
    }

    pub fn length(&mut self, length: u8) {
        self.length = LengthCounter::new(length);
    }

    pub fn update(&mut self, cpu_cycles: u8) {
        self.timer += cpu_cycles as u32;
        self.length.clock(cpu_cycles);
        self.envelope.clock(cpu_cycles);
        if self.timer >= self.timer_set {
            self.timer -= self.timer_set;
            self.sequencer.next();
        }
    }

    pub fn pulse_value(&self) -> i16 {
        if self.length.value() > 0 {
            self.sequencer.get() * self.envelope.value() as i16
        } else {
            0
        }
    }
}



#[cfg(test)]
mod test {
    use super::PulseGenerator;
    const APU_CYCLES_CLOCK_RATE: u64 = 149113;
    use sound::counter::ClockTester;

    #[test]
    fn simple_constant_square_wave() {
        let mut generator: PulseGenerator = PulseGenerator::new(1);
        generator.volume(10);
        generator.timer(0x1AA);
        generator.length(09);

        assert_eq!(generator.pulse_value(), 0);
        let mut clock = ClockTester::new(generator, 426*2);
        {
            clock.count_down(
                |gen, tick| gen.update(tick),
                &|gen, cycles| assert_eq!(gen.pulse_value(), 0, "After {} cycles", cycles),
                &|gen, _| assert_eq!(gen.pulse_value(), 10),
            );
        }

        for _ in 0..176 {
            execute_one_cycle(
                &mut clock,
                &|gen, cycles| assert_eq!(gen.pulse_value(), if cycles >= 149113*8 { 0 } else { 10 }, "After {} cycles", cycles)
            );
        }
    }

    #[test]
    fn simple_decaying_square_wave() {
        let mut generator = PulseGenerator::new(1);
        generator.decaying_volume(4);
        generator.timer(0x1AA);
        generator.length(01);
        let decaying_period = 149113*5;
        let mut clock = ClockTester::new(generator, 426*2);
        {
            clock.count_down(
                |gen, tick| gen.update(tick),
                &|gen, cycles| assert_eq!(gen.pulse_value(), 0, "After {} cycles", cycles),
                &|gen, cycles| assert_eq!(gen.pulse_value(), 15 - (cycles/decaying_period) as i16),
            );
        }
        for _ in 0..1500 {
            execute_one_cycle(
                &mut clock,
                &|gen, cycles| assert_eq!(gen.pulse_value(), 15 - (cycles/decaying_period) as i16, "After {} cycles", cycles)
            );
        }
    }

    fn execute_one_cycle<F>(clock: &mut ClockTester<PulseGenerator>, assert_value_high: &F) where F: Fn(&PulseGenerator, u64) {
        {
            for _ in 0..3 {
                clock.count_down(
                    |gen, tick| gen.update(tick),
                    assert_value_high,
                    assert_value_high,
                );
            }
            clock.count_down(
                |gen, tick| gen.update(tick),
                assert_value_high,
                &|gen, cycles| assert_eq!(gen.pulse_value(), 0, "After {} cycles", cycles),
            );
        }
        {
            for _ in 0..3 {
                clock.count_down(
                    |gen, tick| gen.update(tick),
                    &|gen, cycles| assert_eq!(gen.pulse_value(), 0, "After {} cycles", cycles),
                    &|gen, _| assert_eq!(gen.pulse_value(), 0),
                );
            }
            clock.count_down(
                |gen, tick| gen.update(tick),
                &|gen, cycles| assert_eq!(gen.pulse_value(), 0, "After {} cycles", cycles),
                assert_value_high
            );
        }
    }
}
