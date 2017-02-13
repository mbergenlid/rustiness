extern crate sdl2;
extern crate nes;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use nes::ppu::screen::{Screen, PixelBuffer, Rectangle};

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
            PixelFormatEnum::RGB24, width*2, height*2).unwrap();

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

    fn upload_buffer(&mut self, rect: Option<Rectangle>, buffer: &[u8], pitch: usize) {
        self.texture.update(rect.map(|r| Rect::new(r.x, r.y, r.width, r.height)), buffer, pitch);
    }

    fn update_buffer<T>(&mut self, func: T) where T: FnOnce(&mut PixelBuffer) {
        let scale = self.scale as u8;
        self.texture.with_lock(
            None,
            |buf, pitch| func(&mut PixelBuffer { buffer: buf, pitch: pitch, scale: scale})
        ).unwrap();
    }

    fn render(&mut self, src: Rectangle, dst_x: usize, dst_y: usize) {
        let scale_u32 = self.scale as u32;
        let scale_i32 = self.scale as i32;
        self.renderer.copy(
            &self.texture,
            Some(Rect::new(src.x*scale_i32, src.y*scale_i32, src.width*scale_u32, src.height*scale_u32)),
            Some(Rect::new((dst_x*self.scale) as i32, (dst_y*self.scale) as i32, src.width*scale_u32, src.height*scale_u32)),
        ).unwrap();
    }

    fn present(&mut self) {
        self.renderer.present();
    }
}
