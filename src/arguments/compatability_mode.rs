use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum CompatabilityMode {
    Cosmac,
    Super,
    Xo,
}
