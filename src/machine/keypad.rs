//! Chip8 keypad implementation

use thiserror::Error;
use tklog::{error, trace};

/// Hex keypad (0-F)
pub struct Keypad {
    /// Keypad state
    /// on - pressed
    /// off - released
    state: [bool; 16],
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
        Self { state: [false; 16] }
    }

    /// Sets the new key state (pressed = true/released = false)
    ///
    /// Returns an error if the key pressed doesn't exist (not in 0x0..=0xF)
    pub fn set_key_state(&mut self, key: u8, state: bool) -> Result<(), KeypadError> {
        if key > 0xF {
            error!("Invalid key press request ", key);
            return Err(KeypadError::NoSuchKey);
        }

        self.state[key as usize] = state;
        trace!("Set key ", key, " to state ", state);

        Ok(())
    }

    /// Query whether a key is pressed
    ///
    /// Returns the error if the key queried doesn't exist (not in 0x0..=0xF)
    pub fn is_pressed(&self, key: u8) -> Result<bool, KeypadError> {
        if key > 0xF {
            error!("Invalid is_pressed key request ", key);
            return Err(KeypadError::NoSuchKey);
        }
        trace!(
            "Checking if key ",
            key, " is pressed, result ", self.state[key as usize]
        );
        Ok(self.state[key as usize])
    }

    /// Returns the first key that is pressed right now
    pub fn any_pressed(&self) -> Option<u8> {
        trace!(format!("Current state: {:?}", self.state));
        self.state.iter().position(|&b| b).map(|i| i as u8)
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0x0 ; "lowest key")]
    #[test_case(0xF ; "highest key")]
    #[test_case(0xA ; "random key")]
    fn test_simple_press(key: u8) {
        let mut keypad = Keypad::new();
        keypad.set_key_state(key, true).unwrap();

        assert!(keypad.is_pressed(key).unwrap());
    }

    #[test]
    fn test_simple_press_release() {
        let mut keypad = Keypad::new();
        keypad.set_key_state(0xC, true).unwrap();
        keypad.set_key_state(0xD, true).unwrap();
        keypad.set_key_state(0xC, false).unwrap();

        assert!(!keypad.is_pressed(0xC).unwrap());
        assert!(keypad.is_pressed(0xD).unwrap());
    }

    #[test]
    fn test_invalid_key() {
        let mut keypad = Keypad::new();
        assert!(matches!(
            keypad.set_key_state(0x10, true),
            Err(KeypadError::NoSuchKey)
        ));
    }
}
