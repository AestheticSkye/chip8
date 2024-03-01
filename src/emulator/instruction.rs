use anyhow::{bail, Ok};

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
    /// 3XNN
    IsEqualVal { register: u8, value: u8 },
    /// 4XNN
    NotEqualVal { register: u8, value: u8 },
    /// 5XY0
    IsEqual { register_x: u8, register_y: u8 },
    /// 6XNN
    SetVal { register: u8, value: u8 },
    /// 7XNN
    AddVal { register: u8, value: u8 },
    /// 8XY0
    Set { register_x: u8, register_y: u8 },
    /// 8XY1
    Or { register_x: u8, register_y: u8 },
    /// 8XY2
    And { register_x: u8, register_y: u8 },
    /// 8XY3
    Xor { register_x: u8, register_y: u8 },
    /// 8XY4
    Add { register_x: u8, register_y: u8 },
    /// 8XY5
    SubtractRight { register_x: u8, register_y: u8 },
    /// 8XY6
    ShiftLeft { register_x: u8, register_y: u8 },
    /// 8XY7
    SubtractLeft { register_x: u8, register_y: u8 },
    /// 8XYE
    ShiftRight { register_x: u8, register_y: u8 },
    /// 9XY0
    NotEqual { register_x: u8, register_y: u8 },
    /// ANNN
    SetIndexRegister(u16),
    /// CXNN
    Rand { register: u8, value: u8 },
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

    #[allow(clippy::too_many_lines)]
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

        if value & 0xF000 == 0x3000 {
            return Ok(Self::IsEqualVal {
                register: ((value >> 8) & 0xF) as u8,
                value:    (value & 0xFF) as u8,
            });
        }

        if value & 0xF000 == 0x4000 {
            return Ok(Self::NotEqualVal {
                register: ((value >> 8) & 0xF) as u8,
                value:    (value & 0xFF) as u8,
            });
        }

        if value & 0xF00F == 0x5000 {
            return Ok(Self::IsEqual {
                register_x: ((value >> 8) & 0xF) as u8,
                register_y: ((value >> 4) & 0xF) as u8,
            });
        }

        if value & 0xF000 == 0x6000 {
            return Ok(Self::SetVal {
                register: ((value >> 8) & 0xF) as u8,
                value:    (value & 0xFF) as u8,
            });
        }

        if value & 0xF000 == 0x7000 {
            return Ok(Self::AddVal {
                register: ((value >> 8) & 0xF) as u8,
                value:    (value & 0xFF) as u8,
            });
        }

        if value & 0xF000 == 0x8000 {
            if let Some(instruction) = Self::parse_8xxx(value) {
                return Ok(instruction);
            }
        }

        if value & 0xF00F == 0x9000 {
            return Ok(Self::NotEqual {
                register_x: ((value >> 8) & 0xF) as u8,
                register_y: ((value >> 4) & 0xF) as u8,
            });
        }

        if value & 0xF000 == 0xA000 {
            return Ok(Self::SetIndexRegister(value & 0x0FFF));
        }

        if value & 0xF000 == 0xC000 {
            return Ok(Self::Rand {
                register: ((value >> 8) & 0xF) as u8,
                value:    (value & 0xFF) as u8,
            });
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

        loop {}
        bail!("Failed to parse instruction: {:x}", value)
    }
}

impl Instruction {
    const fn parse_8xxx(value: u16) -> Option<Self> {
        let instruction = ((value) & 0xF) as u8;
        let register_x = ((value >> 8) & 0xF) as u8;
        let register_y = ((value >> 4) & 0xF) as u8;

        match instruction {
            0 => Some(Self::Set {
                register_x,
                register_y,
            }),
            1 => Some(Self::Or {
                register_x,
                register_y,
            }),
            2 => Some(Self::And {
                register_x,
                register_y,
            }),
            3 => Some(Self::Xor {
                register_x,
                register_y,
            }),
            4 => Some(Self::Add {
                register_x,
                register_y,
            }),
            5 => Some(Self::SubtractRight {
                register_x,
                register_y,
            }),
            6 => Some(Self::ShiftLeft {
                register_x,
                register_y,
            }),
            7 => Some(Self::SubtractLeft {
                register_x,
                register_y,
            }),
            0xE => Some(Self::ShiftRight {
                register_x,
                register_y,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;

    #[test]
    fn test_parse_display() {
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
    fn test_parse_goto() {
        let val: u16 = 0x1736;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(instruction, Instruction::Goto(0x736));
    }

    #[test]
    fn test_parse_set_register() {
        let val: u16 = 0x6736;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(
            instruction,
            Instruction::SetVal {
                register: 0x7,
                value:    0x36,
            }
        );
    }

    #[test]
    fn test_parse_is_equal() {
        let val: u16 = 0x5730;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(
            instruction,
            Instruction::IsEqual {
                register_x: 0x7,
                register_y: 0x3,
            }
        );
    }

    #[test]
    fn test_parse_8xxx() {
        let val = 0x8760;

        let instruction: Instruction = val.try_into().unwrap();

        assert_eq!(
            instruction,
            Instruction::Set {
                register_x: 0x7,
                register_y: 0x6,
            }
        );
    }
}
