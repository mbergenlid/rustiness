use sound::square;

pub trait AudioDevice {
    fn play(&self, &[i16]);
}

pub struct APU<T: AudioDevice> {
    audio_device: T,
    square1: square::PulseGenerator,
    cpu_cycles: u32
}

impl<T: AudioDevice> APU<T> {
    pub fn new(audio_device: T) -> APU<T> {
        APU {
            audio_device: audio_device,
            square1: square::PulseGenerator::new(1),
            cpu_cycles: 0,
        }
    }
}

impl<T: AudioDevice> APU<T> {
    pub fn update(&mut self, cpu_cycles: u8) {
        self.square1.update(cpu_cycles);
        self.cpu_cycles += cpu_cycles as u32;
        if self.cpu_cycles >= 37 {
            self.cpu_cycles -= 37;
            self.audio_device.play(&[self.square1.pulse_value()]);
        }
    }
}

#[cfg(test)]
mod test {
    use sound::counter::ClockTester;
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::{AudioDevice, APU};
    const FOUR_PULSE_SAMPLES_IN_CPU_CYCLES: u32 = 149;

    #[test]
    fn should_update_audio_device_at_correct_sample_rate() {
        let audio_device = Rc::new(RefCell::new(Vec::new()));
        let apu = APU::new(audio_device.clone());

        let mut clock = ClockTester::new(apu, FOUR_PULSE_SAMPLES_IN_CPU_CYCLES);
        clock.count_down(
            |apu, tick| apu.update(tick),
            &|_, _| (),
            &|_, _| assert_eq!(audio_device.borrow().len(), 4)
        );
    }

    impl AudioDevice for Rc<RefCell<Vec<i16>>> {
        fn play(&self, pulse: &[i16]) {
            push_all(self.borrow_mut().as_mut(), pulse);
        }
    }

    fn push_all(vec: &mut Vec<i16>, slice: &[i16]) {
        for &d in slice.iter() {
            vec.push(d);
        }
    }
}
