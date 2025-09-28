//! Chip8 keypad implementation

use thiserror::Error;

/// Hex keypad (0-F)
pub struct Keypad {
    /// Keypad keys
    /// on - pressed
    /// off - released
    pub keys: [bool; 16],
}

#[derive(Debug, Error)]
#[must_use]
/// Enum for all possible keypad errors
pub enum KeypadError {}

impl Keypad {
    /// Create a new keypad
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}
