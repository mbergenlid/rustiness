extern crate sdl2;
extern crate nes;

pub mod standard_controller;

pub use self::screen::SDL2Screen;
pub use self::audio::SDLAudioDevice;
pub mod screen;
pub mod audio;

use sdl2::Sdl;

use standard_controller::SdlEvents;
use std::rc::Rc;
use std::cell::RefCell;


pub struct SDL2(Sdl);

impl SDL2 {
    pub fn new() -> SDL2 {
        SDL2(sdl2::init().unwrap())
    }

    pub fn event_pump(&self) -> SdlEvents {
       SdlEvents(Rc::new(RefCell::new(self.0.event_pump().unwrap())))
    }

    pub fn screen(&self, scale: u8) -> SDL2Screen {
        SDL2Screen::new(&self.0, scale)
    }

    pub fn audio(&self) -> SDLAudioDevice {
        audio::new_audio_device(&self.0)
    }
}
