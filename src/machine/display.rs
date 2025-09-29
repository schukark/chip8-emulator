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

    /// Draw the sprite starting at (vx, vy)
    ///
    /// Returns true if any pixels were turned off by drawing this sprite
    pub fn draw_sprite(&mut self, sprite: &[u8], vx: u8, vy: u8) -> bool {
        let mut collision = false;

        for (idx, word) in sprite.iter().enumerate() {
            for i in 0..8 {
                let pixel = (word >> i) & 0x1;
                let pos_x = (vx as usize + i) % 64;
                let pos_y = (vy as usize + idx) % 32;
                if self.pixels[pos_y][pos_x] {
                    collision = true;
                }
                self.pixels[pos_y][pos_x] ^= pixel == 1;
            }
        }
        collision
    }
}
