mod condition_codes;
mod cpu;
mod display;
mod instruction;
mod registers;

use cpu::Cpu;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;

extern crate sdl2;
use display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time::Duration;

fn load_roms(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut addr = 0x00;
    for f in ['h', 'g', 'f', 'e'].iter() {
        let mut file = File::open(format!("roms/invaders.{}", f))?;
        file.read(&mut buffer[addr..addr + 0x800])?;
        addr += 0x800;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    match load_roms(&mut cpu.memory) {
        Ok(_) => (),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = Display::new(sdl_context);

    let debug = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // this where keyboard input would go
                _ => {}
            }
        }
        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);
        let (next_pc, cycles) = cpu.execute(&instr);
        cpu.pc = next_pc;
        if debug {
            println!("{:?}", instr);
            println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
            println!("cycles: {}", cycles);
            println!("{:#x?}", cpu.condition_codes);
            println!("{:#x?}\n", cpu.registers);
        }

        // TODO: work on interuppts and timing
        if !cpu.interrupts_enabled {
            display.draw_display(&mut cpu);
        }
        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
