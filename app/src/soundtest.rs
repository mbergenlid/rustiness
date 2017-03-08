use sdl2::audio::SDLAudioDevice;
use sdl2::SDL2;

use std::thread::sleep;
use std::time::Duration;

fn gen_wave(duration: Duration, frequency: u32) -> Vec<i16> {
    // Generate a square wave
    let volume_scale = 500;
    let mut tone_volume = 15i16;
    let period = 48000 / (frequency as u64 * 2);  // (samples / sec) / (x / sec) => samples / x
    let sample_count = 48 * (duration.as_secs()*1000 + (duration.subsec_nanos()/1000_000) as u64);
    let mut result = Vec::new();
    let tone_period = 48000 / 48; //decay

    for x in 0..sample_count {
        if x % tone_period == 0 && tone_volume > 0 {
            tone_volume -= 1;
        }
        result.push(
            if (x / period) % 2 == 0 {
                tone_volume*volume_scale
            }
            else {
                0
            }
        );
    }
    result
}

use nes::sound::{AudioDevice, PulseGenerator};

pub fn start() {
    let sdl = SDL2::new();
    let audio = sdl.audio();
    let generator = PulseGenerator::new(500);

    for _ in 0..10 {
        audio.play(&generator.decaying(4, 0x1AA, 0x7E));
        sleep(Duration::from_millis(500));
        audio.play(&generator.decaying(4, 0x06A, 0x7E));
        sleep(Duration::from_millis(500));
    }
}
