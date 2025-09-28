//! Chip8 display implementation

use thiserror::Error;

/// 64x32 monochrome screen
pub struct Display {
    /// 2D array of pixels
    /// true = on
    /// false = off
    pub pixels: [[bool; 64]; 32],
}

#[derive(Debug, Error)]
#[must_use]
/// Enum for all possible display errors
pub enum DisplayError {}

impl Display {
    /// Create an empty display
    pub fn new() -> Self {
        Self {
            pixels: [[false; 64]; 32],
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        self.pixels = [[false; 64]; 32];
    }
}
