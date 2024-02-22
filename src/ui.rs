use anyhow::Result;
use minifb::{Key, Scale, Window, WindowOptions};

use crate::draw::Draw;
use crate::{HEIGHT, WIDTH};

pub struct Ui {
    window: Window,
}

impl Ui {
    pub fn new() -> Self {
        let options = WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        };

        let mut window =
            Window::new("Chip8 - ESC to exit", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self { window }
    }
}

impl Draw for Ui {
    type Error = anyhow::Error;

    fn draw(&mut self, buffer: &[bool]) -> Result<()> {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return Ok(());
        }

        let buffer = buffer
            .iter()
            .map(|val| if *val { u32::MAX } else { 0 })
            .collect::<Vec<u32>>();

        self.window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;

        Ok(())
    }
}

impl Default for Ui {
    fn default() -> Self { Self::new() }
}
