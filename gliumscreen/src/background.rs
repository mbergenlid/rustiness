use glium;
use glium::Surface;
use nes::ppu::screen::Tile;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    vertex_index: u32,
    text_coords: [f32; 2],
    texture_index: u32,
}

implement_vertex!(Vertex, position, vertex_index, text_coords, texture_index);

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

    fn set_texture(&mut self, texture: u32) {
        for index in 0..4 {
            self.vertices[index].texture_index = texture;
        }
    }
}

pub struct Background {
    x_offset: f32,
    y_offset: f32,
    program: glium::Program,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    pixels: Vec<Vec<Pixel>>,
}

const AREA_WIDTH_IN_TILES: u32 = 64;
const AREA_HEIGHT_IN_TILES: u32 = 60;

impl Background {
    pub fn new(display: &glium::Display) -> Background {
        let program = glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

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
            glium::VertexBuffer::new(display, &shape).unwrap()
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
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices
            ).unwrap()
        };

        Background {
            x_offset: 0.0,
            y_offset: -30.0,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            pixels: tiles,
        }
    }

    pub fn set_offset(&mut self, x: usize, y: usize) {
        self.x_offset = (x % 512) as f32 / 8.0;
        self.y_offset = -(((240 - y) % 480) as f32 / 8.0);
    }

    pub fn update_tile(&mut self, x: usize, y: usize, tile: &Tile) {
        self.pixels[y][x].set_texture((tile.palette_index as u32)*512 + tile.pattern_index);
    }

    pub fn upload_data(&mut self, display: &mut glium::Display) {
        self.vertex_buffer = {
            let shape: Vec<Vertex> = self.pixels.iter()
                .flat_map(|p| p.iter())
                .flat_map(|p| p.vertices.iter())
                .map(|&v| v)
                .collect();
            glium::VertexBuffer::new(display, &shape).unwrap()
        };
    }

    pub fn draw(&self, target: &mut glium::Frame, texture_buffer: &[glium::texture::Texture2dArray]) {

        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniform! {
                texture_1: &(texture_buffer[0]),
                texture_2: &(texture_buffer[1]),
                texture_3: &(texture_buffer[2]),
                texture_4: &(texture_buffer[3]),
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
    }
}

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