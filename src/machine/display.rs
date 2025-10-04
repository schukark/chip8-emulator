//! Chip8 display implementation

use thiserror::Error;
use tklog::{debug, error, trace};

/// 64x32 monochrome screen
pub struct Display {
    /// 2D array of pixels
    /// true = on
    /// false = off
    pixels: [[bool; 64]; 32],
}

/// Enum for all possible display errors
#[derive(Debug, Error)]
#[must_use]
pub enum DisplayError {
    #[error("Sprite height exceeds 32 in height")]
    /// The height of chip8 display is 32, so no sprite can be taller than it
    SpriteTooBig,
}

impl Display {
    /// Create an empty display
    pub fn new() -> Self {
        Self {
            pixels: [[false; 64]; 32],
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        debug!("The screen was cleared");
        self.pixels = [[false; 64]; 32];
    }

    /// Draw the sprite starting at (vx, vy)
    ///
    /// Returns true if any pixels were turned off by drawing this sprite
    pub fn draw_sprite(&mut self, sprite: &[u8], vx: u8, vy: u8) -> Result<bool, DisplayError> {
        if sprite.len() > 32 {
            error!(
                "DRW was called on sprite of length > 32, namely ",
                sprite.len()
            );
            return Err(DisplayError::SpriteTooBig);
        }

        let mut collision = false;

        for (idx, word) in sprite.iter().enumerate() {
            for i in 0..8 {
                let pixel = (word >> (7 - i)) & 0x1;
                let pos_x = (vx as usize + i) % 64;
                let pos_y = (vy as usize + idx) % 32;

                let new_pixel = self.pixels[pos_y][pos_x] ^ (pixel == 1);
                if self.pixels[pos_y][pos_x] && !new_pixel {
                    collision = true;
                    trace!("Found collision on x ", pos_x, ", y ", pos_y);
                }

                if self.pixels[pos_y][pos_x] != new_pixel {
                    trace!("Pixel at x ", pos_x, ", y ", pos_y, " changed it state");
                }

                self.pixels[pos_y][pos_x] = new_pixel;
            }
        }
        Ok(collision)
    }

    /// Get current display state
    pub fn state(&self) -> &[[bool; 64]; 32] {
        &self.pixels
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use ndarray::{ArrayView2, s};

    use super::*;

    #[test]
    fn test_simple_reactangle() {
        let mut display = Display::new();
        let sprite = [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000];

        let collisions = display.draw_sprite(&sprite, 0, 0).unwrap();
        let display_state = display.state();
        let result = ArrayView2::from(display_state);

        let expected = ndarray::array![
            [true, true, true, true],
            [true, false, false, true],
            [true, true, true, true],
            [true, false, false, true],
            [true, true, true, true],
        ];

        assert!(!collisions);
        assert!(result.slice(s![0..5, 0..4]).eq(&expected));
    }

    #[test]
    fn test_clear_display() {
        let mut display = Display::new();
        let sprite = [0b11110000, 0b10010000, 0b11110000, 0b10010000, 0b11110000];

        let collisions = display.draw_sprite(&sprite, 0, 0).unwrap();
        display.clear();
        let display_state = display.state();

        assert!(!collisions);
        assert_eq!(display_state, &[[false; 64]; 32]);
    }

    #[test]
    fn test_rectangle_intersection() {
        let mut display = Display::new();

        // sprite 5 in width, 4 in height, drawn from 0, 0
        // (0, 0) -> (4, 3)
        let sprite1 = [0b11111000, 0b11111000, 0b11111000, 0b11111000];
        // sprite 6 in width, 3 in height, drawn from 2, 2
        // (2, 2) -> (7, 4)
        let sprite2 = [0b11111100, 0b11111100, 0b11111100];

        let collisions1 = display.draw_sprite(&sprite1, 0, 0).unwrap();
        let collisions2 = display.draw_sprite(&sprite2, 2, 2).unwrap();
        let display_state = display.state();
        let result = ArrayView2::from(display_state);

        let expected = ndarray::array![
            [true, true, true, true, true, false, false, false],
            [true, true, true, true, true, false, false, false],
            [true, true, false, false, false, true, true, true],
            [true, true, false, false, false, true, true, true],
            [false, false, true, true, true, true, true, true],
        ];

        dbg!(result.slice(s![0..5, 0..8]));

        assert!(!collisions1);
        assert!(collisions2);
        assert!(result.slice(s![0..5, 0..8]).eq(&expected));
    }

    #[test]
    fn test_wraparound() {
        let mut display = Display::new();
        let sprite = [0b10000000];
        display.draw_sprite(&sprite, 63, 31).unwrap();
        assert!(display.state()[31][63]);
    }

    #[test]
    fn test_big_sprite_error() {
        let mut display = Display::new();

        let sprite = [0; 33];
        assert!(matches!(
            display.draw_sprite(&sprite, 0, 0),
            Err(DisplayError::SpriteTooBig)
        ));
    }
}
