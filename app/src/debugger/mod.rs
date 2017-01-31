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

fn print_cpu_and_ppu(nes: &nes::NES) {
    use nes::cpu;
    let cpu = &nes.cpu;
    let ppu = &nes.ppu;
    println!("CPU:\t\t\t\t\t|\tPPU:");
    println!("\tProgram counter:  0x{:4X}\t|\t\tControl register: 0b{:08b}", cpu.program_counter(), ppu.ppu_ctrl());
    println!("\tProcessor status: N O B D I Z C\t|\t\tVRAM Pointer: 0x{:08x}", ppu.vram());
    println!("\t                  {} {} {} {} {} {} {}\t|",
             cpu.is_flag_set(cpu::NEGATIVE_FLAG) as u8,
             cpu.is_flag_set(cpu::OVERFLOW_FLAG) as u8,
             cpu.is_flag_set(cpu::BREAK_FLAG) as u8 ,
             cpu.is_flag_set(cpu::DECIMAL_FLAG) as u8,
             cpu.is_flag_set(cpu::INTERRUPT_DISABLE_FLAG) as u8,
             cpu.is_flag_set(cpu::ZERO_FLAG) as u8,
             cpu.is_flag_set(cpu::CARRY_FLAG) as u8
    );
    println!("\tAccumulator:      0x{:02X}\t\t|", cpu.accumulator());
    println!("\tRegister X:       0x{:02X}\t\t|", cpu.register_x());
    println!("\tRegister Y:       0x{:02X}\t\t|", cpu.register_y());
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
