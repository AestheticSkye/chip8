mod arguments;
mod color;
mod draw;
mod emulator;
mod ui;

use std::fs;

use anyhow::Result;
use arguments::scale::Scale;
use arguments::Arguments;
use clap::Parser;
use color::parse_color;
use emulator::Chip8;
use hex_color::HexColor;
use ui::Ui;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let program = fs::read(args.binary_path)?;

    let mut chip = Chip8::new(&program);

    let fg = if let Some(fg_string) = args.foreground_color {
        parse_color(&fg_string)?
    } else {
        HexColor::WHITE
    };

    let bg = if let Some(bg_string) = args.background_color {
        parse_color(&bg_string)?
    } else {
        HexColor::BLACK
    };

    let scale = args.scale.unwrap_or(Scale::X8);

    // TODO: figure out how to fix the ui not fully rendering on call.
    let mut ui = Ui::new(fg, bg, scale);

    chip.run(&mut ui)?;

    Ok(())
}
