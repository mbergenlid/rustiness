use nes::borrow::MutableRef;
use nes::input::standard_controller::StandardController;
use nes_sdl2::SDL2;
use std::env;

use nes::NES;
pub fn start() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} run FILE", args[0]);
    }

    let file = &args[2];
    let sdl = SDL2::new();

    let source = sdl.event_pump();
    let mut standard_controller = StandardController::new(&source);

    let screen = box sdl.screen(2);
    let nes = nes::NES::from_file(
        file,
        MutableRef::Borrowed(&mut standard_controller),
        sdl.audio(),
        screen,
    );

    run(nes, &source);
}
use nes_sdl2::SDL2Screen;

fn run<'a>(
    mut nes: NES<'a, SDL2Screen, nes_sdl2::SDLAudioDevice>,
    source: &nes_sdl2::standard_controller::SdlEvents,
) {
    let mut counter = 0;
    loop {
        nes.execute();
        counter += 1;
        if counter > 0x100_000 {
            if source.should_exit() {
                return;
            }
            counter = 0;
        }
    }
}
