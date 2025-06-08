use std::{fs::File, io::Read, path::Path};

pub struct CPU {
    pub memory: [u8; 4096],
    pub screen: [[u8; 64]; 32],
    pub registers: [u8; 16],
    pub keypad: [bool; 16],
    pub pc: usize,
    pub i: u16,
    pub stack: [u16; 16],
    pub sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub debug: bool,
    pub stop_on_missing_opcode: bool,
}

impl CPU {
    pub fn new_with_flags(debug: bool, strict: bool) -> Self {
        let mut cpu = Self {
            memory: [0; 4096],
            screen: [[0; 64]; 32],
            registers: [0; 16],
            keypad: [false; 16],
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            debug,
            stop_on_missing_opcode: strict,
        };
        cpu.load_font();
        cpu
    }

    pub fn load_font(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        self.memory[0x50..0x50 + 80].copy_from_slice(&fontset);
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.memory[0x200..0x200 + buffer.len()].copy_from_slice(&buffer);
        Ok(())
    }

    pub fn run_one_cycle(&mut self) {
        let opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
        self.pc += 2;

        if self.debug {
            println!(
                "PC: {:03X} OPCODE: {:04X} V0..V3: {:?}",
                self.pc - 2,
                opcode,
                &self.registers[0..4]
            );
        }

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.screen = [[0; 64]; 32],
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] as usize;
                }
                _ => self.unknown_opcode(opcode),
            },
            0x1000 => self.pc = (opcode & 0x0FFF) as usize,
            0x2000 => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = (opcode & 0x0FFF) as usize;
            }
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if self.registers[x] == (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if self.registers[x] != (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = (opcode & 0x00FF) as u8;
            }
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.registers[x] = self.registers[x].wrapping_add((opcode & 0x00FF) as u8);
            }
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    0x0 => self.registers[x] = self.registers[y],
                    0x1 => self.registers[x] |= self.registers[y],
                    0x2 => self.registers[x] &= self.registers[y],
                    0x3 => self.registers[x] ^= self.registers[y],
                    0x4 => {
                        let (res, carry) = self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[0xF] = carry as u8;
                        self.registers[x] = res;
                    }
                    0x5 => {
                        self.registers[0xF] = if self.registers[x] >= self.registers[y] {
                            1
                        } else {
                            0
                        };
                        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
                    }
                    0x6 => {
                        self.registers[0xF] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                    }
                    0x7 => {
                        self.registers[0xF] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
                        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                    }
                    0xE => {
                        self.registers[0xF] = (self.registers[x] >> 7) & 1;
                        self.registers[x] <<= 1;
                    }
                    _ => self.unknown_opcode(opcode),
                }
            }
            0xA000 => self.i = opcode & 0x0FFF,
            0xC000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                let rnd: u8 = rand::random();
                self.registers[x] = rnd & nn;
            }
            0xD000 => self.draw_sprite(opcode),
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x9E => {
                        if self.keypad[self.registers[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if !self.keypad[self.registers[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => self.unknown_opcode(opcode),
                }
            }
            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x07 => self.registers[x] = self.delay_timer,
                    0x0A => {
                        if let Some(key) = self.keypad.iter().position(|&k| k) {
                            self.registers[x] = key as u8;
                        } else {
                            self.pc -= 2;
                        }
                    }
                    0x15 => self.delay_timer = self.registers[x],
                    0x18 => self.sound_timer = self.registers[x],
                    0x1E => self.i = self.i.wrapping_add(self.registers[x] as u16),
                    0x29 => self.i = 0x50 + (self.registers[x] as u16 * 5),
                    0x33 => {
                        let vx = self.registers[x];
                        self.memory[self.i as usize] = vx / 100;
                        self.memory[self.i as usize + 1] = (vx / 10) % 10;
                        self.memory[self.i as usize + 2] = vx % 10;
                    }
                    0x55 => {
                        for offset in 0..=x {
                            self.memory[self.i as usize + offset] = self.registers[offset];
                        }
                    }
                    0x65 => {
                        for offset in 0..=x {
                            self.registers[offset] = self.memory[self.i as usize + offset];
                        }
                    }
                    _ => self.unknown_opcode(opcode),
                }
            }
            _ => self.unknown_opcode(opcode),
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            print!("\x07");
        }
    }

    fn draw_sprite(&mut self, opcode: u16) {
        let x = self.registers[((opcode & 0x0F00) >> 8) as usize] as usize;
        let y = self.registers[((opcode & 0x00F0) >> 4) as usize] as usize;
        let height = (opcode & 0x000F) as usize;
        self.registers[0xF] = 0;
        for byte in 0..height {
            let pixel = self.memory[self.i as usize + byte];
            for bit in 0..8 {
                let x_coord = (x + bit) % 64;
                let y_coord = (y + byte) % 32;
                let bit_val = (pixel >> (7 - bit)) & 1;
                if bit_val == 1 {
                    if self.screen[y_coord][x_coord] == 1 {
                        self.registers[0xF] = 1;
                    }
                    self.screen[y_coord][x_coord] ^= 1;
                }
            }
        }
    }

    fn unknown_opcode(&mut self, opcode: u16) {
        println!("Unimplemented opcode: {:04X}", opcode);
        if self.stop_on_missing_opcode {
            panic!("Halting on unknown opcode.");
        }
    }
}
