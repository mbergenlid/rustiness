extern crate image;
extern crate nes;
extern crate nes_sdl2;

mod breakpoint;
mod command;
mod fakecontroller;
mod opcodes;
mod screen;
use self::fakecontroller::FakeController;
use self::opcodes::OpCodes;
use nes::input::standard_controller::StandardController;
use nes::memory::Memory;
use nes::ppu::attributetable;
use nes::ppu::screen::{Screen, ScreenMock, COLOUR_PALETTE};
use nes::ppu::sprite::Sprite;
use nes::sound::AudioDevice;
use nes::NES;
use nes_sdl2::standard_controller::SdlEvents;
use nes_sdl2::SDL2;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
use std::string::String;

use self::command::Command;
use nes::borrow::MutableRef;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} debug FILE", args[0]);
    }

    let file = &args[2];
    let sdl = SDL2::new();

    let fake_controller: Option<FakeController> = args
        .iter()
        .find(|&a| a == "-c")
        .map(|_| FakeController::new());
    let source = sdl.event_pump();
    let mut standard_controller = fake_controller
        .as_ref()
        .map(|c| StandardController::new(c))
        .unwrap_or_else(|| StandardController::new(&source));
    let use_screen_mock = args
        .iter()
        .find(|&a| a == "-g")
        .map(|_| true)
        .unwrap_or(false);

    if use_screen_mock {
        let screen = box screen::NoScreen(());
        let nes = nes::NES::from_file(
            file,
            MutableRef::Borrowed(&mut standard_controller),
            sdl.audio(),
            screen,
        );

        run(nes, &source, &fake_controller);
    } else {
        let screen = box sdl.screen(2);
        let nes = nes::NES::from_file(
            file,
            MutableRef::Borrowed(&mut standard_controller),
            sdl.audio(),
            screen,
        );

        run(nes, &source, &fake_controller);
    }
}

use self::breakpoint::BreakPoint;
extern crate chrono;
use self::chrono::prelude::*;

fn open_log_file() -> File {
    let now: DateTime<Local> = Local::now();
    let file_name = format!("/tmp/rustiness.{:?}.log", now);
    return File::create(file_name).unwrap();
}

use nes::cpu::*;

#[inline]
fn log<'a, S, A>(log_file: &Option<File>, nes: &NES<'a, S, A>, opcodes: Rc<OpCodes>)
where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    for mut file in log_file.iter() {
        let flags: String = [
            (NEGATIVE_FLAG, 'N'),
            (OVERFLOW_FLAG, 'O'),
            (DECIMAL_FLAG, 'D'),
            (INTERRUPT_DISABLE_FLAG, 'I'),
            (ZERO_FLAG, 'Z'),
            (CARRY_FLAG, 'C'),
        ]
        .iter()
        .filter(|&&(f, _)| nes.cpu.is_flag_set(f))
        .map(|&(_, f)| f)
        .collect();
        file.write_fmt(format_args!(
            "Cycle: {} - {} - A: {}, X: {}, Y: {}, Flags: {}\n",
            nes.cycle_count,
            next_instruction_as_string(&nes, opcodes.clone()),
            nes.cpu.accumulator(),
            nes.cpu.register_x(),
            nes.cpu.register_y(),
            flags
        ))
        .unwrap();
    }
}

fn run<'a, S, A>(
    mut nes: NES<'a, S, A>,
    source: &SdlEvents,
    fake_controller: &Option<FakeController>,
) where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    let mut break_points: Vec<Box<BreakPoint>> = vec![];
    let mut log_file: Option<File> = None;
    let opcodes = Rc::new(OpCodes::new());
    print(&nes);
    print_next_instruction(&nes, opcodes.clone());
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let cmd: Command = read_input();
        match cmd.name() {
            "c" => {
                let cycles: u64 = cmd.arg(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let end_cycle = nes.cycle_count + cycles;
                println!("Running to cycle {}", end_cycle);
                nes.resume();
                let mut should_exit = false;
                let mut counter = 0;
                while (cycles == 0 || nes.cycle_count < end_cycle) && !should_exit {
                    log(&log_file, &nes, opcodes.clone());
                    nes.execute();
                    counter += 1;
                    if counter > 0x100_000 {
                        should_exit = source.should_exit();
                        counter = 0;
                    }
                    should_exit = should_exit || break_points.breakpoint(&nes.cpu, &nes.ppu.borrow(), &nes.memory);
                }
                println!("Clock {}", nes.clock);
                print(&nes);
                print_next_instruction(&nes, opcodes.clone());
            },
            "n" => {
                let arg: u32 = cmd.arg(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
                if arg > 1 {
                    for _ in 0..(arg) {
                        log(&log_file, &nes, opcodes.clone());
                        nes.execute();
                        println!("Cycle count: {}", nes.cycle_count);
                    }
                } else {
                    log(&log_file, &nes, opcodes.clone());
                    nes.execute();
                }
                print(&nes);
                print_next_instruction(&nes, opcodes.clone());
            },
            "break" => {
                match BreakPoint::parse(cmd.args(), opcodes.clone()) {
                    Ok(bp) => {
                        break_points.push(bp);
                    },
                    Err(f) => println!("{:?}", f),
                }
            },
            "set-pc" => {
                match cmd.hex_arg(1) {
                    Some(address) => {
                        nes.cpu.set_program_counter(address);
                        print(&nes);
                        print_next_instruction(&nes, opcodes.clone());
                    },
                    None => println!("Please specify address"),
                };
            },
            "trace" => {
                log_file = Some(open_log_file());
            },
            "pattern" => {
                match cmd.hex_arg(1) {
                    Some(pattern) => {
                        println!("Layer1:\t\tLayer2:\t\tResult:");
                        for address in pattern..(pattern+8) {
                            let layer1 = nes.ppu.borrow().memory().get(address, 0);
                            let layer2 = nes.ppu.borrow().memory().get(address+8, 0);
                            println!(
                                "{:08b}\t{:08b}\tNOT IMPLEMENTED",
                                layer1,
                                layer2
                            );
                        }
                    },
                    None => {
                        for byte in 0x000..0x200 {
                            for b in 0..0x10 {
                                print!("0b{:08b},", nes.ppu.borrow().memory().get((byte << 1) | b, 0));
                            }
                            println!("");
                        }
                    }
                }
            },
            "sprites" => {
                let mut ppu = nes.ppu.borrow_mut();
                let sprites = ppu.sprites();
                for s in 0..64 {
                    let sprite = &sprites[s];
                    println!(
                        "Sprite {}: {} (x), {} (y), 0x{:x} (pattern), 0x{:02x} (attributes)",
                        s,
                        sprite.position_x(),
                        sprite.position_y(),
                        sprite.pattern_index(),
                        sprite[2]
                    );
                }
            },
            "name-table" => {
                let arg: u16 = cmd.arg(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                let base_address = 0x2000 + (arg*0x400);
                println!("Name Table:");
                for row in 0..30 {
                    for col in 0..32 {
                        let tile = nes.ppu.borrow().memory().get(base_address + row*32 + col, 0);
                        print!("{:02x} ", tile);
                    }
                    println!("");
                }
                println!("Attribute Table:");
                for row in 0..15 {
                    for col in 0..16 {
                        let colour_palette_index = {
                            let borrowed_ppu = nes.ppu.borrow();
                            let attribute_table = attributetable::AttributeTable {
                                memory: borrowed_ppu.memory(),
                                address: base_address + 0x3C0,
                            };
                            attribute_table.get_palette_index(row*2, col*2)
                        };
                        print!("{:02x} ", colour_palette_index);
                    }
                    println!("");
                }
            },
            "palette" => {
                let palette: u16 = cmd.arg(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                for i in 0..4 {
                    let palette_value = nes.ppu.borrow().memory().get(0x3F00 + 4*palette + i, 0) as usize;
                    println!("Palette value: {:x}: Colour {:?}", palette_value, COLOUR_PALETTE[palette_value]);
                }
            },
            "screenshot" => {
                let mut ppu = nes.ppu.borrow_mut();
                let mut screen = ScreenMock::new();
                ppu.invalidate_tile_cache();
                ppu.update_screen(&mut screen);
                let now: DateTime<Local> = Local::now();
                let file_name = format!("/tmp/rustiness.{:?}.png", now);

                image::save_buffer(
                    &Path::new(&file_name),
                    screen.screen_buffer.as_ref(),
                    256,
                    240,
                    image::RGB(8)
                ).unwrap();
                let background_file_name = format!("/tmp/rustiness.{:?}.bg.png", now);
                image::save_buffer(
                    &Path::new(&background_file_name),
                    screen.temp_buffer.as_ref(),
                    256*2,
                    240*2,
                    image::RGB(8)
                ).unwrap();
            },
            "stack" => {
                let mut entries: u8 = cmd.arg(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(5);
                let mut stack = nes.cpu.stack_pointer.wrapping_add(1);
                while entries > 0 && stack != 0xFF {
                    let pointer = stack as u16 + 0x100;
                    println!("Stack pointer 0x{:04x} -> 0x{:02x}", pointer, nes.memory.get(pointer, 0));
                    stack = stack.wrapping_add(1);
                    entries -= 1;
                }
            },
            "mem" => {
                let address = cmd.hex_arg(1).unwrap_or(0);
                let cycle = cmd.hex_arg(2).unwrap_or(0);
                println!("Memory 0x{:04x} -> 0x{:02x}", address, nes.memory.get(address, cycle as u8));
            },
            "press" => {
                match *fake_controller {
                    Some(ref ctrl) => ctrl.press(cmd.arg(1).map(|s| s.trim()).unwrap_or("")),
                    None => println!("Unable to fake button press since you're not using a fake controller, try run with '-c' flag")
                }
            },
            "release" => {
                match *fake_controller {
                    Some(ref ctrl) => ctrl.release(cmd.arg(1).map(|s| s.trim()).unwrap_or("")),
                    None => println!("Unable to fake button press since you're not using a fake controller, try run with '-c' flag")
                }
            }
            "exit" => break,
            _ => println!("Unknown cmd '{}'", cmd.name()),
        }
    }
}

fn print<S, A>(nes: &nes::NES<S, A>)
where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    println!("Cycle count: {}", nes.cycle_count);
    println!("Clock: {}", nes.clock);
    print_cpu_and_ppu(nes);
}

use std::io::{BufRead, BufReader};
use std::ops::Deref;
fn print_cpu_and_ppu<S, A>(nes: &nes::NES<S, A>)
where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    let cpu = &nes.cpu;
    let ppu = &nes.ppu;

    let mut cpu_buffer = Vec::new();
    cpu_buffer.write_fmt(format_args!("{}", cpu)).unwrap();
    let mut ppu_buffer = Vec::new();
    ppu_buffer
        .write_fmt(format_args!("{}", ppu.borrow().deref()))
        .unwrap();

    let cpu_buf_reader = BufReader::new(cpu_buffer.as_slice());
    let ppu_buf_reader = BufReader::new(ppu_buffer.as_slice());

    let mut ppu_lines = ppu_buf_reader.lines();
    for line in cpu_buf_reader.lines() {
        print!("{}\t", line.unwrap());
        println!(
            "{}",
            ppu_lines
                .next()
                .map(|r| r.unwrap())
                .unwrap_or(String::new())
        );
    }
}

fn next_instruction_as_string<S, A>(nes: &nes::NES<S, A>, opcodes: Rc<OpCodes>) -> String
where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    let op_code = nes.memory.get(nes.cpu.program_counter(), 0);
    opcodes.debug_instruction(op_code, &nes.cpu, &nes.memory)
}

fn print_next_instruction<S, A>(nes: &nes::NES<S, A>, opcodes: Rc<OpCodes>)
where
    S: Screen + Sized,
    A: AudioDevice + Sized,
{
    println!(
        "Next instruction: {}",
        next_instruction_as_string(nes, opcodes)
    );
}

fn read_input() -> Command {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return Command::from(line);
}
