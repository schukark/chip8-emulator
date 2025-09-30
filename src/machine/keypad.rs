//! Chip8 keypad implementation

use thiserror::Error;

/// Hex keypad (0-F)
pub struct Keypad {
    /// Keypad keys
    /// on - pressed
    /// off - released
    keys: [bool; 16],
    /// Holds the last pressed key
    ///
    /// Optional because it can be reset
    last_pressed: Option<u8>,
}

/// Enum for all possible keypad errors
#[derive(Debug, Error)]
#[must_use]
pub enum KeypadError {
    #[error("No such key")]
    /// Keys are 0-F (inclusive), so if keys >= 17 are being pressed, it is an error
    NoSuchKey,
}

impl Keypad {
    /// Create a new keypad
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            last_pressed: None,
        }
    }

    /// Press key
    ///
    /// Returns an error if the key pressed doesn't exist (not in 0x0..=0xF)
    pub fn press_key(&mut self, key: u8) -> Result<(), KeypadError> {
        if key > 0xF {
            return Err(KeypadError::NoSuchKey);
        }

        self.keys[key as usize] = true;
        self.last_pressed = Some(key);

        Ok(())
    }

    /// Release key
    ///
    /// Returns an error if the key pressed doesn't exist (not in 0x0..=0xF)
    pub fn release_key(&mut self, key: u8) -> Result<(), KeypadError> {
        if key > 0xF {
            return Err(KeypadError::NoSuchKey);
        }

        self.keys[key as usize] = false;

        Ok(())
    }

    /// Query whether a key is pressed
    pub fn is_pressed(&self, key: u8) -> Result<bool, KeypadError> {
        if key > 0xF {
            return Err(KeypadError::NoSuchKey);
        }
        Ok(self.keys[key as usize])
    }

    /// Get the last pressed key
    ///
    /// Even if the key is later released, is still returned
    pub fn last_pressed(&self) -> Option<u8> {
        self.last_pressed
    }

    /// Clear last pressed key
    pub fn clear_last(&mut self) {
        self.last_pressed = None;
    }
}
