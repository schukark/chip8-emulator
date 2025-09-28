//! Chip8 memory implementation

use thiserror::Error;

/// Chip8 ram struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory {
    /// Bytes in memory
    data: [u8; 4096],
}

#[derive(Debug, Error)]
#[must_use]
pub enum MemoryError {}

impl Memory {
    /// Create an empty ram
    pub fn new() -> Self {
        Self { data: [0; 4096] }
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
    pub fn load(&mut self, start: u16, bytes: &[u8]) {
        self.data[start as usize..start as usize + bytes.len()].copy_from_slice(bytes);
    }
}
