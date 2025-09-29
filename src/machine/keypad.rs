//! Chip8 keypad implementation

use thiserror::Error;

/// Hex keypad (0-F)
pub struct Keypad {
    /// Keypad keys
    /// on - pressed
    /// off - released
    pub keys: [bool; 16],
}

/// Enum for all possible keypad errors
#[derive(Debug, Error)]
#[must_use]
pub enum KeypadError {}

impl Keypad {
    /// Create a new keypad
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}
