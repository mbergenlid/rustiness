extern crate nes;
extern crate sdl2;

mod opcodes;
mod fakecontroller;
mod breakpoint;
use nes::NES;
use nes::memory::BasicMemory;
use nes::memory::Memory;
use nes::ines::INes;
use nes::ppu::{PPU, attributetable};
use nes::ppu::screen::{Screen, ScreenMock, COLOUR_PALETTE};
use nes::input::standard_controller::StandardController;
use sdl2::SDL2;
use nes::sound::AudioDevice;
use nes::sound::APU;
use sdl2::standard_controller::SdlEvents;
use self::fakecontroller::FakeController;

use std::fs::File;
use std::env;
use std::string::String;
use std::io;
use std::io::Write;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} debug FILE", args[0]);
    }

    let mut memory = box BasicMemory::new();

    let file = File::open(&args[2]).unwrap();
    let rom_file = box INes::from_file(file);
    rom_file.load(memory.as_mut());
    let ppu_memory = rom_file.ppu_memory();

    let ppu = PPU::with_mirroring(ppu_memory, rom_file.mirroring);
    let sdl = SDL2::new();

    let fake_controller: Option<FakeController> =
        args.iter().find(|&a| a == "-c")
            .map(|_| FakeController::new());
    let source = sdl.event_pump();
    let mut standard_controller =
        fake_controller.as_ref()
            .map(|c| StandardController::new(c))
            .unwrap_or_else(|| {
                StandardController::new(&source)
            });
    let apu = APU::new(sdl.audio(), 500);
    let use_screen_mock = args.iter().find(|&a| a == "-g").map(|_| true).unwrap_or(false);

    if use_screen_mock {
        let screen = box ScreenMock::new();
        let nes = nes::NES::new(ppu, apu, memory, screen, &mut standard_controller);

        run(nes, &source, &fake_controller);
    } else {
        let screen = box sdl.screen(2);

        let nes = nes::NES::new(ppu, apu, memory, screen, &mut standard_controller);

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

fn run<'a, S, A>(mut nes: NES<'a, S, A>, source: &SdlEvents, fake_controller: &Option<FakeController>) where S: Screen + Sized, A: AudioDevice + Sized {
    let mut break_points: Vec<Box<BreakPoint>> = vec!();
    let mut log_file = open_log_file();
    print(&nes);
    print_next_instruction(&nes);
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let cmd = read_input();
        match cmd.name() {
            "c" => {
                let cycles: u64 = cmd.arg(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
                let end_cycle = nes.cycle_count + cycles;
                println!("Running to cycle {}", end_cycle);
                nes.resume();
                let mut should_exit = false;
                let mut counter = 0;
                while (cycles == 0 || nes.cycle_count < end_cycle) && !should_exit {
                    log_file.write_fmt(format_args!("{}\n", next_instruction_as_string(&nes))).unwrap();
                    nes.execute();
                    counter += 1;
                    if counter > 0x100_000 {
                        should_exit = source.should_exit();
                        counter = 0;
                    }
                    should_exit = should_exit || break_points.breakpoint(&nes.cpu, &nes.memory);
                }
                println!("Clock {}", nes.clock);
                print(&nes);
                print_next_instruction(&nes);
            },
            "n" => {
                let arg: u32 = cmd.arg(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
                if arg > 1 {
                    for _ in 0..(arg) {
                        log_file.write_fmt(format_args!("{}\n", next_instruction_as_string(&nes))).unwrap();
                        nes.execute();
                        println!("Cycle count: {}", nes.cycle_count);
                    }
                } else {
                    log_file.write_fmt(format_args!("{}\n", next_instruction_as_string(&nes))).unwrap();
                    nes.execute();
                }
                print(&nes);
                print_next_instruction(&nes);
            },
            "break" => {
                match cmd.hex_arg(1) {
                    Some(destination_address) => {
                        println!("Set break point at address 0x{:02X}", destination_address);
                        break_points.push(box destination_address);
                    },
                    None => println!("Please specify address"),
                };
            },
            "set-pc" => {
                match cmd.hex_arg(1) {
                    Some(address) => {
                        nes.cpu.set_program_counter(address);
                        print(&nes);
                        print_next_instruction(&nes);
                    },
                    None => println!("Please specify address"),
                };
            },
            "pattern" => {
                match cmd.hex_arg(1) {
                    Some(pattern) => {
                        println!("Layer1:\t\tLayer2:\t\tResult:");
                        for address in pattern..(pattern+8) {
                            let layer1 = nes.ppu.borrow().memory().get(address);
                            let layer2 = nes.ppu.borrow().memory().get(address+8);
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
                                print!("0b{:08b},", nes.ppu.borrow().memory().get((byte << 1) | b));
                            }
                            println!("");
                        }
                    }
                }
            },
            "name-table" => {
                let arg: u16 = cmd.arg(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                let base_address = 0x2000 + (arg*0x400);
                println!("Name Table:");
                for row in 0..30 {
                    for col in 0..32 {
                        let tile = nes.ppu.borrow().memory().get(base_address + row*32 + col);
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
                    let palette_value = nes.ppu.borrow().memory().get(0x3F00 + 4*palette + i) as usize;
                    println!("Palette value: {:x}: Colour {:?}", palette_value, COLOUR_PALETTE[palette_value]);
                }
            },
            "screenshot" => {
                use nes::ppu::screen::PixelBuffer;
                let mut buffer = [0; 256*240*3];
                nes.ppu.borrow_mut().draw_buffer(&mut PixelBuffer { buffer: &mut buffer, pitch: 256*3, scale: 1});

                for y in 0..(240 as usize) {
                    for x in 0..(256 as usize) {
                        print!("({},{},{}), ", buffer[y*256*3 + x*3], buffer[y*256*3 + x*3+1], buffer[y*256*3 + x*3+2]);
                    }
                    println!("");
                }
            },
            "stack" => {
                let mut entries: u8 = cmd.arg(1).and_then(|s| s.parse::<u8>().ok()).unwrap_or(5);
                let mut stack = nes.cpu.stack_pointer.wrapping_add(1);
                while entries > 0 && stack != 0xFF {
                    let pointer = stack as u16 + 0x100;
                    println!("Stack pointer 0x{:04x} -> 0x{:02x}", pointer, nes.memory.get(pointer));
                    stack = stack.wrapping_add(1);
                    entries -= 1;
                }
            },
            "mem" => {
                let address = cmd.hex_arg(1).unwrap_or(0);
                println!("Memory 0x{:04x} -> 0x{:02x}", address, nes.memory.get(address));
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
struct Command {
    value: Vec<String>
}

impl Command {
    pub fn from(string: String) -> Command {
        Command {
            value: string.split_whitespace().map(|s| s.to_string()).collect()
        }
    }
    pub fn name(&self) -> &str {
        &self.value[0]
    }

    pub fn arg(&self, index: usize) -> Option<&String> {
        self.value.get(index)
    }

    pub fn hex_arg(&self, index: usize) -> Option<u16> {
        self.arg(index).and_then(|s| Command::parse_hex(s))
    }

    fn parse_hex(string: &String) -> Option<u16> {
        let mut value: u16 = 0;
        for c in string.chars() {
            let digit = c as u16;
            if digit >= 0x30 && digit <= 0x39 {
                value = value*16 + (digit - 0x30);
            } else if digit >= 0x41 && digit <= 0x46 {
                value = value*16 + (digit - 0x41 + 10);
            } else {
                return None;
            }
        }
        return Some(value);
    }
}

fn print<S, A>(nes: &nes::NES<S, A>) where S: Screen + Sized, A: AudioDevice + Sized {
    println!("Cycle count: {}", nes.cycle_count);
    println!("Clock: {}", nes.clock);
    print_cpu_and_ppu(nes);
}

use std::ops::Deref;
use std::io::{BufReader, BufRead};
fn print_cpu_and_ppu<S, A>(nes: &nes::NES<S, A>) where S: Screen + Sized, A: AudioDevice + Sized {
    let cpu = &nes.cpu;
    let ppu = &nes.ppu;

    let mut cpu_buffer = Vec::new();
    cpu_buffer.write_fmt(format_args!("{}", cpu)).unwrap();
    let mut ppu_buffer = Vec::new();
    ppu_buffer.write_fmt(format_args!("{}", ppu.borrow().deref())).unwrap();

    let cpu_buf_reader = BufReader::new(cpu_buffer.as_slice());
    let ppu_buf_reader = BufReader::new(ppu_buffer.as_slice());

    let mut ppu_lines = ppu_buf_reader.lines();
    for line in cpu_buf_reader.lines() {
        print!("{}\t", line.unwrap());
        println!("{}", ppu_lines.next().map(|r| r.unwrap()).unwrap_or(String::new()));
    }
}

fn next_instruction_as_string<S, A>(nes: &nes::NES<S, A>) -> String where S: Screen + Sized, A: AudioDevice + Sized {
    let op_code = nes.memory.get(nes.cpu.program_counter());
    opcodes::debug_instruction(op_code, &nes.cpu, &nes.memory)
}

fn print_next_instruction<S, A>(nes: &nes::NES<S, A>) where S: Screen + Sized, A: AudioDevice + Sized {
    println!("Next instruction: {}", next_instruction_as_string(nes));
}

fn read_input() -> Command {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return Command::from(line);
}
