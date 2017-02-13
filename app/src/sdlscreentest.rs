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


    std::thread::sleep(Duration::from_millis(2000));
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..30 {

        screen.draw(|buffer| {
            let mut tile_x = 0;
            for tile_y in 0..28 {
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
    }
    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());

    use nes::ppu::screen::PixelBuffer;
    let mut buffer1 = [0; 256*2*3*240*2];
    let mut pixel_buffer = PixelBuffer { buffer: &mut buffer1, pitch: 256*2*3, scale: 2};
    let start = Instant::now();
    for i in 0..30 {
        for tile_y in 0..30 {
            let mut tile_x = 0;
            for tile in 0..32 {
                let pixels = patterns[(tile % 10) + 1].data;
                for row in 0..8 {
                    for col in 0..8 {
                        let colour = pixels[row][col]*255;
                        pixel_buffer.set_pixel(col+tile_x, row+(tile_y*8), (colour, colour, colour));
                    }
                }
                tile_x += 8;
            }
        }
        screen.draw2(&pixel_buffer.buffer, 256*2*3);
//        std::thread::sleep(Duration::from_millis(500));
    }
    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());

}