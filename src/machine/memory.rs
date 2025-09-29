//! Chip8 memory implementation

use thiserror::Error;

/// Chip8 ram struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory {
    /// Bytes in memory
    data: [u8; 4096],
}

/// Predefined sprites for all hex digits
const DIGIT_PRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // ZERO
    [0x20, 0x60, 0x20, 0x20, 0x70], // ONE
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // TWO
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // THREE
    [0x90, 0x90, 0xF0, 0x10, 0x10], // FOUR
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // FIVE
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // SIX
    [0xF0, 0x10, 0x20, 0x40, 0x40], // SEVEN
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // EIGHT
    [0xF0, 0x00, 0xF0, 0x10, 0xF0], // NINE
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

/// Enum with variants encoding all memory-related errors
#[derive(Debug, Error)]
#[must_use]
pub enum MemoryError {
    #[error("Reserved memory access")]
    /// Addresses before 0x200 are reserved and can't be overwritten
    PermissionDenied,
    #[error("Sprite index out of range")]
    /// There are exactly 16 sprites (0..=F), accesing other indices is erroneous
    IncorrectSprite,
}

impl Memory {
    /// Create a ram filled with digit sprites (0x000 - 0x1FF)
    pub fn new() -> Self {
        let mut data = [0; 4096];

        for (number, constant) in DIGIT_PRITES.iter().enumerate() {
            for (i, byte) in constant.iter().enumerate() {
                data[number * 5 + i] = *byte;
            }
        }

        Self { data }
    }

    /// Fetch sprite address from reserved memory
    ///
    /// Returns error if the sprite asked is not in 0..=F
    pub fn read_sprite_address(&self, digit: u8) -> Result<u16, MemoryError> {
        if digit > 0xF {
            Err(MemoryError::IncorrectSprite)
        } else {
            Ok(digit as u16 * 5)
        }
    }

    /// Fetch a byte from address
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    /// Fetch a 2-byte word from address
    pub fn read_word(&self, addr: u16) -> u16 {
        let hi = self.data[addr as usize] as u16;
        let lo = self.data[(addr + 1) as usize] as u16;
        (hi << 8) | lo
    }

    /// Load bytes into ram starting from given address
    pub fn load(&mut self, start: u16, bytes: &[u8]) -> Result<(), MemoryError> {
        if start < 0x200 {
            return Err(MemoryError::PermissionDenied);
        }

        self.data[start as usize..start as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }
}
