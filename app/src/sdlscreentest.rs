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
//    for _ in 0..30 {
//
//        screen.draw(|buffer| {
//            let mut tile_x = 0;
//            for tile_y in 0..28 {
//                for tile in 0..32 {
//                    let pixels = patterns[(tile % 10) + 1].data;
//                    for row in 0..8 {
//                        for col in 0..8 {
//                            let colour = pixels[row][col]*255;
//                            buffer.set_pixel(col+tile_x, row+(tile_y*8), (colour, colour, colour));
//                        }
//                    }
//                    tile_x += 8;
//                }
//            }
//         });
//    }
    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());

    use nes::ppu::screen::PixelBuffer;
//    let mut buffer1 = [0; 256*2*3*240*2];
//    let mut pixel_buffer = PixelBuffer { buffer: &mut buffer1, pitch: 256*2*3, scale: 2};
//    let start = Instant::now();
//    for i in 0..30 {
//        for tile_y in 0..30 {
//            let mut tile_x = 0;
//            for tile in 0..32 {
//                let pixels = patterns[(tile % 10) + 1].data;
//                for row in 0..8 {
//                    for col in 0..8 {
//                        let colour = pixels[row][col]*255;
//                        pixel_buffer.set_pixel(col+tile_x, row+(tile_y*8), (colour, colour, colour));
//                    }
//                }
//                tile_x += 8;
//            }
//        }
//        screen.draw2(&pixel_buffer.buffer, 256*2*3);
////        std::thread::sleep(Duration::from_millis(500));
//    }
    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());


    let mut buffer1 = [0; 256*2*3*240*2];
    let mut pixel_buffer1 = PixelBuffer { buffer: &mut buffer1, pitch: 256*2*3, scale: 2};
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
//    let mut buffer2 = [0; 256*2*3*240*2];
//    let mut pixel_buffer2 = PixelBuffer { buffer: &mut buffer2, pitch: 256*2*3, scale: 2};

    use nes::ppu::screen::Rectangle;
//    screen.update(None, &pixel_buffer1.buffer, 512*2*3);
//    screen.copy(
//        Some(Rectangle { x: 16, y: 0, width: (256-8)*2, height: 240*2 }),
//        Some(Rectangle { x: 0, y: 0, width: (256-8)*2, height: 240*2 }),
//    );
    screen.render(
        Rectangle { x: 0, y: 0, width: 8*2, height: 240*2 },
        512-16, 0
    );
    screen.present();

    let start = Instant::now();
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
//        screen.update(None, &pixel_buffer1.buffer, 512*2*3);
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

        screen.present();
//        std::thread::sleep(Duration::from_millis(50));
    }

    let elapsed = start.elapsed();
    println!("Execution time: {}.{:09}s", elapsed.as_secs(), elapsed.subsec_nanos());
//    screen.draw4(Some(Rectangle { x: 128, y: 0, width: 256*2, height: 240*2 }));

}
