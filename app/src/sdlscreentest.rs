use nes::ppu::screen::{Pattern, Screen};
use sdl2screen::SDL2Screen;

use std;
use std::time::Duration;


pub fn start() {
    let mut screen = SDL2Screen::new(2);

    let patterns = vec!(
        Pattern {
            data: [
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,1,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,1,1,1,1,1,1,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,0,0,0,0,1,1,0],
                [0,0,0,0,1,1,0,0],
                [0,0,1,1,0,0,0,0],
                [0,1,1,0,0,0,0,0],
                [0,1,1,1,1,1,1,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,0,0,0,0,1,1,0],
                [0,0,0,1,1,1,0,0],
                [0,0,0,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,0,0,0,1,1,0],
                [0,0,0,0,1,1,1,0],
                [0,0,0,1,1,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,1,1,1,1,1],
                [0,0,0,0,0,1,1,0],
                [0,0,0,0,0,1,1,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,1,1,1,1,1,1,0],
                [0,1,1,0,0,0,0,0],
                [0,1,1,1,1,1,0,0],
                [0,0,0,0,0,1,1,0],
                [0,0,0,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,0,0,0],
                [0,1,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,1,1,1,1,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,0,0,1,1,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,1,1,0,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,1,0],
                [0,0,0,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
        Pattern {
            data: [
                [0,0,1,1,1,1,0,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,1,1,1,0],
                [0,1,1,1,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,1,1,0,0,1,1,0],
                [0,0,1,1,1,1,0,0],
                [0,0,0,0,0,0,0,0],
            ]
        },
    );


    let sprite = [
        [1,1,1,1,1,1,1,1],
        [1,1,1,1,1,1,1,1],
        [1,1,0,0,0,0,1,1],
        [1,1,0,0,0,0,1,1],
        [1,1,0,0,0,0,1,1],
        [1,1,0,0,0,0,1,1],
        [1,1,1,1,1,1,1,1],
        [1,1,1,1,1,1,1,1],
    ];
    std::thread::sleep(Duration::from_millis(2000));
    use std::time::Instant;

    screen.update_buffer(|buffer| {
        for tile_y in 0..30 {
            let mut tile_x = 0;
            for tile in 0..32 {
                let pixels = patterns[(tile % 10) + 1].data;
                for row in 0..8 {
                    for col in 0..8 {
                        let colour = pixels[row][col]*255;
                        buffer.set_pixel(col+tile_x, row+(tile_y*8), (colour, colour, colour));
                    }
                }
                tile_x += 8;
            }
        }
    });

    screen.update_sprites(|buffer| {
        for y in 0..8 {
            for x in 0..8 {
                let colour = sprite[y][x]*255;
                if colour == 0 {
                    buffer.set_pixel(x, y, (0, 0, 0, 0));
                } else {
                    buffer.set_pixel(x, y, (255, 0, colour, 0));
                }
            }
        }
    });
    use nes::ppu::screen::Rectangle;
    screen.render(
        Rectangle { x: 0, y: 0, width: 8*2, height: 240*2 },
        512-16, 0
    );
    screen.present();

    
    let start = Instant::now();
    let mut sprite_x = 0;
    for i in 0..200 {
        let u = i as u32;
        screen.update_buffer(|buffer| {
            let pixels = patterns[(u as usize % 10) + 1].data;
            for tile in 0..32 {
                for row in 0..8 {
                    for col in 0..8 {
                        let colour = pixels[row][col] * 255;
                        buffer.set_pixel(col + (tile*8), row + (17 * 8), (colour, colour, colour));
                    }
                }
            }
        });
        screen.render(
            Rectangle { x: i, y: 0, width: (256-u), height: 120 },
            0,0
        );
        screen.render(
            Rectangle { x: 0, y: 0, width: u, height: 120 },
            256- i as usize, 0
        );
        screen.render(
            Rectangle { x: 0, y: 120, width: 256, height: 240 },
            0, 120
        );

        screen.render_sprite(
            Rectangle { x: 0, y: 0, width: 8, height: 8 },
            sprite_x, 8
        );
        sprite_x += 1;
        screen.present();
        std::thread::sleep(Duration::from_millis(50));
    }

    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());

}
