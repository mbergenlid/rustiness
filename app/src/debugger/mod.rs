
extern crate nes;
mod opcodes;
use nes::memory::BasicMemory;
use nes::memory::Memory;
use nes::ines::INes;
use nes::cpu::CPU;

use std::fs::File;
use std::env;
use std::string::String;
use std::io;
use std::io::Write;

pub fn start() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} FILE", args[0]);
    }
    let mut nes = nes::NES::new();
    let file = File::open(&args[1]).unwrap();

    let mut memory = BasicMemory::new();
    let rom_file = INes::from_file(file);
    rom_file.load(&mut memory);

    loop {
        print_cpu(&nes.cpu);
        print_next_instruction(&nes, &memory);
        print!(">");
        io::stdout().flush();
        let cmd = read_input();
        match cmd.trim() {
            "next" => nes.execute(&mut memory),
            "exit" => break,
            _ => println!("Unknown cmd '{}'", cmd),
        }

    }
}

fn print_cpu(cpu: &CPU) {
    println!("CPU:");
    println!("\tProgram counter:  0x{:x}", cpu.program_counter());
    println!("\tProcessor status: 0b{:b}", cpu.processor_status());
    println!("\tAccumulator:      0x{:x}", cpu.accumulator());
    println!("\tRegister X:       0x{:x}", cpu.register_x());
    println!("\tRegister Y:       0x{:x}", cpu.register_y());
}

fn print_next_instruction(nes: &nes::NES, memory: &Memory) {
    let op_code = memory.get(nes.cpu.program_counter());

    opcodes::debug_instruction(op_code, &nes.cpu, memory);
}

fn read_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line;
}
