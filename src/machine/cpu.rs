//! Chip8 cpu implementation

use crate::types::Index;

use rand::{Rng, SeedableRng, rng, rngs::SmallRng};
use thiserror::Error;

use tklog::{debug, error};

/// Store register and stack state\
#[derive(Debug, Clone, PartialEq, Eq)]
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
    stack_pointer: usize,
    /// Stack
    stack: [u16; 16],
    /// Random engine for reproducible randomness
    pub(crate) random_engine: SmallRng,
}

#[cfg_attr(coverage_nightly, coverage(off))]
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            general: [0; 16],
            address: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200, // programs start at 0x200
            stack_pointer: 0,
            stack: [0; 16],
            random_engine: SmallRng::from_rng(&mut rng()),
        }
    }

    /// Get program counter
    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Advance program counter
    ///
    /// Returns an error if the program counter doesn't fit into 12 bits
    pub fn advance_program_counter(&mut self, step: u16) -> Result<(), CpuError> {
        if self.program_counter + step >= 1 << 12 {
            error!("Program counter out of range!");
            Err(CpuError::PCOutOfRange)
        } else {
            self.program_counter += step;
            debug!(format!(
                "Program counter was advanced by {} to {}",
                step, self.program_counter
            ));
            Ok(())
        }
    }

    /// Set program counter
    ///
    /// Returns an error if the program counter doesn't fit into 12 bits
    pub fn set_program_counter(&mut self, new_pc: u16) -> Result<(), CpuError> {
        if new_pc >= 1 << 12 {
            error!("Program counter out of range!");
            Err(CpuError::PCOutOfRange)
        } else {
            self.program_counter = new_pc;
            debug!("Program counter was set to ", new_pc);
            Ok(())
        }
    }

    /// Get address register (I)
    pub fn address(&self) -> u16 {
        self.address
    }

    /// Set a new address register value (I)
    ///
    /// Returns an error if the address doesn't fit into 12 bits
    pub fn set_address(&mut self, value: u16) -> Result<(), CpuError> {
        if value >= 1 << 12 {
            error!("Address is too big for 12 bits!");
            Err(CpuError::AddressOutOfRange)
        } else {
            self.address = value;
            debug!("Address was set to ", value);
            Ok(())
        }
    }

    /// Increment address register by given step
    ///
    /// Returns an error if the address doesn't fit into 12 bits
    pub fn advance_address(&mut self, step: u16) -> Result<(), CpuError> {
        if step + self.address >= 1 << 12 {
            error!("Address is too big for 12 bits!");
            Err(CpuError::AddressOutOfRange)
        } else {
            debug!(format!(
                "Address register was advanced by {} to {}",
                step, self.address
            ));
            self.address += step;
            Ok(())
        }
    }

    /// Push an addres onto stack, updating stack pointer accordingly
    ///
    /// Returns an error if the stack limit is reached
    pub fn stack_push(&mut self, address: u16) -> Result<(), CpuError> {
        if self.stack_pointer == 16 {
            error!("Stack limit of 16 was reached!");
            return Err(CpuError::StackLimitReached);
        }

        self.stack[self.stack_pointer] = address;
        self.stack_pointer += 1;
        debug!("Put value ", address, " on top of the stack");

        Ok(())
    }

    /// Pop an address from the stack, returning it
    ///
    /// Returns an error if was called on an empty stack
    pub fn stack_pop(&mut self) -> Result<u16, CpuError> {
        if self.stack_pointer == 0 {
            error!("Pop was called on empty stack!");
            return Err(CpuError::StackEmpty);
        }

        let result = self.stack[self.stack_pointer - 1];
        self.stack_pointer -= 1;
        debug!("Removed value ", result, " from the stack");

        Ok(result)
    }

    /// Helper to get the VX register, reduced boilerplate
    pub fn vx(&mut self, x: Index) -> &mut u8 {
        &mut self.general[x.into_inner() as usize]
    }

    /// Get delay timer value
    pub fn delay_timer(&self) -> u8 {
        self.delay_timer
    }

    /// Get sound timer value
    pub fn sound_timer(&self) -> u8 {
        self.sound_timer
    }

    /// Set delay timer
    pub fn set_delay_timer(&mut self, value: u8) {
        debug!("Delay timer was set to ", value);
        self.delay_timer = value;
    }

    /// Set sound timer
    pub fn set_sound_timer(&mut self, value: u8) {
        debug!("Sound timer was set to ", value);
        self.sound_timer = value;
    }

    /// Get randomness
    pub fn random(&mut self) -> u8 {
        self.random_engine.random_range(0x0..=0xFF)
    }

    /// Tick timers down by one if possible
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            debug!("Delay timer ticked down to ", self.delay_timer);
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            debug!("Sound timer ticked down to ", self.sound_timer);
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::types::Index;

    #[test]
    fn test_new() {
        let cpu = Cpu::new();
        assert_eq!(cpu.program_counter(), 0x200);
        assert_eq!(cpu.address(), 0);
        assert_eq!(cpu.delay_timer(), 0);
        assert_eq!(cpu.stack_pointer, 0);
        assert_eq!(cpu.stack, [0; 16]);
    }

    mod program_counter {
        use crate::machine::cpu::{Cpu, CpuError};

        #[test]
        fn test_advance() {
            let mut cpu = Cpu::new();
            assert!(cpu.advance_program_counter(0x100).is_ok());
            assert_eq!(cpu.program_counter(), 0x300);
        }

        #[test]
        fn test_set() {
            let mut cpu = Cpu::new();

            assert!(cpu.set_program_counter(0x100).is_ok());
            assert_eq!(cpu.program_counter(), 0x100);
        }

        #[test]
        fn test_out_of_range() {
            let mut cpu = Cpu::new();
            assert!(matches!(
                cpu.set_program_counter(0x1000),
                Err(CpuError::PCOutOfRange)
            ));
        }
    }

    mod address {
        use crate::machine::cpu::{Cpu, CpuError};

        #[test]
        fn test_set() {
            let mut cpu = Cpu::new();
            assert!(cpu.set_address(0x100).is_ok());
            assert_eq!(cpu.address(), 0x100);
        }

        #[test]
        fn test_advance() {
            let mut cpu = Cpu::new();
            assert!(cpu.advance_address(0x100).is_ok());
            assert_eq!(cpu.address(), 0x100);
        }

        #[test]
        fn test_out_of_range() {
            let mut cpu = Cpu::new();
            assert!(matches!(
                cpu.set_address(0x1000),
                Err(CpuError::AddressOutOfRange)
            ));
        }
    }

    mod stack {
        use crate::machine::cpu::{Cpu, CpuError};

        #[test]
        fn test_correct_opeation() {
            let mut cpu = Cpu::new();

            assert!(cpu.stack_push(0x100).is_ok());
            assert_eq!(cpu.stack_pointer, 1);

            assert!(cpu.stack_push(0x200).is_ok());
            assert_eq!(cpu.stack_pointer, 2);
        }

        #[test]
        fn test_overflow() {
            let mut cpu = Cpu::new();

            for i in 0..16 {
                assert!(cpu.stack_push(i as u16).is_ok());
            }
            assert!(matches!(
                cpu.stack_push(0x100),
                Err(CpuError::StackLimitReached)
            ));
        }

        #[test]
        fn test_empty_error() {
            let mut cpu = Cpu::new();
            assert!(matches!(cpu.stack_pop(), Err(CpuError::StackEmpty)));
        }
    }

    #[test]
    fn test_registers() {
        let mut cpu = Cpu::new();
        let index = Index::try_new(0).unwrap();
        *cpu.vx(index) = 0x10;
        assert_eq!(cpu.general[0], 0x10);
    }

    #[test]
    fn test_timers() {
        let mut cpu = Cpu::new();
        cpu.set_delay_timer(0x10);
        assert_eq!(cpu.delay_timer(), 0x10);

        cpu.set_sound_timer(0x20);
        assert_eq!(cpu.sound_timer, 0x20);
    }

    #[test]
    fn test_reproducible_randomness() {
        let mut cpu1 = Cpu::new();
        cpu1.random_engine = SmallRng::seed_from_u64(42);

        let values = (0..10).map(|_| cpu1.random()).collect::<Vec<_>>();

        let mut cpu2 = Cpu::new();
        cpu2.random_engine = SmallRng::seed_from_u64(42);

        let values2 = (0..10).map(|_| cpu2.random()).collect::<Vec<_>>();

        assert_eq!(values, values2);
    }
}
