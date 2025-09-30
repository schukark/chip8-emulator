use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::{machine::*, types::Address};

mod bitop;
mod cond;
mod flow;
mod mem;

const V0: Index = unsafe { Index::new_unchecked(0) };

#[test]
fn test_unsupported_instruction_error() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::CallMachineCode {
        address: Address::try_new(0xAB0).unwrap(),
    };

    assert!(matches!(
        chip8.execute(instr),
        Err(Chip8Error::UnsupportedInstruction)
    ));
}

#[test]
fn test_assign_const() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0;

    let instr = Instruction::AssignConst { x: V0, value: 42 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(V0), 42);
}

#[test]
fn test_add_assign_const() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0xF;

    let instr = Instruction::AddAssignConst { x: V0, value: 42 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(V0), 57);
}

#[test]
fn test_assign_reg() {
    let mut chip8 = Chip8::new();
    let index_y = Index::try_new(0xD).unwrap();
    *chip8.cpu.vx(V0) = 0;
    *chip8.cpu.vx(index_y) = 0xC4;

    let instr = Instruction::AssignReg { x: V0, y: index_y };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(V0), 0xC4);
}

#[test]
fn test_random() {
    let mut chip8 = Chip8::new();
    chip8.cpu.random_engine = SmallRng::seed_from_u64(42);

    let mut rng = SmallRng::seed_from_u64(42);

    let instr = Instruction::Rand { x: V0, value: 0xEA };
    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(V0), rng.random_range(0..=0xFF) & 0xEA);
}
