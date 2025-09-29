//! Newtypes to make the operations with nibbles and registers easier ans safer

use std::fmt;

use nutype::nutype;

/// Newtype struct for addresses, they are 12-bit in chip8
#[nutype(
    validate(less = 4096),
    derive(Debug, PartialEq, Eq, Clone, Display, Copy)
)]
pub struct Address(u16);

impl From<[u8; 3]> for Address {
    fn from(arr: [u8; 3]) -> Self {
        Address::try_new(((arr[0] as u16) << 8) | ((arr[1] as u16) << 4) | (arr[2] as u16)).unwrap()
    }
}

impl fmt::UpperHex for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::UpperHex::fmt(&self.into_inner(), f)
    }
}

/// Newtype struct for register number, they are 4-bit in chip8
///
/// new_unchecked is used to reduce boilerplate, VF is used in
/// 1/3 of instructions and stores as an associated constant to type Chip8
#[nutype(
    const_fn,
    new_unchecked,
    derive(Debug, PartialEq, Eq, Clone, Display, Copy),
    validate(less = 16)
)]
pub struct Index(u8);

impl fmt::UpperHex for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::UpperHex::fmt(&self.into_inner(), f)
    }
}

/// Newtype struct for sprite height, they are 4-bit in chip8
#[nutype(
    derive(Debug, PartialEq, Eq, Clone, Display, Copy),
    validate(less = 16)
)]
pub struct SpriteHeight(u8);
