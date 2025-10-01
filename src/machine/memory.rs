//! Chip8 memory implementation

use thiserror::Error;

/// Chip8 ram struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    /// Bytes in memory
    data: [u8; 4096],
}

use tklog::{error, trace};

/// Predefined sprites for all hex digits
const DIGIT_SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // ZERO
    [0x20, 0x60, 0x20, 0x20, 0x70], // ONE
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // TWO
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // THREE
    [0x90, 0x90, 0xF0, 0x10, 0x10], // FOUR
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // FIVE
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // SIX
    [0xF0, 0x10, 0x20, 0x40, 0x40], // SEVEN
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // EIGHT
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // NINE
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
    #[error("Memory access out of range: {0:#X}")]
    /// Access of memory beyond 12 bit of addresses available
    OutOfRange(u16),
}

impl Memory {
    /// Create a ram filled with digit sprites (0x000 - 0x1FF)
    pub fn new() -> Self {
        let mut data = [0; 4096];

        for (number, constant) in DIGIT_SPRITES.iter().enumerate() {
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
            error!(
                "Incorrect digit sprite asked, should be <= 0xF, was ",
                digit
            );
            Err(MemoryError::IncorrectSprite)
        } else {
            trace!("Found digit ", digit, " sprite at ", digit as u16 * 5);
            Ok(digit as u16 * 5)
        }
    }

    /// Fetch a byte from address
    pub fn read_byte(&self, addr: u16) -> Result<u8, MemoryError> {
        if addr >= 1 << 12 {
            error!("Got incorrect address, should be below 4096, was ", addr);
            return Err(MemoryError::OutOfRange(addr));
        }
        trace!(
            "Read byte ",
            self.data[addr as usize], " from address ", addr
        );
        Ok(self.data[addr as usize])
    }

    /// Fetch a 2-byte word from address
    pub fn read_word(&self, addr: u16) -> Result<u16, MemoryError> {
        if addr >= (1 << 12) - 1 {
            error!("Got incorrect address, should be below 4095, was ", addr);
            return Err(MemoryError::OutOfRange(addr));
        }
        let hi = self.data[addr as usize] as u16;
        let lo = self.data[(addr + 1) as usize] as u16;
        trace!("Read word ", (hi << 8) | lo, " from address ", addr);
        Ok((hi << 8) | lo)
    }

    /// Load bytes into ram starting from given address
    pub fn load(&mut self, start: u16, bytes: &[u8]) -> Result<(), MemoryError> {
        if start < 0x200 {
            error!(
                "Requested reserved memory, addresses should be at leas 512, got ",
                start
            );
            return Err(MemoryError::PermissionDenied);
        }

        if start as usize + bytes.len() > self.data.len() {
            error!(
                "Invalid memory request, address should be below 4096, got ",
                start + bytes.len() as u16
            );
            return Err(MemoryError::OutOfRange(start + bytes.len() as u16));
        }

        self.data[start as usize..start as usize + bytes.len()].copy_from_slice(bytes);
        trace!(format!(
            "Wrote {bytes:?} to memory from {start} to {}",
            start + bytes.len() as u16
        ));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_incorrect_sprite_index() {
        let memory = Memory::new();

        assert!(matches!(
            memory.read_sprite_address(0xF + 2),
            Err(MemoryError::IncorrectSprite)
        ));
    }

    #[test_case(0x0 ; "lowest sprite")]
    #[test_case(0xF ; "highest sprite")]
    #[test_case(0x8 ; "random sprite")]
    fn test_correct_sprite_indexes(index: u8) {
        let memory = Memory::new();

        // 5 rows for each digit
        assert_eq!(memory.read_sprite_address(index).unwrap(), index as u16 * 5);
    }

    #[test]
    fn test_read_byte_out_of_range() {
        let memory = Memory::new();

        assert!(matches!(
            memory.read_byte(1 << 12),
            Err(MemoryError::OutOfRange(_))
        ));
    }

    #[test_case(0x200 ; "lowest address")]
    #[test_case(0xFFF ; "highest address")]
    #[test_case(0xB42 ; "random address")]
    fn test_read_byte_correct(addr: u16) {
        let memory = Memory::new();

        assert_eq!(memory.read_byte(addr).unwrap(), 0);
    }

    #[test]
    fn test_read_word_out_of_range() {
        let memory = Memory::new();

        assert!(matches!(
            memory.read_word(1 << 12),
            Err(MemoryError::OutOfRange(_))
        ));

        // edge case when the hi in word is in memory, but lo is outside of it
        assert!(matches!(
            memory.read_word((1 << 12) - 1),
            Err(MemoryError::OutOfRange(_))
        ));
    }

    #[test_case(0x200 ; "lowest address")]
    #[test_case(0xFFE ; "highest address")] // 0xFFF - 1 because we read 2 bytes at a time
    #[test_case(0xB42 ; "random address")]
    fn test_read_word_correct(addr: u16) {
        let memory = Memory::new();

        assert_eq!(memory.read_word(addr).unwrap(), 0);
    }

    #[test]
    fn test_load_permission_denied() {
        let mut memory = Memory::new();
        assert!(matches!(
            memory.load(0x100, &[0xAA]),
            Err(MemoryError::PermissionDenied)
        ));
    }

    #[test]
    fn test_load_out_of_range() {
        let mut memory = Memory::new();
        assert!(matches!(
            memory.load(0xFFF, &[0xAA, 0xBB]),
            Err(MemoryError::OutOfRange(_))
        ));
    }

    #[test]
    fn test_read_write_cycle() {
        let mut memory = Memory::new();
        memory.load(0x200, &[0xAB, 0xCD]).unwrap();
        assert_eq!(memory.read_byte(0x200).unwrap(), 0xAB);
        assert_eq!(memory.read_word(0x200).unwrap(), 0xABCD);
    }
}
