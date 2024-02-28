use anyhow::Result;

/// Allow the emulator to be switched out with different UIs
pub trait Draw {
    /// Draw the emulators bitmap grid onto a screen.
    ///
    /// Each item in buffer represents a pixel going from left to right, top to bottom.
    /// `true` is active, `false` is not.
    ///
    /// The screens buffer should be 64 pixels tall and 32 pixels wide.
    fn draw(&mut self, buffer: &[[bool; 64]; 32]) -> Result<()>;
}
