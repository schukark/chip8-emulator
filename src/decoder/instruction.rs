//! Complete list of opcodes for the chip8 emulator
use std::fmt::Display;

use thiserror::Error;

use crate::{
    decoder::macros::*,
    types::{Address, Index, SpriteHeight},
};

/// All the opcodes for chip8 emulator
///
/// NNN - 12-bit address
/// NN - 8-bit constant
/// N - 4-bit constant
/// X/Y - 4-bit register code (0-F)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// 0NNN, call machine code routine
    CallMachineCode {
        /// Address of the subroutine
        address: Address,
    },
    /// 00E0, clears the display
    ClearDisplay,
    /// 00EE, returns from the function
    Return,
    /// 1NNN, jump to address NNN
    Goto {
        /// address to jump to
        address: Address,
    },
    /// 2NNN, call subroutine at address NNN
    CallSubroutine {
        /// subroutine start address
        address: Address,
    },
    /// 3XNN, if VX == NN, skip next instruction
    EqConst {
        /// register index for VX
        x: Index,
        /// value to compare to
        value: u8,
    },
    /// 4XNN, if VX != NN, skip next instruction
    NeqConst {
        /// register index for VX
        x: Index,
        /// value to compare to
        value: u8,
    },
    /// 5XY0, if VX == VY, skip next instruction
    EqReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 6XNN, set VX to NN
    AssignConst {
        /// register index for VX
        x: Index,
        /// value to assign
        value: u8,
    },
    /// 7XNN, VX += NN
    AddAssignConst {
        /// register index for VX
        x: Index,
        /// value to assign
        value: u8,
    },
    /// 8XY0, set VX to VY
    AssignReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY1, VX |= VY
    OrReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY2, VX &= VY
    AndReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY3, VX ^= VY
    XorReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY4, VX += VY
    AddAssignReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY5, VX -= VY
    SubAssignReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY6, VX >>= 1
    RShift {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XY7, VX = VY - VX
    SubAssignRegInverse {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 8XYE, VX <<= 1
    LShift {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// 9XY0, if VX != VY, skip next instruction
    NeqReg {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
    },
    /// ANNN, set I to NNN
    SetI {
        /// address to set I to
        address: Address,
    },
    /// BNNN, jump to address NNN + V0
    GotoPlusV0 {
        /// address to jump to
        address: Address,
    },
    /// CXNN, VX = rand() & NN
    Rand {
        /// register index for VX
        x: Index,
        /// constant to and with
        value: u8,
    },
    /// DXYN, draw sprite at VX, VY with height being N
    DrawSprite {
        /// register index for VX
        x: Index,
        /// register index for VY
        y: Index,
        /// sprite height
        height: SpriteHeight,
    },
    /// EX9E, skip next instruction if key pressed matches with one stored in VX
    KeyPressedSkip {
        /// register index for VX
        x: Index,
    },
    /// EXA1, skip next instruction if key pressed doesn't match with one stored in VX
    KeyReleasedSkip {
        /// register index for VX
        x: Index,
    },
    /// FX07, set VX to delay timer value
    GetDelayTimer {
        /// register index for VX
        x: Index,
    },
    /// FX0A, wait until any key is pressed and store it to VX
    AwaitKeyPress {
        /// register index for VX
        x: Index,
    },
    /// FX15, set delay timer to VX
    SetDelayTimer {
        /// register index for VX
        x: Index,
    },
    /// FX18, set sound timer to VX
    SetSoundTimer {
        /// register index for VX
        x: Index,
    },
    /// FX1E, I += VX
    AddAssignAddress {
        /// register index for VX
        x: Index,
    },
    /// FX29, I = sprite_addr\[VX\]
    SetSpriteAddr {
        /// register index for VX
        x: Index,
    },
    /// FX33, store BCD of VX to I
    SetBCD {
        /// register index for VX
        x: Index,
    },
    /// FX55, dump register V0..=VX in memory starting at I
    DumpRegisters {
        /// register index for VX
        x: Index,
    },
    /// FX65, load register V0..=VX from memory starting at I
    LoadRegisters {
        /// register index for VX
        x: Index,
    },
}

/// Enum for all possible decode errors
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Command {0:#04X} is incorrect")]
    /// The command's bytes don't correspond to any correct instruction
    NoSuchInstruction(u16),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::CallMachineCode { address } => format!("SYS {address:#03X}"),
                Instruction::Return => String::from("RET"),
                Instruction::ClearDisplay => String::from("CLS"),
                Instruction::Goto { address } => format!("JP {address:#03X}"),
                Instruction::CallSubroutine { address } => format!("CALL {address:#03X}"),
                Instruction::EqConst { x, value } => format!("SE V{x:X}, {value:#02X}"),
                Instruction::NeqConst { x, value } => format!("SNE V{x:X},{value:#02X}"),
                Instruction::EqReg { x, y } => format!("SE V{x:X}, VV{y:X}"),
                Instruction::AssignConst { x, value } => format!("LD V{x:X}, {value:#02X}"),
                Instruction::AddAssignConst { x, value } => format!("ADD V{x:X}, {value:#02X}"),
                Instruction::AssignReg { x, y } => format!("LD V{x:X}, V{y:X}"),
                Instruction::OrReg { x, y } => format!("OR V{x:X}, V{y:X}"),
                Instruction::AndReg { x, y } => format!("AND V{x:X}, V{y:X}"),
                Instruction::XorReg { x, y } => format!("XOR V{x:X}, V{y:X}"),
                Instruction::AddAssignReg { x, y } => format!("ADD V{x:X}, V{y:X}"),
                Instruction::SubAssignReg { x, y } => format!("SUB V{x:X}, V{y:X}"),
                Instruction::RShift { x, y: _y } => format!("SHR V{x:X}"),
                Instruction::SubAssignRegInverse { x, y } => format!("SUBN V{x:X}, V{y:X}"),
                Instruction::LShift { x, y: _y } => format!("SHL V{x:X}"),
                Instruction::NeqReg { x, y } => format!("SNE V{x:X}, V{y:X}"),
                Instruction::SetI { address } => format!("LD I, {address:#03X}"),
                Instruction::GotoPlusV0 { address } => format!("JP V0, {address:#03X}"),
                Instruction::Rand { x, value } => format!("RND V{x:X}, {value:#02X}"),
                Instruction::DrawSprite { x, y, height } => format!("DRW V{x:X}, V{y:X}, {height}"),
                Instruction::KeyPressedSkip { x } => format!("SKP V{x:X}"),
                Instruction::KeyReleasedSkip { x } => format!("SKNP V{x:X}"),
                Instruction::GetDelayTimer { x } => format!("LD V{x:X}, DT"),
                Instruction::AwaitKeyPress { x } => format!("LD V{x:X}, K"),
                Instruction::SetDelayTimer { x } => format!("LD DT, V{x:X}"),
                Instruction::SetSoundTimer { x } => format!("LD ST, V{x:X}"),
                Instruction::AddAssignAddress { x } => format!("ADD I, V{x:X}"),
                Instruction::SetSpriteAddr { x } => format!("LD F, V{x:X}"),
                Instruction::SetBCD { x } => format!("LD B, V{x:X}"),
                Instruction::DumpRegisters { x } => format!("LD [I], V{x:X}"),
                Instruction::LoadRegisters { x } => format!("LD V{x:X}, [I]"),
            }
        )
    }
}

impl TryFrom<u16> for Instruction {
    type Error = DecodeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let parts = [
            ((value >> 12) & 0xF) as u8,
            ((value >> 8) & 0xF) as u8,
            ((value >> 4) & 0xF) as u8,
            (value & 0xF) as u8,
        ];

        let instruction = match parts {
            [0, 0, 0xE, 0xE] => Instruction::Return,
            [0, 0, 0xE, 0] => Instruction::ClearDisplay,
            [0, n1, n2, n3] => op_addr!(CallMachineCode, n1, n2, n3),
            [1, n1, n2, n3] => op_addr!(Goto, n1, n2, n3),
            [2, n1, n2, n3] => op_addr!(CallSubroutine, n1, n2, n3),
            [3, x, n1, n2] => op_regconst!(EqConst, x, n1, n2),
            [4, x, n1, n2] => op_regconst!(NeqConst, x, n1, n2),
            [5, x, y, 0] => op_reg2!(EqReg, x, y),
            [6, x, n1, n2] => op_regconst!(AssignConst, x, n1, n2),
            [7, x, n1, n2] => op_regconst!(AddAssignConst, x, n1, n2),
            [8, x, y, 0] => op_reg2!(AssignReg, x, y),
            [8, x, y, 1] => op_reg2!(OrReg, x, y),
            [8, x, y, 2] => op_reg2!(AndReg, x, y),
            [8, x, y, 3] => op_reg2!(XorReg, x, y),
            [8, x, y, 4] => op_reg2!(AddAssignReg, x, y),
            [8, x, y, 5] => op_reg2!(SubAssignReg, x, y),
            [8, x, y, 6] => op_reg2!(RShift, x, y),
            [8, x, y, 7] => op_reg2!(SubAssignRegInverse, x, y),
            [8, x, y, 0xE] => op_reg2!(LShift, x, y),
            [9, x, y, 0] => op_reg2!(NeqReg, x, y),
            [0xA, n1, n2, n3] => op_addr!(SetI, n1, n2, n3),
            [0xB, n1, n2, n3] => op_addr!(GotoPlusV0, n1, n2, n3),
            [0xC, x, n1, n2] => op_regconst!(Rand, x, n1, n2),
            [0xD, x, y, n] => op_reg3!(DrawSprite, x, y, n),
            [0xE, x, 9, 0xE] => op_reg1!(KeyPressedSkip, x),
            [0xE, x, 0xA, 1] => op_reg1!(KeyReleasedSkip, x),
            [0xF, x, 0, 7] => op_reg1!(GetDelayTimer, x),
            [0xF, x, 0, 0xA] => op_reg1!(AwaitKeyPress, x),
            [0xF, x, 1, 5] => op_reg1!(SetDelayTimer, x),
            [0xF, x, 1, 8] => op_reg1!(SetSoundTimer, x),
            [0xF, x, 1, 0xE] => op_reg1!(AddAssignAddress, x),
            [0xF, x, 2, 9] => op_reg1!(SetSpriteAddr, x),
            [0xF, x, 3, 3] => op_reg1!(SetBCD, x),
            [0xF, x, 5, 5] => op_reg1!(DumpRegisters, x),
            [0xF, x, 6, 5] => op_reg1!(LoadRegisters, x),
            _ => {
                return Err(DecodeError::NoSuchInstruction(value));
            }
        };

        Ok(instruction)
    }
}
