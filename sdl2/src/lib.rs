extern crate sdl2;
extern crate nes;

pub mod standard_controller;

use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, BlendMode};

use nes::ppu::screen::{Screen, PixelBuffer, SpriteBuffer, Rectangle};
use standard_controller::SdlEvents;
use std::rc::Rc;
use std::cell::RefCell;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

pub struct SDL2Screen<'a> {
    scale: usize,

    renderer: Renderer<'a>,
    texture: Texture,
    sprite_texture: Texture,

    sdl_context: Sdl,
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
            PixelFormatEnum::RGB24, width*2, height*2+16*(scale as u32)).unwrap();

        let mut sprite_texture = renderer.create_texture_streaming(
            PixelFormatEnum::ARGB8888, 64*8*(scale as u32), 8*(scale as u32)
        ).unwrap();

        sprite_texture.set_blend_mode(BlendMode::Blend);

        let scale = scale as usize;

        SDL2Screen {
            scale: scale,
            renderer: renderer,
            texture: texture,
            sprite_texture: sprite_texture,

            sdl_context: sdl_context,
        }
    }

    pub fn event_pump(&self) -> SdlEvents {
       SdlEvents(Rc::new(RefCell::new(self.sdl_context.event_pump().unwrap())))
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
        self.texture.update(rect.map(|r| Rect::new(r.x, r.y, r.width, r.height)), buffer, pitch).unwrap();
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

    fn update_sprites<T>(&mut self, func: T) where T: FnOnce(&mut SpriteBuffer) {
        let scale = self.scale as u8;
        self.sprite_texture.with_lock(
            None,
            |buf, pitch| func(&mut SpriteBuffer { buffer: buf, pitch: pitch, scale: scale})
        ).unwrap();
    }

    fn render_sprite(&mut self, src: Rectangle, dst_x: usize, dst_y: usize, flip_horizontal: bool, flip_vertical: bool) {
        let scale_u32 = self.scale as u32;
        let scale_i32 = self.scale as i32;
        self.renderer.copy_ex(
            &self.sprite_texture,
            Some(Rect::new(src.x*scale_i32, src.y*scale_i32, src.width*scale_u32, src.height*scale_u32)),
            Some(Rect::new((dst_x*self.scale) as i32, (dst_y*self.scale) as i32, src.width*scale_u32, src.height*scale_u32)),
            0.0, /* angle */
            None, /* center */
            flip_horizontal,
            flip_vertical,
        ).unwrap();
    }

    fn present(&mut self) {
        self.renderer.present();
    }
}
