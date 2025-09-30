use super::*;

const V0: Index = unsafe { Index::new_unchecked(0) };
const V4: Index = unsafe { Index::new_unchecked(4) };

#[test]
fn test_eq_const_skips_when_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 42;

    let instr = Instruction::EqConst { x: V0, value: 42 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x202);
}

#[test]
fn test_eq_const_stays_when_not_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0;

    let instr = Instruction::EqConst { x: V0, value: 42 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x200);
}

#[test]
fn test_neq_const_stays_when_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 42;

    let instr = Instruction::NeqConst { x: V0, value: 42 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x200);
}

#[test]
fn test_neq_const_skips_when_not_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0;

    let instr = Instruction::NeqConst { x: V0, value: 42 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x202);
}

#[test]
fn test_eq_reg_skips_when_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 42;
    *chip8.cpu.vx(V4) = 42;

    let instr = Instruction::EqReg { x: V0, y: V4 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x202);
}

#[test]
fn test_eq_reg_stays_when_not_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0;
    *chip8.cpu.vx(V4) = 42;

    let instr = Instruction::EqReg { x: V0, y: V4 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x200);
}

#[test]
fn test_neq_reg_stays_when_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 42;
    *chip8.cpu.vx(V4) = 42;

    let instr = Instruction::NeqReg { x: V0, y: V4 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x200);
}

#[test]
fn test_neq_reg_skips_when_not_equal() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V0) = 0;
    *chip8.cpu.vx(V4) = 42;

    let instr = Instruction::NeqReg { x: V0, y: V4 };

    chip8.execute(instr).unwrap();
    assert_eq!(chip8.cpu.program_counter(), 0x202);
}
