#[macro_use]
extern crate glium;
extern crate nes;

mod background;

use glium::Surface;
use glium::DisplayBuild;

use nes::ppu::screen::Color;

const BLACK: [f32; 3] = [0.0, 0.0, 0.0];

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;


use nes::ppu::screen::{Screen, Tile, Pattern, COLOUR_PALETTE};
use background::Background;

pub struct GliumScreen {
    scale: usize,
    display: glium::Display,
    texture_buffer: Vec<glium::texture::Texture2dArray>,
    palettes: [[Color; 4]; 4],

    background: Background,
}

impl GliumScreen {
    pub fn new(scale: u8) -> GliumScreen {
        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .with_dimensions(SCREEN_WIDTH*(scale as u32), SCREEN_HEIGHT*(scale as u32))
            .build_glium().unwrap();

        GliumScreen::init_bottom_bar(&display);

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
        let background = Background::new(&display);
        GliumScreen {
            scale: scale as usize,
            display: display,
            texture_buffer: texture_buffer,
            palettes: [[BLACK; 4]; 4],
            background: background,
        }
    }

    fn init_bottom_bar(display: &glium::Display) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        #[derive(Copy, Clone, Debug)]
        struct Vertex {
            position: [f32; 2],
        }
        implement_vertex!(Vertex, position);
        let vertex_buffer = {
            let shape = vec![
                Vertex { position: [-1.0, -0.875] },
                Vertex { position: [-1.0, -1.0] },
                Vertex { position: [1.0, -1.0] },

                Vertex { position: [-1.0, -0.875] },
                Vertex { position: [1.0,  -1.0] },
                Vertex { position: [1.0, -0.875] },
            ];
            glium::VertexBuffer::new(display, &shape).unwrap()
        };
       let vertex_shader_src = r#"
            #version 410

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.0, 0.0, 1.0, 1.0);
            }
        "#;
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        //Draw the bottom area into both buffers.
        for _ in 0..2 {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            target.draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                }
            ).unwrap();

            target.finish().unwrap();
        }
    }
}

impl Screen for GliumScreen {

    fn update_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        self.background.update_tile(x, y, tile);
    }

    fn update_patterns(&mut self, patterns: &[Pattern]) {
        //Create an image for each palette
        for palette in 0..4 {

            let textures: Vec<Vec<Vec<(f32, f32, f32)>>> = patterns.iter().map(|pattern| {
                let mut image: Vec<Vec<(f32, f32, f32)>> = vec!();
                for row in 0..(8*self.scale) {
                    let mut row_vec = vec!();
                    for col in 0..(8*self.scale) {
                        let pixel = pattern.data[row/self.scale][col/self.scale];
                        let colour = self.palettes[palette][pixel as usize];
                        row_vec.push((colour[0], colour[1], colour[2]));
                    }
                    image.push(row_vec);
                }
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
    fn update_palette(&mut self, palette: u8, index: u8, palette_value: u8) {
        self.palettes[palette as usize][index as usize] = COLOUR_PALETTE[palette_value as usize];
    }

    fn set_background_offset(&mut self, x: usize, y: usize) {
        self.background.set_offset(x, y);
    }

    fn draw(&mut self) {
        self.background.upload_data(&mut self.display);
        let mut target: glium::Frame = self.display.draw();

        self.background.draw(&mut target, self.texture_buffer.as_ref());

        target.finish().unwrap();
    }
}