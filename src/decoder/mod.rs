//! Decoder module containing implementations of chip8 assembly and rom -> rust equivalent

pub mod instruction;
mod macros;
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests;
