mod font;
mod instruction;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;

use self::font::FONT;
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
    stack_pointer:   usize,
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
        for (instruction_index, memory_index) in (0x200..(0x200 + executable.len())).enumerate() {
            memory[memory_index] = executable[instruction_index];
        }

        Self {
            memory,
            display: BLANK_DISPLAY,
            program_counter: 0x200,
            index_register: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            var_registers: [0; 16],
        }
    }

    pub fn run(&mut self, ui: &mut impl Draw) -> Result<()> {
        loop {
            let instruction_range =
                self.program_counter as usize..(self.program_counter + 2) as usize;

            let bytes = &self.memory[instruction_range];

            // Concatenate the two bytes together.
            let instruction = ((u16::from(bytes[0]) << 8) + u16::from(bytes[1])).try_into()?;

            self.run_instruction(instruction, ui);

            self.program_counter += 2;

            // Run 700 instructions per second.
            sleep(Duration::from_micros(1428));
        }
    }

    fn run_instruction(&mut self, instruction: Instruction, ui: &mut impl Draw) {
        println!("{instruction:?}");

        match instruction {
            Instruction::ClearScreen => self.display = BLANK_DISPLAY,
            Instruction::Return => {
                self.program_counter = self.stack[self.stack_pointer];
                self.stack[self.stack_pointer] = 0;
                self.stack_pointer -= 1;
            }
            Instruction::Goto(address) => self.program_counter = address,
            Instruction::Subroutine(address) => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter;
                self.program_counter = address;
            }
            Instruction::IsEqualVal { register, value } => {
                if self.var_registers[register as usize] == value {
                    self.program_counter += 2;
                }
            }
            Instruction::NotEqualVal { register, value } => {
                if self.var_registers[register as usize] != value {
                    self.program_counter += 2;
                }
            }
            Instruction::IsEqual {
                register_a,
                register_b,
            } => {
                if self.var_registers[register_a as usize]
                    == self.var_registers[register_b as usize]
                {
                    self.program_counter += 2;
                }
            }
            Instruction::SetRegister { register, value } => {
                self.var_registers[register as usize] = value;
            }
            Instruction::AddRegister { register, value } => {
                self.var_registers[register as usize] =
                    self.var_registers[register as usize].wrapping_add(value);
            }
            Instruction::NotEqual {
                register_a,
                register_b,
            } => {
                if self.var_registers[register_a as usize]
                    != self.var_registers[register_b as usize]
                {
                    self.program_counter += 2;
                }
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
