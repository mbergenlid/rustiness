extern crate nes;
extern crate gliumscreen;
mod opcodes;
use nes::memory::BasicMemory;
use nes::memory::Memory;
use nes::ines::INes;
use nes::ppu::PPU;
use nes::ppu::screen::ScreenMock;
use gliumscreen::GliumScreen;

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

    let ppu = match args.iter().find(|arg| arg.trim() == "-g") {
        Some(_) => PPU::new(
            ppu_memory,
            box (GliumScreen::new(2))
        ),
        None => PPU::new(
            ppu_memory,
            box ScreenMock::new()
        )
    };
    let mut nes = nes::NES::new(ppu);


    loop {
        print(&nes);
        print_next_instruction(&nes, memory.as_mut());
        print!(">");
        io::stdout().flush().unwrap();
        let cmd = read_input();
        match cmd.name() {
            "next" => {
                let arg: u32 = cmd.arg(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
                if arg > 1 {
                    for _ in 0..(arg) {
                        nes.execute(memory.as_mut());
                        println!("Cycle count: {}", nes.cycle_count);
                    }
                } else {
                    nes.execute(memory.as_mut());
                }
            },
            "goto" => {
                match cmd.hex_arg(1) {
                    Some(destination_address) => {
                        println!("Continuing to address 0x{:02X}", destination_address);
                        while nes.cpu.program_counter() != destination_address {
                            nes.execute(memory.as_mut());
                            println!("Current Address: 0x{:02X}", nes.cpu.program_counter());
                        }
                    },
                    None => println!("Please specify address"),
                };
            },
            "pattern" => {
                match cmd.hex_arg(1) {
                    Some(pattern) => {
                        println!("Layer1:\t\tLayer2:\t\tResult:");
                        for address in pattern..(pattern+8) {
                            let layer1 = nes.ppu.memory().get(address);
                            let layer2 = nes.ppu.memory().get(address+8);
                            println!(
                                "{:08b}\t{:08b}\tNOT IMPLEMENTED",
                                layer1,
                                layer2
                            );
                        }
                    },
                    None => println!("Not a valid address"),
                }
            },
            "name-table" => {
                let arg: u16 = cmd.arg(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
                let base_address = 0x2000 + (arg*0x400);
                for row in 0..30 {
                    for col in 0..32 {
                        let tile = nes.ppu.memory().get(base_address + row*32 + col);
                        print!("{:02x} ", tile);
                    }
                    println!("");
                }
            },
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
            if digit < 0x30 || digit > 0x39 {
                return None;
            }
            value = value*16 + (digit - 0x30);
        }
        return Some(value)
    }
}

fn print(nes: &nes::NES) {
    println!("Cycle count: {}", nes.cycle_count);
    print_cpu_and_ppu(nes);
}

use std::io::{BufReader, BufRead};
fn print_cpu_and_ppu(nes: &nes::NES) {
    let cpu = &nes.cpu;
    let ppu = &nes.ppu;

    let mut cpu_buffer = Vec::new();
    cpu_buffer.write_fmt(format_args!("{}", cpu)).unwrap();
    let mut ppu_buffer = Vec::new();
    ppu_buffer.write_fmt(format_args!("{}", ppu)).unwrap();

    let cpu_buf_reader = BufReader::new(cpu_buffer.as_slice());
    let ppu_buf_reader = BufReader::new(ppu_buffer.as_slice());

    let mut ppu_lines = ppu_buf_reader.lines();
    for line in cpu_buf_reader.lines() {
        print!("{}\t", line.unwrap());
        println!("{}", ppu_lines.next().map(|r| r.unwrap()).unwrap_or(String::new()));
    }
}

fn print_next_instruction(nes: &nes::NES, memory: &Memory) {
    let op_code = memory.get(nes.cpu.program_counter());

    opcodes::debug_instruction(op_code, &nes.cpu, memory);
}

fn read_input() -> Command {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return Command::from(line);
}
