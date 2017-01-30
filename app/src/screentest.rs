use nes::ppu::screen::{Screen, Pattern, Tile};
use gliumscreen::GliumScreen;

use std;
use std::time::Duration;


pub fn start() {
    let mut screen = GliumScreen::new(2);

    screen.set_universal_background(0x3F);
    screen.update_palette(0, 1, 0x00);
    screen.update_palette(0, 2, 0x00);
    screen.update_palette(0, 3, 0x20);

    screen.update_palette(1, 1, 0x00);
    screen.update_palette(1, 2, 0x00);
    screen.update_palette(1, 3, 0x15);

    let mut patterns = vec!(
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
    for _ in 0..509 {
        patterns.push(Pattern {
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
        });
    }

    screen.update_patterns(&patterns);


    for i in 0..30 {
        screen.update_tile(0, i, &Tile { pattern_index: (i % 10) as u32+1, palette_index: 0 });
//        screen.update_tile(32, i, &Tile { pattern_index: (i % 10) as u32+1, palette_index: 0 });
    }

//    for i in 30..60 {
//        screen.update_tile(0, i, &Tile { pattern_index: (i % 10) as u32+1, palette_index: 0 });
////        screen.update_tile(32, i, &Tile { pattern_index: (i % 10) as u32+1, palette_index: 0 });
//    }

    screen.set_background_offset(10, 10);
    let mut y_scroll = 0;
    let mut x_scroll = 0;
    screen.draw();
    loop {

        std::thread::sleep(Duration::from_millis(50));

//        y_scroll += 1;
//        x_scroll += 1;
//        screen.set_background_offset(x_scroll, y_scroll);
//        screen.draw();
    }
//    screen.draw();
//    for row in 0..32 {
//        for col in 0..30 {
//            screen.draw();
//
//            screen.update_tile(col, row, &Tile { pattern_index: 1, palette_index: 0});
//            std::thread::sleep(Duration::from_millis(50));
//        }
//    }
}