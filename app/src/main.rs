#![feature(box_syntax)]
#[macro_use]
extern crate nes;
extern crate sdl2;

mod debugger;
mod sdlscreentest;
mod soundtest;
mod instruction_benchmark;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} COMMAND", args[0]);
    }

    match args[1].trim() {
        "debug" => debugger::start(),
        "sdl2screen" => sdlscreentest::start(),
        "sound" => soundtest::start(),
        "bench" => instruction_benchmark::run(&args[2..]),
        _ => panic!("Unknown command {}", args[1]),
    }
}

//fn performance_test() {
//    use std::time::Instant;
//
//    let mut memory = external_memory!(
//            0x00A5 => 0xF0,
//            0x00A6 => 0x10,
//            //ADC $05
//            0x8000 => 0x69,
//            0x8001 => 0x05,
//            0x8002 => 0x10
//        );
//
//    let mut nes = nes::NES::new();
//
//    let start = Instant::now();
//    nes.execute(&mut memory);
//
//    //One cycle: 500 ns,
//    println!("Took {} ns", start.elapsed().subsec_nanos());
//}
