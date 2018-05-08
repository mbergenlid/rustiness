use sound::length_counter::LengthCounter;
use sound::envelope::Envelope;
use Cycles;

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
    envelope: Envelope,
    timer_set: u32,
    timer: u32,
    sequencer: CircularBuffer,
    length: LengthCounter,
}

impl PulseGenerator {
    pub fn new() -> PulseGenerator {
        PulseGenerator {
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

    pub fn timer_low(&mut self, timer_low: u8) {
        self.timer_set = (self.timer_set & 0xFF_FF_FE_00) | ((timer_low as u32) << 1);
        self.timer = 0;
    }

    pub fn timer_high(&mut self, timer_high: u8) {
        self.timer_set = (self.timer_set & 0xFF_FF_F1_FF) | ((timer_high as u32) << 9);
        self.timer = 0;
    }

    pub fn timer(&mut self, timer: u32) {
        self.timer_low(timer as u8);
        self.timer_high((timer >> 8) as u8);
    }

    pub fn length(&mut self, length: u8) {
        self.length = LengthCounter::new(length);
    }

    pub fn update(&mut self, cpu_cycles: Cycles) {
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

