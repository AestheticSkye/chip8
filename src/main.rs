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
use ui::Ui;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let program = fs::read(args.binary_path)?;

    let fg = parse_color(&args.foreground_color)?;

    let bg = parse_color(&args.background_color)?;

    let scale = args.scale.unwrap_or(Scale::X8);

    let mut chip = Chip8::new(&program, args.compatability_mode);

    // TODO: figure out how to fix the ui not fully rendering on call.
    let mut ui = Ui::new(fg, bg, scale);

    chip.run(&mut ui)?;

    Ok(())
}
