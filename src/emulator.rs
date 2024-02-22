mod font;
mod instruction;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use font::FONT;

use self::instruction::Instruction;
use crate::draw::Draw;

const BLANK_DISPLAY: [bool; 64 * 32] = [false; 64 * 32];

/// Original spec specified a 48 byte stack,
/// Since the stack will be represented by u16s
/// the size can be halved.
const STACK_SIZE: usize = 24;

#[derive(Debug, Clone, Copy)]
pub struct Chip8 {
    memory:          [u8; 4096],
    display:         [bool; 64 * 32],
    stack:           [u16; STACK_SIZE],
    var_registers:   [u8; 16],
    program_counter: u16,
    index_register:  u16,
    delay_timer:     u8,
    sound_timer:     u8,
}

impl Chip8 {
    pub fn new(executable: &[u8]) -> Self {
        let mut memory = [0; 4096];

        // Insert font into memory
        for (font_index, memory_index) in (0x050..=0x09F).enumerate() {
            memory[memory_index] = FONT[font_index];
        }

        // Load executable
        for (instruction_index, memory_address_index) in
            (0x200..(0x200 + executable.len())).enumerate()
        {
            memory[memory_address_index] = executable[instruction_index];
        }

        Self {
            memory,
            display: BLANK_DISPLAY,
            program_counter: 0x200,
            index_register: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            var_registers: [0; 16],
        }
    }

    pub fn run(&mut self, ui: &mut impl Draw) -> Result<()> {
        loop {
            let instruction_range =
                self.program_counter as usize..(self.program_counter + 2) as usize;

            println!("{instruction_range:?}");

            let bytes = &self.memory[instruction_range];

            println!("{bytes:?}");

            // Concatenate the two bytes together.
            let instruction = ((u16::from(bytes[0]) << 8) + u16::from(bytes[1])).try_into()?;

            self.run_instruction(instruction, ui);

            self.program_counter += 2;

            // Run 700 instructions per second.
            sleep(Duration::from_micros(1428));
        }
    }

    fn run_instruction(&mut self, instruction: Instruction, ui: &mut impl Draw) {
        match instruction {
            Instruction::ClearScreen => self.display = BLANK_DISPLAY,
            Instruction::Return => todo!(),
            Instruction::Goto(address) => self.program_counter = address,
            Instruction::Subroutine(_) => todo!(),
            Instruction::SetRegister { register, value } => {
                self.var_registers[register as usize] = value;
            }
            Instruction::AddRegister { register, value } => {
                self.var_registers[register as usize] += value;
            }
            Instruction::SetIndexRegister(value) => self.index_register = value,
            Instruction::Display {
                x_coord_register,
                y_coord_register,
                sprite_height,
            } => {
                let x_coord = self.var_registers[x_coord_register as usize];
                let y_coord = self.var_registers[y_coord_register as usize];

                let sprite_address = self.index_register;

                self.draw(x_coord, y_coord, sprite_height, sprite_address, ui);
            }
        }
    }

    fn draw(
        &mut self,
        x_coord: u8,
        y_coord: u8,
        sprite_height: u8,
        sprite_address: u16,
        ui: &mut impl Draw,
    ) {
    }
}
