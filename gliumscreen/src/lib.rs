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
    text_coords: [f32; 2],
    texture_index: u32,
}

implement_vertex!(Vertex, position, text_coords, texture_index);

const VERTEX_SHADER_SRC: &'static str = r#"
        #version 410

        in vec2 position;
        in vec2 text_coords;
        in int texture_index;

        flat out int index;
        out vec2 v_text_coords;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            index = texture_index;
            v_text_coords = text_coords;
        }
    "#;
const FRAGMENT_SHADER_SRC: &'static str = r#"
        #version 140

        uniform sampler2DArray texture_1;
        uniform sampler2DArray texture_2;
        uniform sampler2DArray texture_3;
        uniform sampler2DArray texture_4;
        in vec2 v_text_coords;
        flat in int index;

        out vec4 color;

        void main() {
            int palette = index / 512;
            if(palette == 0) {
                color = texture(texture_1, vec3(v_text_coords, index % 512));
            } else if (palette == 1) {
                color = texture(texture_2, vec3(v_text_coords, index % 512));
            } else if (palette == 2) {
                color = texture(texture_3, vec3(v_text_coords, index % 512));
            } else {
                color = texture(texture_4, vec3(v_text_coords, index % 512));
            }
        }
    "#;

struct Pixel {
    vertices: [Vertex; 4]
}

impl Pixel {
    fn new(left: f32, top: f32, size: f32, texture: u32) -> Pixel {
        Pixel {
            vertices: [
                Vertex { position: [left, top], text_coords: [0.0, 1.0], texture_index: texture},
                Vertex { position: [left, top-size], text_coords: [0.0, 0.0], texture_index: texture},
                Vertex { position: [left+size, top-size], text_coords: [1.0, 0.0], texture_index: texture},
                Vertex { position: [left+size, top], text_coords: [1.0, 1.0], texture_index: texture},
            ]
        }
    }

    fn set_texture(&mut self, vertex_buffer: &mut glium::VertexBuffer<Vertex>, start_index: usize, texture: u32) {
        for index in 0..4 {
            self.vertices[index].texture_index = texture;
            vertex_buffer.map_write().set(start_index + index, self.vertices[index]);
        }
    }
}

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

use nes::ppu::screen::{Screen2, Tile, Pattern, COLOUR_PALETTE};

pub struct GliumScreen2 {
    display: glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    texture_buffer: Vec<glium::texture::Texture2dArray>,

    palettes: [[Color; 4]; 4],
    pixels: Vec<Vec<Pixel>>,
}

impl GliumScreen2 {
    pub fn new(scale: u8) -> GliumScreen2 {
        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_dimensions(SCREEN_WIDTH*(scale as u32), SCREEN_HEIGHT*(scale as u32))
            .build_glium().unwrap();

        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();


        let tile_size: f32 = 2.0 / (SCREEN_WIDTH as f32) * 8.0;
        let tiles: Vec<Vec<Pixel>> = (0..30)
            .map(|row| {
                let top = 1.0 - (row as f32 * tile_size);
                (0..32)
                    .map(|col| {
                        Pixel::new((col as f32)*tile_size - 1.0, top, tile_size, 0)
                    })
                    .collect()
            })
            .collect();
        let vertex_buffer = {
            let shape: Vec<Vertex> = tiles.iter()
                .flat_map(|p| p.iter())
                .flat_map(|p| p.vertices.iter())
                .map(|&v| v)
                .collect();
            glium::VertexBuffer::new(&display, &shape).unwrap()
        };

        let index_buffer = {
            let mut indices: Vec<u32> = vec!();
            for index in 0..(30*32) {
                let base = index*4;
                indices.push(base);
                indices.push(base+1);
                indices.push(base+3);
                indices.push(base+3);
                indices.push(base+1);
                indices.push(base+2);
            }
            glium::index::IndexBuffer::new(
                &display,
                glium::index::PrimitiveType::TrianglesList,
                &indices
            ).unwrap()
        };

        let image: Vec<Vec<(f32, f32, f32)>> = vec!(
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
            vec!((0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0),(0.0,0.0,0.0)),
        );

        let texture_buffer = vec!(
            glium::texture::Texture2dArray::new(&display, vec!(image.clone())).unwrap(),
            glium::texture::Texture2dArray::new(&display, vec!(image.clone())).unwrap(),
            glium::texture::Texture2dArray::new(&display, vec!(image.clone())).unwrap(),
            glium::texture::Texture2dArray::new(&display, vec!(image.clone())).unwrap(),
        );
        GliumScreen2 {
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture_buffer: texture_buffer,
            palettes: [[BLACK; 4]; 4],
            pixels: tiles,
        }
    }
}

impl Screen2 for GliumScreen2 {

    fn update_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        let top = y;
        let left = x;
        let start_index = top*(32 as usize*4) + (left*4);
        self.pixels[y][x].set_texture(&mut self.vertex_buffer, start_index, (tile.palette_index as u32)*512 + tile.pattern_index);
    }

    fn update_patterns(&mut self, patterns: &[Pattern]) {
        //Create an image for each palette
        for palette in 0..4 {

            let textures = patterns.iter().map(|pattern| {
                let image: Vec<Vec<(f32, f32, f32)>>  = pattern.data.iter().map(|row|
                    row.iter().map(|&pixel| {
                        let colour = self.palettes[palette][pixel as usize];
                        (colour[0], colour[1], colour[2])
                    }).collect()
                ).collect();
                image
            }).collect();

            self.texture_buffer[palette] = glium::texture::Texture2dArray::new(&self.display, textures).unwrap();
        }
    }

    fn set_universal_background(&mut self, background_value: u8) {
        self.palettes[0][0] = COLOUR_PALETTE[background_value as usize];
        self.palettes[1][0] = COLOUR_PALETTE[background_value as usize];
        self.palettes[2][0] = COLOUR_PALETTE[background_value as usize];
        self.palettes[3][0] = COLOUR_PALETTE[background_value as usize];
    }
    fn update_palette_0(&mut self, index: u8, palette_value: u8) {
        self.palettes[0][index as usize] = COLOUR_PALETTE[palette_value as usize];
    }
    fn update_palette_1(&mut self, index: u8, palette_value: u8) {
        self.palettes[1][index as usize] = COLOUR_PALETTE[palette_value as usize];
    }
    fn update_palette_2(&mut self, index: u8, palette_value: u8) {
        self.palettes[2][index as usize] = COLOUR_PALETTE[palette_value as usize];
    }
    fn update_palette_3(&mut self, index: u8, palette_value: u8) {
        self.palettes[3][index as usize] = COLOUR_PALETTE[palette_value as usize];
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();
//        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform! {
                texture_1: &(self.texture_buffer[0]),
                texture_2: &(self.texture_buffer[1]),
                texture_3: &(self.texture_buffer[2]),
                texture_4: &(self.texture_buffer[3]),
            },
            &Default::default()
        ).unwrap();

        target.finish().unwrap();
    }
}
