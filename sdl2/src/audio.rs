use nes::sound::AudioDevice;
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::Sdl;

pub struct SDLAudioDevice {
    audio_queue: AudioQueue<i16>,
}

pub fn new_audio_device(sdl_context: &Sdl) -> SDLAudioDevice {
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        // mono  -
        samples: None, // default sample size
    };

    let device = audio_subsystem
        .open_queue::<i16>(None, &desired_spec)
        .unwrap();
    println!("Spec {:?}", device.spec());
    SDLAudioDevice {
        audio_queue: device,
    }
}

impl AudioDevice for SDLAudioDevice {
    fn play(&self, pulse: &[i16]) {
        self.audio_queue.queue(pulse);
        self.audio_queue.resume()
    }
}
