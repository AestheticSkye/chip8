use anyhow::{bail, Error};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// 00E0
    ClearScreen,
    /// 00EE
    Return,
    /// 1NNN
    Goto(u16),
    /// 2NNN
    Subroutine(u16),
    /// 6XNN
    SetRegister { register: u8, value: u8 },
    /// 7XNN
    AddRegister { register: u8, value: u8 },
    /// ANNN
    SetIndexRegister(u16),
    /// DXYN
    Display {
        // X
        x_coord_register: u8,
        // Y
        y_coord_register: u8,
        // N
        sprite_height:    u8,
    },
}

impl TryFrom<u16> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0x00E0 {
            return Ok(Self::ClearScreen);
        }

        if value == 0x00EE {
            return Ok(Self::Return);
        }

        if value & 0xF000 == 0x1000 {
            return Ok(Self::Goto(value & 0x0FFF));
        }

        if value & 0xF000 == 0x2000 {
            return Ok(Self::Subroutine(value & 0x0FFF));
        }

        if value & 0xF000 == 0xA000 {
            return Ok(Self::SetIndexRegister(value & 0x0FFF));
        }

        if value & 0xF000 == 0xD000 {
            // Size of a nibble
            const SHIFT_LENGTH: u16 = 4;
            const MASK: u16 = 0xF;

            // Bits 8-11
            let x_coord_register = ((value >> (SHIFT_LENGTH * 2)) & MASK) as u8;
            // Bits 4-7
            let y_coord_register = ((value >> SHIFT_LENGTH) & MASK) as u8;
            // Bits 0-3
            let sprite_height = (value & MASK) as u8;

            return Ok(Self::Display {
                x_coord_register,
                y_coord_register,
                sprite_height,
            });
        }

        bail!("Failed to parse instruction")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;

    #[test]
    fn test_parse_display_instruction() {
        let val: u16 = 0xD654;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(
            instruction,
            Instruction::Display {
                x_coord_register: 6,
                y_coord_register: 5,
                sprite_height:    4,
            }
        );
    }

    #[test]
    fn test_parse_goto_instruction() {
        let val: u16 = 0x1736;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(instruction, Instruction::Goto(0x736));
    }
}
