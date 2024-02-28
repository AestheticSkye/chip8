pub mod scale;

use std::path::PathBuf;

use clap::Parser;

use self::scale::Scale;

#[derive(Parser, Debug)]
pub struct Arguments {
    /// Path for the interpreters executable.
    pub binary_path: PathBuf,

    /// Color the UIs background, defaults to black. Accepts labelled colour or rgb string, such as
    /// #736 or #429278
    #[arg(short, long)]
    pub background_color: Option<String>,

    /// Color the UIs foreground, defaults to white. Accepts labelled colour or rgb string, such as
    /// #736 or #429278
    #[arg(short, long)]
    pub foreground_color: Option<String>,

    /// Scale for the emulators UI, defaults to x4.
    #[arg(short, long)]
    pub scale: Option<Scale>,
}
