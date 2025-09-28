//! Chip8 cpu implementation

use crate::types::Index;

use thiserror::Error;

/// Store register and stack state\
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cpu {
    /// General purpose registers, V0 through VF, 8 bit each
    general: [u8; 16],
    /// Address register
    address: u16,
    /// Delay timer
    delay_timer: u8,
    /// Sound timer
    sound_timer: u8,
    /// Program counter
    program_counter: u16,
    /// Stack pointer
    stack_pointer: u16,
    /// Stack
    stack: [u16; 16],
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

/// Enum of possible CPU errors
#[derive(Debug, Error)]
#[must_use]
pub enum CpuError {
    #[error("Stack limit of 16 was reached")]
    /// Stack is limited to 16 entries
    StackLimitReached,
    #[error("Stack is empty")]
    /// Stack is empty, but pop was executed
    StackEmpty,
    #[error("Address out of range")]
    /// Address can't be bigger than 2^12 = 4096
    AddressOutOfRange,
    #[error("Program counter out of range")]
    /// Program counter can't be bigger than 2^12 = 4096
    PCOutOfRange,
}

impl Cpu {
    /// Create a new CPU
    pub fn new() -> Self {
        Self {
            general: [0; 16],
            address: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200, // programs start at 0x200
            stack_pointer: 0,
            stack: [0; 16],
        }
    }

    /// Get program counter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Advance program counter
    ///
    /// Returns an error if the program counter doesn't fit into 12 bits
    pub fn advance_program_counter(&mut self, step: u16) -> Result<(), CpuError> {
        if self.program_counter + step >= 1 << 12 {
            Err(CpuError::PCOutOfRange)
        } else {
            self.program_counter += step;
            Ok(())
        }
    }

    /// Set program counter
    ///
    /// Returns an error if the program counter doesn't fit into 12 bits
    pub fn set_program_counter(&mut self, new_pc: u16) -> Result<(), CpuError> {
        if new_pc >= 1 << 12 {
            Err(CpuError::PCOutOfRange)
        } else {
            self.program_counter = new_pc;
            Ok(())
        }
    }

    /// Get address register (I)
    pub fn get_address(&self) -> u16 {
        self.address
    }

    /// Set a new address register value (I)
    ///
    /// Returns an error if the address doesn't fit into 12 bits
    pub fn set_address(&mut self, value: u16) -> Result<(), CpuError> {
        if value >= 1 << 12 {
            Err(CpuError::AddressOutOfRange)
        } else {
            self.address = value;
            Ok(())
        }
    }

    /// Increment address register by given value
    ///
    /// Returns an error if the address doesn't fit into 12 bits
    pub fn advance_address(&mut self, value: u16) -> Result<(), CpuError> {
        if value + self.address >= 1 << 12 {
            Err(CpuError::AddressOutOfRange)
        } else {
            self.address += value;
            Ok(())
        }
    }

    /// Push an addres onto stack, updating stack pointer accordingly
    ///
    /// Returns an error if the stack limit is reached
    pub fn stack_push(&mut self, address: u16) -> Result<(), CpuError> {
        if self.stack_pointer == 16 {
            return Err(CpuError::StackLimitReached);
        }

        self.stack[self.stack_pointer as usize] = address;
        self.stack_pointer += 1;

        Ok(())
    }

    /// Pop an address from the stack, returning it
    ///
    /// Returns an error if was called on an empty stack
    pub fn stack_pop(&mut self) -> Result<u16, CpuError> {
        if self.stack_pointer == 0 {
            return Err(CpuError::StackEmpty);
        }

        let result = self.stack[self.stack_pointer as usize - 1];
        self.stack_pointer -= 1;

        Ok(result)
    }

    /// Helper to get the VX register, reduced boilerplate
    pub fn vx(&mut self, x: Index) -> &mut u8 {
        &mut self.general[x.into_inner() as usize]
    }

    /// Get delay timer value
    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    /// Set delay timer
    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    /// Set sound timer
    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }
}
