use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    /// Path for the interpreters executable.
    pub binary_path: PathBuf,
}
