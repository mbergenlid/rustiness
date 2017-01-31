#[macro_use]
extern crate glium;
extern crate nes;

use glium::Surface;
use glium::DisplayBuild;

use nes::ppu::screen::Color;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    vertex_index: u32,
    text_coords: [f32; 2],
    texture_index: u32,
}

implement_vertex!(Vertex, position, vertex_index, text_coords, texture_index);

const VERTEX_SHADER_SRC: &'static str = r#"
        #version 410

        uniform mat4 scale;
        uniform mat4 translation;
        uniform mat4 scroll;
        in vec2 position;
        in uint vertex_index;
        in vec2 text_coords;
        in int texture_index;

        flat out int index;
        out vec2 v_text_coords;

        void main() {
            vec4 new_position = scroll * vec4(position, 0.0, 1.0);
            if(new_position.x >= 63) {
                new_position.x = new_position.x - 64;
            }
            if(new_position.y <= -29) {
                new_position.y = new_position.y + 30;
            }
            if(vertex_index == 0) {
                gl_Position = vec4(new_position.x, new_position.y, 1.0, 1.0);
            } else  if(vertex_index == 1) {
                gl_Position = vec4(new_position.x, new_position.y-1, 1.0, 1.0);
            } else if(vertex_index == 2) {
                gl_Position = vec4(new_position.x+1, new_position.y-1, 1.0, 1.0);
            } else if(vertex_index == 3) {
                gl_Position = vec4(new_position.x+1, new_position.y, 1.0, 1.0);
            }
            gl_Position = scale * translation * gl_Position;
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
    fn new(left: f32, top: f32, texture: u32) -> Pixel {
        Pixel {
            vertices: [
                Vertex { position: [left, top], vertex_index: 0, text_coords: [0.0, 0.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 1, text_coords: [0.0, 1.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 2, text_coords: [1.0, 1.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 3, text_coords: [1.0, 0.0], texture_index: texture},
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

const AREA_WIDTH_IN_TILES: u32 = 64;
const AREA_HEIGHT_IN_TILES: u32 = 60;


use nes::ppu::screen::{Screen, Tile, Pattern, COLOUR_PALETTE};

pub struct GliumScreen {
    scale: usize,
    x_offset: f32,
    y_offset: f32,
    display: glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    texture_buffer: Vec<glium::texture::Texture2dArray>,

    palettes: [[Color; 4]; 4],
    pixels: Vec<Vec<Pixel>>,
}

impl GliumScreen {
    pub fn new(scale: u8) -> GliumScreen {
        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .with_dimensions(SCREEN_WIDTH*(scale as u32), SCREEN_HEIGHT*(scale as u32))
            .build_glium().unwrap();

        GliumScreen::init_bottom_bar(&display);

        let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        let tiles: Vec<Vec<Pixel>> = (0..AREA_HEIGHT_IN_TILES)
            .map(|row| {
                let top = row; //if row >= 30 { row + 2 } else { row };
                (0..AREA_WIDTH_IN_TILES)
                    .map(|col| {
                        Pixel::new(col as f32, -(top as f32), 0)
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
            for index in 0..(AREA_HEIGHT_IN_TILES*AREA_WIDTH_IN_TILES) {
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
        GliumScreen {
            scale: scale as usize,
            x_offset: 0.0,
            y_offset: -30.0,
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture_buffer: texture_buffer,
            palettes: [[BLACK; 4]; 4],
            pixels: tiles,
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
        let top = y;
        let left = x;
        let start_index = top*(AREA_WIDTH_IN_TILES as usize*4) + (left*4);
        self.pixels[y][x].set_texture(&mut self.vertex_buffer, start_index, (tile.palette_index as u32)*512 + tile.pattern_index);
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
        self.x_offset = (x % 512) as f32 / 8.0;
        self.y_offset = -(((240 - y) % 480) as f32 / 8.0);
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform! {
                texture_1: &(self.texture_buffer[0]),
                texture_2: &(self.texture_buffer[1]),
                texture_3: &(self.texture_buffer[2]),
                texture_4: &(self.texture_buffer[3]),
                scale: [
                    [0.0625, 0.0, 0.0, 0.0],
                    [0.0, 0.0625, 0.0, 0.0],
                    [0.0, 0.0, 0.0625, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                translation: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [-16.0, 16.0, 0.0, 1.0f32],
                ],
                scroll: [
                    [1.0,           0.0, 0.0, 0.0],
                    [0.0,           1.0, 0.0, 0.0],
                    [0.0,           0.0, 1.0, 0.0],
                    [-self.x_offset, self.y_offset, 0.0, 1.0],
                ],
            },
            &glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: false,
                    .. Default::default()
                },
                .. Default::default()
            }
        ).unwrap();

        target.finish().unwrap();
    }
}