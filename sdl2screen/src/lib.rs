extern crate sdl2;
extern crate nes;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use nes::ppu::screen::{Screen, PixelBuffer};

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct SDL2Screen<'a> {
    scale: usize,

    renderer: Renderer<'a>,
    texture: Texture,
}

impl <'a> SDL2Screen<'a> {
    pub fn new(scale: u8) -> SDL2Screen<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = 256*(scale as u32);
        let height = 240*(scale as u32);
        let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();
        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, width, height).unwrap();

        let scale = scale as usize;

        SDL2Screen {
            scale: scale,
            renderer: renderer,
            texture: texture,
        }
    }
}



impl <'a> Screen for SDL2Screen<'a> {
    fn draw<T>(&mut self, func: T) where Self: Sized, T: FnOnce(&mut PixelBuffer) {
        let width = SCREEN_WIDTH*self.scale;
        let height = SCREEN_HEIGHT*self.scale;
        let scale = self.scale as u8;
        self.texture.with_lock(
            None,
            |buf, pitch| func(&mut PixelBuffer { buffer: buf, pitch: pitch, scale: scale})
        ).unwrap();

        self.renderer.clear();
        self.renderer.copy(&self.texture, None, Some(Rect::new(0, 0, width as u32, height as u32))).unwrap();

        self.renderer.present();
    }

}
