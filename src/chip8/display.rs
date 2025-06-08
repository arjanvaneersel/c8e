use minifb::{Key, Window, WindowOptions};

pub struct Display {
    pub window: Window,
}

impl Display {
    pub fn new(title: &str, width: usize, height: usize, opts: WindowOptions) -> Self {
        let window = Window::new(title, width, height, opts).expect("Window creation failed");
        Self { window }
    }

    pub fn draw(&mut self, screen: &[[u8; 64]; 32]) {
        const SCALE: usize = 10;
        const WIDTH: usize = 64;
        const HEIGHT: usize = 32;

        let mut buffer = vec![0u32; WIDTH * SCALE * HEIGHT * SCALE];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if screen[y][x] == 0 {
                    0x000000
                } else {
                    0xFFFFFF
                };
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let sx = x * SCALE + dx;
                        let sy = y * SCALE + dy;
                        buffer[sy * WIDTH * SCALE + sx] = color;
                    }
                }
            }
        }

        self.window
            .update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)
            .unwrap();
    }
}

pub fn map_key(key: Key) -> Option<usize> {
    match key {
        // Player 1 – WASD
        Key::W => Some(0x5),
        Key::A => Some(0x4),
        Key::S => Some(0x6),
        Key::D => Some(0xD),

        // Player 2 – IJKL
        Key::I => Some(0x8),
        Key::J => Some(0x7),
        Key::K => Some(0x9),
        Key::L => Some(0xE),

        // Optional: map remaining CHIP-8 keys for debugging
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}
