use glium;
use glium::Surface;
use nes::ppu::screen;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    texture_cords: [f32; 2],
    texture_index: u32,
    position: [f32; 2],
    vertex_index: u32,
}

implement_vertex!(Vertex, texture_cords, texture_index, position, vertex_index);

struct Sprite {
    vertices: [Vertex; 4]
}

impl Sprite {
    fn new(left: f32, top: f32, texture: u32) -> Sprite {
        Sprite {
            vertices: [
                Vertex { position: [left, top], vertex_index: 0, texture_cords: [0.0, 0.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 1, texture_cords: [0.0, 1.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 2, texture_cords: [1.0, 1.0], texture_index: texture},
                Vertex { position: [left, top], vertex_index: 3, texture_cords: [1.0, 0.0], texture_index: texture},
            ]
        }
    }

    fn set_texture(&mut self, texture: u32) {
        for index in 0..4 {
            self.vertices[index].texture_index = texture;
        }
    }

    fn update(&mut self, sprite: &screen::Sprite) {
        for index in 0..4 {
            self.vertices[index].texture_index =
                (sprite.palette_index as u32)*512 + sprite.pattern_index;
        }
    }
}

pub struct Sprites {
    program: glium::Program,

    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
    sprites: Vec<Sprite>,
}

impl Sprites {
    pub fn new(display: &glium::Display) -> Sprites {
        let program = glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

        let sprites: Vec<Sprite> = (0..64)
            .map(|sprite_index| {
                Sprite::new(0.0, 0.0, 0)
            })
            .collect();
        let vertex_buffer = {
            let shape: Vec<Vertex> = sprites.iter()
                .flat_map(|sprite| sprite.vertices.iter())
                .map(|&v| v)
                .collect();
            glium::VertexBuffer::new(display, &shape).unwrap()
        };


        let index_buffer = {
            let mut indices: Vec<u32> = vec!();
            for index in 0..63 {
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

        Sprites {
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            sprites: sprites,
        }
    }

    pub fn update_sprite(&mut self, index: usize, sprite: &screen::Sprite) {
        self.sprites[index].update(sprite);
    }

    pub fn upload_data(&mut self, display: &mut glium::Display) {
        self.vertex_buffer = {
            let shape: Vec<Vertex> = self.sprites.iter()
                .flat_map(|sprite| sprite.vertices.iter())
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
        in vec2 texture_cords;
        in int texture_index;
        in vec2 position;
        in int vertex_index;

        flat out int index;
        out vec2 v_text_coords;

        void main() {
            if(vertex_index == 0) {
                gl_Position = vec4(position.x, position.y, 1.0, 1.0);
            } else  if(vertex_index == 1) {
                gl_Position = vec4(position.x, position.y-1, 1.0, 1.0);
            } else if(vertex_index == 2) {
                gl_Position = vec4(position.x+1, position.y-1, 1.0, 1.0);
            } else if(vertex_index == 3) {
                gl_Position = vec4(position.x+1, position.y, 1.0, 1.0);
            }
            gl_Position = scale * translation * gl_Position;
            index = texture_index;
            v_text_coords = texture_cords;
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