use nes::sound;

pub struct AudioDevice {}

impl sound::AudioDevice for AudioDevice {
    fn play(&self, _: &[i16]) { }
}
