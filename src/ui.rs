use anyhow::Result;
use hex_color::HexColor;
use minifb::{Key, Window, WindowOptions};

use crate::arguments::scale::Scale;
use crate::draw::Draw;
use crate::{HEIGHT, WIDTH};

pub struct Ui {
    window:           Window,
    foreground_color: HexColor,
    background_color: HexColor,
}

impl Ui {
    pub fn new(foreground_color: HexColor, background_color: HexColor, scale: Scale) -> Self {
        let options = WindowOptions {
            scale: scale.into(),
            ..WindowOptions::default()
        };

        let mut window =
            Window::new("Chip8 - ESC to exit", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            window,
            foreground_color,
            background_color,
        }
    }
}

impl Draw for Ui {
    fn draw(&mut self, buffer: &[[bool; 64]; 32]) -> Result<()> {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return Ok(());
        }

        let buffer = buffer
            .iter()
            .flatten()
            .map(|val| {
                if *val {
                    self.foreground_color.to_u24()
                } else {
                    self.background_color.to_u24()
                }
            })
            .collect::<Vec<u32>>();

        self.window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;

        Ok(())
    }
}
