use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum Scale {
    FitScreen,
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
}

impl From<Scale> for minifb::Scale {
    fn from(val: Scale) -> Self {
        match val {
            Scale::FitScreen => Self::FitScreen,
            Scale::X1 => Self::X1,
            Scale::X2 => Self::X2,
            Scale::X4 => Self::X4,
            Scale::X8 => Self::X8,
            Scale::X16 => Self::X16,
            Scale::X32 => Self::X32,
        }
    }
}
