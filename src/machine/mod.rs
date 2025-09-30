//! Chip8 machine
//!
//! Contains all components of chip8: registers, stack, display, timers and sounds

use thiserror::Error;

use crate::{
    decoder::instruction::{DecodeError, Instruction},
    machine::{
        cpu::{Cpu, CpuError},
        display::{Display, DisplayError},
        keypad::{Keypad, KeypadError},
        memory::{Memory, MemoryError},
    },
    types::Index,
};

mod cpu;
mod display;
mod keypad;
mod memory;

/// Enum of program counter statuses after command execution
enum ExecResult {
    /// PC should advance to next word
    Advance,
    /// Last instruction jumped, no need to move PC
    Jumped,
    /// Waiting on something, no need to move PC
    Wait,
}

/// Full Chip-8 machine
pub struct Chip8 {
    /// Chip8 cpu
    pub cpu: Cpu,
    /// Chip8 memory
    pub memory: Memory,
    /// Chip8 display
    pub display: Display,
    /// Chip8 keypad
    pub keypad: Keypad,
}

/// Enum of all possible errors with chip8 instance
#[derive(Debug, Error)]
pub enum Chip8Error {
    #[error("Cpu error")]
    /// Cpu error
    CpuError(#[from] CpuError),
    #[error("Memory error")]
    /// Memory error
    MemoryError(#[from] MemoryError),
    #[error("Display error")]
    /// Display error
    DisplayError(#[from] DisplayError),
    #[error("Ram error")]
    /// Keypad error
    KeypadError(#[from] KeypadError),
    #[error("Decode error")]
    /// Instruction decoding error
    DecodeError(#[from] DecodeError),
    #[error("Unsupported instruction")]
    /// Unsupported instruction called (assembly subroutines)
    UnsupportedInstruction,
}

impl Chip8 {
    /// VF register is used a lot here, so it is a predefined constant
    const VF: Index = unsafe { Index::new_unchecked(0xF) };

    /// Create a new Chip8
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    /// Load the program for execution, starting at address 0x200
    pub fn load_program(&mut self, program: &[u8]) -> Result<(), Chip8Error> {
        self.memory.load(0x200, program)?;
        self.cpu.set_program_counter(0x200)?;

        Ok(())
    }

    /// Run one fetch-decode-execute cycle
    pub fn step(&mut self) -> Result<(), Chip8Error> {
        let pc = self.cpu.program_counter();
        let opcode = self.memory.read_word(pc)?;
        let instruction = Instruction::try_from(opcode)?;

        match self.execute(instruction) {
            Ok(ExecResult::Advance) => self.cpu.advance_program_counter(2)?,
            Ok(ExecResult::Jumped) => {}
            Ok(ExecResult::Wait) => {}
            Err(err) => return Err(err),
        };

        Ok(())
    }

    /// Execute an instruction
    fn execute(&mut self, instruction: Instruction) -> Result<ExecResult, Chip8Error> {
        let result = match instruction {
            Instruction::CallMachineCode { address: _address } => {
                return Err(Chip8Error::UnsupportedInstruction);
            }
            Instruction::ClearDisplay => {
                self.display.clear();
                ExecResult::Advance
            }
            Instruction::Return => {
                let new_pc = self.cpu.stack_pop()?;
                self.cpu.set_program_counter(new_pc)?;
                ExecResult::Jumped
            }
            Instruction::Goto { address } => {
                self.cpu.set_program_counter(address.into_inner())?;
                ExecResult::Jumped
            }
            Instruction::CallSubroutine { address } => {
                self.cpu.stack_push(self.cpu.program_counter())?;
                self.cpu.set_program_counter(address.into_inner())?;
                ExecResult::Jumped
            }
            Instruction::EqConst { x, value } => {
                if *self.cpu.vx(x) == value {
                    self.cpu.advance_program_counter(2)?;
                }

                ExecResult::Advance
            }
            Instruction::NeqConst { x, value } => {
                if *self.cpu.vx(x) != value {
                    self.cpu.advance_program_counter(2)?;
                }
                ExecResult::Advance
            }
            Instruction::EqReg { x, y } => {
                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                if vx == vy {
                    self.cpu.advance_program_counter(2)?;
                }
                ExecResult::Advance
            }
            Instruction::AssignConst { x, value } => {
                *self.cpu.vx(x) = value;
                ExecResult::Advance
            }
            Instruction::AddAssignConst { x, value } => {
                *self.cpu.vx(x) += value;
                ExecResult::Advance
            }
            Instruction::AssignReg { x, y } => {
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) = vy;
                ExecResult::Advance
            }
            Instruction::OrReg { x, y } => {
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) |= vy;
                ExecResult::Advance
            }
            Instruction::AndReg { x, y } => {
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) &= vy;
                ExecResult::Advance
            }
            Instruction::XorReg { x, y } => {
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) ^= vy;
                ExecResult::Advance
            }
            Instruction::AddAssignReg { x, y } => {
                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) += vy;
                *self.cpu.vx(Chip8::VF) = ((vx as u16) + (vy as u16) > 255) as u8;
                ExecResult::Advance
            }
            Instruction::SubAssignReg { x, y } => {
                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) -= vy;
                *self.cpu.vx(Chip8::VF) = (vx >= vy) as u8;
                ExecResult::Advance
            }
            Instruction::RShift { x, y: _y } => {
                *self.cpu.vx(Chip8::VF) = *self.cpu.vx(x) & 1;
                *self.cpu.vx(x) >>= 1;
                ExecResult::Advance
            }
            Instruction::SubAssignRegInverse { x, y } => {
                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                *self.cpu.vx(x) = vy - vx;
                *self.cpu.vx(Chip8::VF) = (vy >= vx) as u8;
                ExecResult::Advance
            }
            Instruction::LShift { x, y: _y } => {
                *self.cpu.vx(Chip8::VF) = (*self.cpu.vx(x) >> 7) & 1;
                *self.cpu.vx(x) <<= 1;
                ExecResult::Advance
            }
            Instruction::NeqReg { x, y } => {
                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                if vx != vy {
                    self.cpu.advance_program_counter(2)?;
                }
                ExecResult::Advance
            }
            Instruction::SetI { address } => {
                self.cpu.set_address(address.into_inner())?;
                ExecResult::Advance
            }
            Instruction::GotoPlusV0 { address } => {
                let vx = *self.cpu.vx(Index::try_new(0x0_u8).unwrap());
                self.cpu
                    .set_program_counter(address.into_inner() + vx as u16)?;
                ExecResult::Jumped
            }
            Instruction::Rand { x, value } => {
                let rand = self.cpu.random();
                *self.cpu.vx(x) = rand & value;
                ExecResult::Advance
            }
            Instruction::DrawSprite { x, y, height } => {
                let mut sprite = vec![0_u8; height.into_inner() as usize];

                for i in 0..height.into_inner() {
                    sprite[i as usize] = self.memory.read_byte(self.cpu.address() + i as u16)?;
                }

                let vx = *self.cpu.vx(x);
                let vy = *self.cpu.vx(y);
                let collision = self.display.draw_sprite(&sprite, vx, vy)?;

                *self.cpu.vx(Chip8::VF) = collision as u8;

                ExecResult::Advance
            }
            Instruction::KeyPressedSkip { x: _ } => {
                unimplemented!("Keypad is not yet implemented")
            }
            Instruction::KeyReleasedSkip { x: _ } => {
                unimplemented!("Keypad is not yet implemented")
            }
            Instruction::GetDelayTimer { x } => {
                *self.cpu.vx(x) = self.cpu.delay_timer();
                ExecResult::Advance
            }
            Instruction::AwaitKeyPress { x: _ } => unimplemented!("Keypad is not yet implemented"),
            Instruction::SetDelayTimer { x } => {
                let vx = *self.cpu.vx(x);
                self.cpu.set_delay_timer(vx);
                ExecResult::Advance
            }
            Instruction::SetSoundTimer { x } => {
                let vx = *self.cpu.vx(x);
                self.cpu.set_sound_timer(vx);
                ExecResult::Advance
            }
            Instruction::AddAssignAddress { x } => {
                let vx = *self.cpu.vx(x);

                self.cpu.advance_address(vx as u16)?;
                ExecResult::Advance
            }
            Instruction::SetSpriteAddr { x } => {
                let digit = *self.cpu.vx(x);

                // & 0x0F guarantees that the sprite index is in required bounds (0..=F)
                let sprite_addr = self.memory.read_sprite_address(digit)?;
                self.cpu.set_address(sprite_addr)?;

                ExecResult::Advance
            }
            Instruction::SetBCD { x } => {
                let vx = *self.cpu.vx(x);

                let hundreds = vx / 100;
                let tens = (vx / 10) % 10;
                let ones = vx % 10;

                self.memory
                    .load(self.cpu.address(), &[hundreds, tens, ones])?;
                ExecResult::Advance
            }
            Instruction::DumpRegisters { x } => {
                for i in 0..=x.into_inner() {
                    self.memory.load(
                        self.cpu.address() + i as u16,
                        &[*self.cpu.vx(Index::try_new(i).unwrap())],
                    )?;
                }
                ExecResult::Advance
            }
            Instruction::LoadRegisters { x } => {
                for i in 0..=x.into_inner() {
                    *self.cpu.vx(Index::try_new(i).unwrap()) =
                        self.memory.read_byte(self.cpu.address() + i as u16)?;
                }
                ExecResult::Advance
            }
        };

        Ok(result)
    }
}
