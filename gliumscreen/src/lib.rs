#[macro_use]
extern crate glium;
extern crate nes;

use glium::Surface;
use glium::DisplayBuild;

use nes::ppu::screen::Color;
use nes::ppu::screen::Screen;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

const VERTEX_SHADER_SRC: &'static str = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 my_attr;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            my_attr = color;
        }
    "#;
const FRAGMENT_SHADER_SRC: &'static str = r#"
        #version 140

        out vec4 color;
        in vec3 my_attr;

        void main() {
            color = vec4(my_attr, 1.0);
        }
    "#;

struct Pixel {
    vertices: [Vertex; 4]
}

impl Pixel {
    fn new(left: f32, top: f32, size: f32, color: Color) -> Pixel {
        Pixel {
            vertices: [
                Vertex { position: [left, top], color: color},
                Vertex { position: [left, top-size], color: color},
                Vertex { position: [left+size, top-size], color: color},
                Vertex { position: [left+size, top], color: color},
            ]
        }
    }

    fn color(&self) -> Color {
        self.vertices[0].color
    }

    fn set_color(&mut self, vertex_buffer: &mut glium::VertexBuffer<Vertex>, start_index: usize, color: Color) {
        for index in 0..4 {
            self.vertices[index].color = color;
            vertex_buffer.map_write().set(start_index + index, self.vertices[index]);
        }
    }
}

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

pub struct GliumScreen {
    display: glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    pixels: Vec<Vec<Pixel>>,
}

impl GliumScreen {
    pub fn new(scale: u8) -> GliumScreen {
        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_dimensions(SCREEN_WIDTH*(scale as u32), SCREEN_HEIGHT*(scale as u32))
            .build_glium().unwrap();
        let pixel_size: f32 = 2.0 / (SCREEN_WIDTH as f32);

        let pixel_vec: Vec<Vec<Pixel>> = (0..SCREEN_HEIGHT)
            .map(|row| {
                let top = 1.0 - (row as f32 * pixel_size);
                (0..SCREEN_WIDTH)
                    .map(|col| {
                        Pixel::new((col as f32)*pixel_size - 1.0, top, pixel_size, BLACK)
                    })
                    .collect()
            })
            .collect();

        let shape: Vec<Vertex> = pixel_vec.iter()
            .flat_map(|p| p.iter())
            .flat_map(|p| p.vertices.iter())
            .map(|&v| v)
            .collect();
        let vertex_buffer = glium::VertexBuffer::dynamic(&display, &shape).unwrap();
        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        let mut indices: Vec<u32> = vec!();
        for index in 0..SCREEN_WIDTH*SCREEN_HEIGHT {
            let base = index*4;
            indices.push(base);
            indices.push(base+1);
            indices.push(base+3);
            indices.push(base+3);
            indices.push(base+1);
            indices.push(base+2);
        }

        let index_buffer = glium::index::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices
        ).unwrap();

        GliumScreen {
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            pixels: pixel_vec,
        }
    }

    pub fn get_color(&self, index: usize) -> Color {
        self.pixels[index][0].color()
    }

    pub fn poll_events(&self) -> glium::backend::glutin_backend::PollEventsIter {
        self.display.poll_events()
    }
}

impl Screen for GliumScreen {
    fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let top = y;
        let left = x;
        let start_index = top*(SCREEN_WIDTH as usize*4) + (left*4);
        self.pixels[y][x].set_color(&mut self.vertex_buffer, start_index, color);
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
    }

    fn get_row(&self, _: usize) -> &[Color] {
        unimplemented!()
    }
}
