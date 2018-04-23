#![feature(box_syntax)]
#[macro_use]
extern crate nes;

mod fakes;
use nes::cpu::opcodes;
use nes::input::standard_controller::StandardController;
use nes::ppu::screen::ScreenMock;
use nes::ppu::ppumemory::PPUMemory;
use nes::NES;
use nes::borrow::MutableRef;
use nes::ines::mapper::Mapper;

#[test]
fn vbl_timing() {
    let memory = memory!(
            0x8000 => opcodes::NOP_IMPLIED,
            0x8001 => opcodes::NOP_IMPLIED,
            0x8002 => opcodes::NOP_IMPLIED,
            0x8003 => opcodes::NOP_IMPLIED,
            0x8004 => opcodes::NOP_IMPLIED,
            0x8005 => opcodes::NOP_IMPLIED,
            0x8006 => opcodes::NOP_IMPLIED,
            0x8007 => opcodes::NOP_IMPLIED,

            0x8008 => opcodes::BIT_ABSOLUTE,
            0x8009 => 0x02,
            0x800A => 0x20,

            0x800B => opcodes::BIT_ABSOLUTE,
            0x800C => 0x02,
            0x800D => 0x20,

            0x800E => opcodes::BRANCH_PLUS,
            0x800F => (-(0x10 as i32)) as u8,

            0x8010 => opcodes::LDA_ABSOLUTE,
            0x8011 => 0x02,
            0x8012 => 0x20,

            0xFFFC => 0x00,
            0xFFFD => 0x80
        );
    let controller = fakes::controller::FakeController::new();
    let standard_controller = StandardController::new(&controller);

    let screen = box ScreenMock::new();

    let mut nes = NES::new(
        Mapper { cpu_memory: box memory, ppu_memory: PPUMemory::no_mirroring() },
        MutableRef::Box(box standard_controller),
        fakes::audio_device::AudioDevice {},
        screen
    );

    delay(&mut nes.ppu.borrow_mut(), 27393-(27-3));
    println!("{}", nes.ppu.borrow());
    for _ in 0..10 {
        nes.execute();
    }
    println!("{}", nes.ppu.borrow()); //82_179
    for _ in 0..(11*1103) {
        nes.execute();
        if nes.cpu.program_counter() >= 0x8010 {
            println!("AHSD");
        }
    }
    println!("{}", nes.ppu.borrow()); //82_180
    for _ in 0..(11*1103) {
        nes.execute();
        if nes.cpu.program_counter() >= 0x8010 {
            println!("AHSD");
        }
    }
    println!("{}", nes.ppu.borrow()); //82_181
    for _ in 0..(11*1103) {
        nes.execute();
        if nes.cpu.program_counter() >= 0x8010 {
            println!("AHSD");
        }
    }
    println!("{}", nes.ppu.borrow()); //82_182
    nes.execute();
    println!("{}", nes.ppu.borrow());
    assert_eq!(nes.cpu.program_counter(), 0x8010);
    delay(&mut nes.ppu.borrow_mut(), 29_775);

    println!("{}", nes.ppu.borrow());
    assert!(nes.ppu.borrow_mut().status(0) & 0x80 == 0);

    nes.execute();
    //delay(&mut nes.ppu.borrow_mut(), 1);

    println!("{}", nes.ppu.borrow());
    assert!(nes.cpu.accumulator() & 0x80 != 0);

}

use nes::ppu::PPU;
fn delay(ppu: &mut PPU, cycles: u32) {
    let mut screen = ScreenMock::new();
    for _ in 0..cycles {
        ppu.sync(1, &mut screen);
    }
}

#[test]
fn nmi_timing() {
    let memory = memory!(
            0x8000 => opcodes::NOP_IMPLIED,

            0x8001 => opcodes::JMP_ABSOLUTE,
            0x8002 => 0x00,
            0x8003 => 0x80,

            0xFFFC => 0x00,
            0xFFFD => 0x80
        );
    let controller = fakes::controller::FakeController::new();
    let standard_controller = StandardController::new(&controller);

    let screen = box ScreenMock::new();

    let mut nes = NES::new(
        Mapper { cpu_memory: box memory, ppu_memory: PPUMemory::no_mirroring() },
        MutableRef::Box(box standard_controller),
        fakes::audio_device::AudioDevice {},
        screen
    );

    nes.ppu.borrow_mut().set_ppu_ctrl(0x80); //Enable nmi
    delay(&mut nes.ppu.borrow_mut(), 27_389);
    println!("{}", nes.ppu.borrow());
    nes.execute();
    nes.execute();

    assert_eq!(0x8000, nes.cpu.program_counter());
    nes.execute();
    assert_eq!(0x0000, nes.cpu.program_counter());
}
