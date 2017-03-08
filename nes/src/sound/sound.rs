pub trait Pulse {
    fn get(&self) -> &[i16];
}

pub trait AudioDevice {
    fn play(&self, &Pulse);
}

pub struct SquarePulse {
    pulse: Vec<i16>
}

impl Pulse for SquarePulse {
    fn get(&self) -> &[i16] {
        &self.pulse
    }
}

pub struct PulseGenerator {
    volume_scale: i16
}

impl PulseGenerator {
    pub fn new(volume_scale: i16) -> PulseGenerator {
        PulseGenerator { volume_scale: volume_scale }
    }

    pub fn constant_volume(&self, volume: i16, wave_length: u16, length: u8) -> SquarePulse {
        let period = (((wave_length as u32 + 1)*768) / 1790) / 2;
        let sample_count = 48000*(length as u32/60);
        let mut result = Vec::new();

        for x in 0..sample_count {
            result.push(
                if (x / period) % 2 == 0 {
                    volume*self.volume_scale
                }
                else {
                    0
                }
            );
        }
        SquarePulse { pulse: result }
    }

    pub fn decaying(&self, rate: u16, wave_length: u16, length: u8) -> SquarePulse {
        let period = (((wave_length as u32 + 1)*768) / 1790) / 2;
        let sample_count = 48000*(length as u32/60);
        let mut result = Vec::new();
        let mut volume = 16i16;
        let tone_period = 48000 / (240/(rate as u32 + 1)); //decay

        for x in 0..sample_count {
            if x % tone_period == 0 && volume > 0 {
                volume -= 1;
            }
            result.push(
                if (x / period) % 2 == 0 {
                    volume*self.volume_scale
                }
                else {
                    0
                }
            );
        }
        SquarePulse { pulse: result }
    }
}


#[cfg(test)]
mod test {
    use super::{PulseGenerator, SquarePulse};

    #[test]
    fn simple_constant_square_wave() {
        let generator: PulseGenerator = PulseGenerator::new(1);
        let pulse = generator.constant_volume(10, 0x1AA, 0x7F);
        let mut expected_wave = Vec::new(); //gen_wave(Duration::from_millis(1), 262);
        push_all(&mut expected_wave, &[10; 91]);
        push_all(&mut expected_wave, &[0; 91]);
        push_all(&mut expected_wave, &[10; 91]);
        assert_eq!(pulse.pulse[0..91*3], expected_wave[0..91*3]);

        assert_eq!(pulse.pulse.len(), 48000*(127/60));
    }

    #[test]
    fn simple_decaying_square_wave() {
        let generator: PulseGenerator = PulseGenerator::new(1);
        let pulse = generator.decaying(4, 0x1AA, 0x7F);
        let mut expected_wave = Vec::new(); //gen_wave(Duration::from_millis(1), 262);
        push_all(&mut expected_wave, &[15; 91]);
        push_all(&mut expected_wave, &[0; 91]);
        push_all(&mut expected_wave, &[15; 91]);
        assert_eq!(pulse.pulse[0..91*3], expected_wave[0..91*3]);

        let mut decaying_wave = Vec::new();
        push_all(&mut decaying_wave, &[15; 1000-910]);
        push_all(&mut decaying_wave, &[14; 91-(1000-910)]);
        assert_eq!(pulse.pulse[910..910+91], decaying_wave[0..91]);

        assert_eq!(pulse.pulse.len(), 48000*(127/60));
    }


    fn push_all(vec: &mut Vec<i16>, slice: &[i16]) {
        for &d in slice.iter() {
            vec.push(d);
        }
    }
}
