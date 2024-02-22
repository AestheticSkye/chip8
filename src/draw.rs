/// Allow the emulator to be switched out with different UIs
pub trait Draw {
    type Error;

    fn draw(&mut self, buffer: &[bool]) -> Result<(), Self::Error>;
}
