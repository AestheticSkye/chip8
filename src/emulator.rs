mod font;
mod instruction;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use bitvec::order::Msb0;
use bitvec::view::BitView;

use self::font::FONT;
use self::instruction::Instruction;
use crate::draw::Draw;

const BLANK_DISPLAY: [[bool; 64]; 32] = [[false; 64]; 32];

/// Original spec specified a 48 byte stack,
/// Since the stack will be represented by u16s
/// the size can be halved.
const STACK_SIZE: usize = 24;

#[derive(Debug, Clone, Copy)]
pub struct Chip8 {
    memory:          [u8; 4096],
    display:         [[bool; 64]; 32],
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

            self.program_counter += 2;

            self.run_instruction(instruction, ui)?;

            // ui.draw(&self.display)?;

            // Run 700 instructions per second.
            sleep(Duration::from_micros(1428));
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run_instruction(&mut self, instruction: Instruction, ui: &mut impl Draw) -> Result<()> {
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
            Instruction::SetVal { register, value } => {
                self.var_registers[register as usize] = value;
            }
            Instruction::AddVal { register, value } => {
                self.var_registers[register as usize] =
                    self.var_registers[register as usize].wrapping_add(value);
            }
            Instruction::Set {
                register_a,
                register_b,
            } => {
                self.var_registers[register_a as usize] = self.var_registers[register_b as usize];
            }
            Instruction::Or {
                register_a,
                register_b,
            } => {
                self.var_registers[register_a as usize] |= self.var_registers[register_b as usize];
            }
            Instruction::And {
                register_a,
                register_b,
            } => {
                self.var_registers[register_a as usize] &= self.var_registers[register_b as usize];
            }
            Instruction::Xor {
                register_a,
                register_b,
            } => {
                self.var_registers[register_a as usize] ^= self.var_registers[register_b as usize];
            }
            Instruction::Add {
                register_a,
                register_b,
            } => todo!(),
            Instruction::SubtractRight {
                register_a,
                register_b,
            } => todo!(),
            Instruction::ShiftLeft {
                register_a,
                register_b,
            } => todo!(),
            Instruction::SubtractLeft {
                register_a,
                register_b,
            } => todo!(),
            Instruction::ShiftRight {
                register_a,
                register_b,
            } => todo!(),
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
                let column = self.var_registers[x_coord_register as usize] % 64;
                let row = self.var_registers[y_coord_register as usize] % 32;

                self.draw(column, row, sprite_height, ui)?;
            }
        }

        Ok(())
    }

    fn draw(
        &mut self,
        start_column: u8,
        start_row: u8,
        sprite_height: u8,
        ui: &mut impl Draw,
    ) -> Result<()> {
        self.var_registers[0xF] = 0;

        let sprite_address = self.index_register as usize;

        for row in 0..sprite_height as usize {
            let byte = self.memory[sprite_address + row];

            for (column, bit) in byte.view_bits::<Msb0>().iter().enumerate() {
                let display_bit =
                    &mut self.display[start_row as usize + row][start_column as usize + column];

                if *display_bit {
                    self.var_registers[0xF] = 1;
                }

                *display_bit = *bit;
            }
        }

        ui.draw(&self.display)?;

        Ok(())
    }
}
