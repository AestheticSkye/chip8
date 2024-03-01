mod font;
mod instruction;

use std::thread::{sleep, sleep_ms};
use std::time::Duration;

use anyhow::Result;
use bitvec::order::Msb0;
use bitvec::view::BitView;

use self::font::FONT;
use self::instruction::Instruction;
use crate::arguments::compatability_mode::CompatabilityMode;
use crate::draw::Draw;

const BLANK_DISPLAY: [[bool; 64]; 32] = [[false; 64]; 32];

/// Original spec specified a 48 byte stack,
/// Since the stack will be represented by u16s
/// the size can be halved.
const STACK_SIZE: usize = 24;

#[derive(Debug, Clone, Copy)]
pub struct Chip8 {
    memory:             [u8; 4096],
    display:            [[bool; 64]; 32],
    stack:              [u16; STACK_SIZE],
    stack_pointer:      usize,
    var_registers:      [u8; 16],
    program_counter:    u16,
    index_register:     u16,
    delay_timer:        u8,
    sound_timer:        u8,
    compatibility_mode: CompatabilityMode,
}

impl Chip8 {
    pub fn new(executable: &[u8], compatibility_mode: CompatabilityMode) -> Self {
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
            compatibility_mode,
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

            ui.draw(&self.display)?;

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
                register_x,
                register_y,
            } => {
                if self.var_registers[register_x as usize]
                    == self.var_registers[register_y as usize]
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
                register_x,
                register_y,
            } => {
                self.var_registers[register_x as usize] = self.var_registers[register_y as usize];
            }
            Instruction::Or {
                register_x,
                register_y,
            } => {
                self.var_registers[register_x as usize] |= self.var_registers[register_y as usize];
            }
            Instruction::And {
                register_x,
                register_y,
            } => {
                self.var_registers[register_x as usize] &= self.var_registers[register_y as usize];
            }
            Instruction::Xor {
                register_x,
                register_y,
            } => {
                self.var_registers[register_x as usize] ^= self.var_registers[register_y as usize];
            }
            Instruction::Add {
                register_x,
                register_y,
            } => {
                let y = u16::from(self.var_registers[register_y as usize]);
                let register_x = &mut u16::from(self.var_registers[register_x as usize]);

                if (*register_x + y) > 255 {
                    self.var_registers[0xF] = 1;
                }

                *register_x = register_x.wrapping_add(y);
            }
            Instruction::SubtractRight {
                register_x,
                register_y,
            } => {
                let x = self.var_registers[register_x as usize];
                let y = self.var_registers[register_y as usize];

                self.var_registers[0xF] = u8::from(x > y);
                self.var_registers[register_x as usize] = x.wrapping_sub(y);
            }
            Instruction::ShiftLeft {
                register_x,
                register_y,
            } => {
                let value = match self.compatibility_mode {
                    CompatabilityMode::Cosmac => self.var_registers[register_y as usize],
                    _ => self.var_registers[register_x as usize],
                };

                let shifted_value = value << 1;
                let shifted_bit = value & 1;

                self.var_registers[register_x as usize] = shifted_value;
                self.var_registers[0xF] = shifted_bit;
            }
            Instruction::SubtractLeft {
                register_x,
                register_y,
            } => {
                let x = self.var_registers[register_x as usize];
                let y = self.var_registers[register_y as usize];

                self.var_registers[0xF] = u8::from(y > x);
                self.var_registers[register_x as usize] = y.wrapping_sub(x);
            }
            Instruction::ShiftRight {
                register_x,
                register_y,
            } => {
                let value = match self.compatibility_mode {
                    CompatabilityMode::Cosmac => self.var_registers[register_y as usize],
                    _ => self.var_registers[register_x as usize],
                };

                let shifted_value = value >> 1;
                let shifted_bit = value & 0b1000_0000;

                self.var_registers[register_x as usize] = shifted_value;
                self.var_registers[0xF] = shifted_bit;
            }
            Instruction::NotEqual {
                register_x,
                register_y,
            } => {
                if self.var_registers[register_x as usize]
                    != self.var_registers[register_y as usize]
                {
                    self.program_counter += 2;
                }
            }
            Instruction::SetIndexRegister(value) => self.index_register = value,
            Instruction::Rand { register, value } => {
                let random_value = rand::random::<u8>() & value;
                self.var_registers[register as usize] = random_value;
            }
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

        'a: for row in 0..sprite_height as usize {
            let byte = self.memory[sprite_address + row];

            for (column, bit) in byte.view_bits::<Msb0>().iter().enumerate() {
                let Some(display_row) = self.display.get_mut(start_row as usize + row) else {
                    // Rest of sprite goes out of bounds, end drawing.
                    break 'a;
                };

                let Some(display_bit) = display_row.get_mut(start_column as usize + column) else {
                    // Current row goe out of bounds, go to next row.
                    break;
                };

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
