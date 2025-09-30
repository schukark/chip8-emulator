//! Chip8 keypad implementation

use thiserror::Error;
use tklog::{error, trace};

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
            error!("Invalid key press request ", key);
            return Err(KeypadError::NoSuchKey);
        }

        self.keys[key as usize] = true;
        self.last_pressed = Some(key);
        trace!("Pressed key ", key);

        Ok(())
    }

    /// Release key
    ///
    /// Returns an error if the key pressed doesn't exist (not in 0x0..=0xF)
    pub fn release_key(&mut self, key: u8) -> Result<(), KeypadError> {
        if key > 0xF {
            error!("Invalid key release request ", key);
            return Err(KeypadError::NoSuchKey);
        }

        self.keys[key as usize] = false;
        trace!("Released key ", key);

        Ok(())
    }

    /// Query whether a key is pressed
    pub fn is_pressed(&self, key: u8) -> Result<bool, KeypadError> {
        if key > 0xF {
            error!("Invalid is_pressed key request ", key);
            return Err(KeypadError::NoSuchKey);
        }
        trace!(
            "Checking if key ",
            key, " is pressed, result ", self.keys[key as usize]
        );
        Ok(self.keys[key as usize])
    }

    /// Get the last pressed key
    ///
    /// Even if the key is later released, is still returned
    pub fn last_pressed(&self) -> Option<u8> {
        trace!(format!(
            "Checking what key was pressed last, result {:?}",
            self.last_pressed
        ));
        self.last_pressed
    }

    /// Clear last pressed key
    pub fn clear_last(&mut self) {
        trace!("Cleared the last pressed key info");
        self.last_pressed = None;
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
        keypad.press_key(key).unwrap();

        assert!(keypad.is_pressed(key).unwrap());
    }

    #[test]
    fn test_simple_press_release() {
        let mut keypad = Keypad::new();
        keypad.press_key(0xC).unwrap();
        keypad.press_key(0xD).unwrap();
        keypad.release_key(0xC).unwrap();

        assert!(!keypad.is_pressed(0xC).unwrap());
        assert!(keypad.is_pressed(0xD).unwrap());
    }

    #[test]
    fn test_last_pressed() {
        let mut keypad = Keypad::new();

        keypad.press_key(0x7).unwrap();
        assert_eq!(keypad.last_pressed().unwrap(), 0x7);

        keypad.press_key(0xF).unwrap();
        assert_eq!(keypad.last_pressed().unwrap(), 0xF);

        keypad.clear_last();
        assert!(keypad.last_pressed().is_none());
    }

    #[test]
    fn test_invalid_key() {
        let mut keypad = Keypad::new();
        assert!(matches!(
            keypad.press_key(0x10),
            Err(KeypadError::NoSuchKey)
        ));
    }
}
