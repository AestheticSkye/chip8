use anyhow::{bail, Error, Result};
use hex_color::HexColor;

#[allow(clippy::module_name_repetitions)]
pub fn parse_color(color: &str) -> Result<HexColor> {
    Ok(match color.to_lowercase().as_str() {
        "black" => HexColor::BLACK,
        "blue" => HexColor::BLUE,
        "cyan" => HexColor::CYAN,
        "grey" | "gray" => HexColor::GRAY,
        "green" => HexColor::GREEN,
        "magenta" | "pink" => HexColor::MAGENTA,
        "red" => HexColor::RED,
        "white" => HexColor::WHITE,
        "yellow" => HexColor::YELLOW,
        color => {
            let Ok(color) = HexColor::parse(color) else {
                bail!("Failed to parse color: `{color}`")
            };
            color
        }
    })
}
