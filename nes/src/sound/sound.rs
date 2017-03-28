use sound::square;
use std::rc::Rc;
use std::cell::RefCell;

pub trait AudioDevice {
    fn play(&self, &[i16]);
}

pub struct APU<T: AudioDevice> {
    audio_device: T,
    volume_scale: i16,
    square1: Rc<RefCell<square::PulseGenerator>>,
    square2: Rc<RefCell<square::PulseGenerator>>,
    cpu_cycles: u32
}

impl<T: AudioDevice> APU<T> {
    pub fn new(audio_device: T, volume_scale: i16) -> APU<T> {
        APU {
            audio_device: audio_device,
            volume_scale: volume_scale,
            square1: Rc::new(RefCell::new(square::PulseGenerator::new())),
            square2: Rc::new(RefCell::new(square::PulseGenerator::new())),
            cpu_cycles: 0,
        }
    }

    pub fn square1(&self) -> Rc<RefCell<square::PulseGenerator>> {
        self.square1.clone()
    }
    pub fn square2(&self) -> Rc<RefCell<square::PulseGenerator>> {
        self.square2.clone()
    }
}

impl<T: AudioDevice> APU<T> {
    pub fn update(&mut self, cpu_cycles: u8) {
        self.square1.borrow_mut().update(cpu_cycles);
        self.square2.borrow_mut().update(cpu_cycles);
        self.cpu_cycles += cpu_cycles as u32;
        if self.cpu_cycles >= 37 {
            self.cpu_cycles -= 37;
            self.audio_device.play(
                &[(self.square1.borrow().pulse_value()+ self.square2.borrow().pulse_value())*self.volume_scale]
            );
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
