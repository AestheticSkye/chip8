pub mod compatability_mode;
pub mod scale;

use std::path::PathBuf;

use clap::{command, Parser};

use self::compatability_mode::CompatabilityMode;
use self::scale::Scale;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Path for the interpreters executable.
    pub binary_path: PathBuf,

    /// Color the UIs background. Accepts labelled colour or rgb string, such as
    /// #736 or #429278
    #[arg(short, long, default_value = "black")]
    pub background_color: String,

    /// Color the UIs foreground. Accepts labelled colour or rgb string, such as
    /// #736 or #429278
    #[arg(short, long, default_value = "white")]
    pub foreground_color: String,

    /// Scale for the emulators UI.
    #[arg(short, long, default_value = "x8")]
    pub scale: Option<Scale>,

    /// Compatability for SUPER-CHIP programs.
    #[arg(short, long, default_value = "cosmac")]
    pub compatability_mode: CompatabilityMode,
}
