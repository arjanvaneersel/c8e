mod chip8;

use crate::chip8::{CPU, Display, display::map_key};
use minifb::{Key, KeyRepeat, WindowOptions};
use std::time::{Duration, Instant};

const SCALE: usize = 10;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Extract ROM path and optional flags
    let rom_path = args.get(1).filter(|s| !s.starts_with("--")).cloned();
    let debug = args.contains(&"--debug".to_string());
    let strict = args.contains(&"--strict".to_string());

    let rom_path = match rom_path {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <rom_file> [--debug] [--strict]", args[0]);
            std::process::exit(1);
        }
    };

    let mut cpu = CPU::new_with_flags(debug, strict);
    cpu.load_rom(&rom_path).expect("Failed to load ROM");

    let mut display = Display::new(
        "CHIP-8 Emulator",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    );

    let mut last_timer_update = Instant::now();

    while display.window.is_open() && !display.window.is_key_down(Key::Escape) {
        let keys = display.window.get_keys();
        cpu.keypad = [false; 16];
        for key in keys {
            if let Some(i) = map_key(key) {
                cpu.keypad[i] = true;
            }
        }

        cpu.run_one_cycle();

        if last_timer_update.elapsed() >= Duration::from_millis(16) {
            cpu.update_timers();
            last_timer_update = Instant::now();
        }

        display.draw(&cpu.screen);
    }
}
