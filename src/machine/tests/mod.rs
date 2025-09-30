use crate::{machine::*, types::Address};

mod bitop;
mod cond;
mod flow;
mod mem;

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
    let index = Index::try_new(0).unwrap();
    *chip8.cpu.vx(index) = 0;

    let instr = Instruction::AssignConst {
        x: index,
        value: 42,
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(index), 42);
}

#[test]
fn test_add_assign_const() {
    let mut chip8 = Chip8::new();
    let index = Index::try_new(0).unwrap();
    *chip8.cpu.vx(index) = 0xF;

    let instr = Instruction::AddAssignConst {
        x: index,
        value: 42,
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(index), 57);
}

#[test]
fn test_assign_reg() {
    let mut chip8 = Chip8::new();
    let index_x = Index::try_new(0).unwrap();
    let index_y = Index::try_new(0xD).unwrap();
    *chip8.cpu.vx(index_x) = 0;
    *chip8.cpu.vx(index_y) = 0xC4;

    let instr = Instruction::AssignReg {
        x: index_x,
        y: index_y,
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*chip8.cpu.vx(index_x), 0xC4);
}
