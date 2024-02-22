mod draw;
mod emulator;
mod ui;

use anyhow::Result;
use emulator::Chip8;
use ui::Ui;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() -> Result<()> {
    let mut ui = Ui::new();

    let mut chip = Chip8::new();

    // chip.load_program(&program);

    chip.run(&mut ui)?;

    Ok(())
}
