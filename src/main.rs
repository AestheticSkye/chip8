mod arguments;
mod draw;
mod emulator;
mod ui;

use std::fs;

use anyhow::Result;
use arguments::Arguments;
use clap::Parser;
use emulator::Chip8;
use ui::Ui;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let program = fs::read(args.binary_path)?;

    let mut chip = Chip8::new(&program);

    let mut ui = Ui::new();

    chip.run(&mut ui)?;

    Ok(())
}
