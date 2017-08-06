use nes::ppu::screen::{Screen, Color, Rectangle, PixelBuffer};

pub struct NoScreen(pub ());
impl Screen for NoScreen {
    fn draw<T>(&mut self, _: T) where Self: Sized, T: FnOnce(&mut PixelBuffer) {}

    fn set_backdrop_color(&mut self, _: Color) {}

    fn upload_buffer(&mut self, _: Option<Rectangle>, _: &[u8], _: usize) {}
    fn update_buffer<T>(&mut self, _: T) where T: FnOnce(&mut PixelBuffer) {}
    fn render(&mut self, _: Rectangle, _: usize, _: usize) {}

    fn render_sprite(&mut self, _: Rectangle, _: usize, _: usize, _: bool, _: bool) {}
    fn update_sprites<T>(&mut self, _: T) where T: FnOnce(&mut PixelBuffer) {}

    fn present(&mut self) {}
}
