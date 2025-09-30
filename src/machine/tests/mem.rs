use super::*;

const V7: Index = unsafe { Index::new_unchecked(7) };

#[test]
fn test_set_address() {
    let mut chip8 = Chip8::new();
    let addr = Address::try_new(0x3FB).unwrap();

    let instr = Instruction::SetI { address: addr };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(chip8.cpu.address(), 0x3FB);
}

#[test]
fn test_add_assign_address() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V7) = 0xBB;
    chip8.cpu.set_address(0x3FB).unwrap();

    let instr = Instruction::AddAssignAddress { x: V7 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(chip8.cpu.address(), 0x3FB + 0xBB);
}

#[test]
fn test_load_sprite() {
    let mut chip8 = Chip8::new();
    *chip8.cpu.vx(V7) = 0xD;

    let instr = Instruction::SetSpriteAddr { x: V7 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(chip8.cpu.address(), 0xD * 5);
}

#[test]
fn test_dump_registers() {
    let mut chip8 = Chip8::new();

    for i in 0..=7 {
        *chip8.cpu.vx(Index::try_new(i).unwrap()) = i * 2;
    }
    chip8.cpu.set_address(0x27F).unwrap();

    let instr = Instruction::DumpRegisters { x: V7 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(chip8.cpu.address(), 0x27F);

    for i in 0..=7 {
        assert_eq!(chip8.memory.read_byte(0x27F + i).unwrap(), i as u8 * 2);
    }
}

#[test]
fn test_load_registers() {
    let mut chip8 = Chip8::new();

    for i in 0..=7 {
        chip8.memory.load(0x27F + i, &[i as u8 * 2]).unwrap();
    }
    chip8.cpu.set_address(0x27F).unwrap();

    let instr = Instruction::LoadRegisters { x: V7 };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(chip8.cpu.address(), 0x27F);

    for i in 0..=7 {
        assert_eq!(*chip8.cpu.vx(Index::try_new(i).unwrap()), i * 2);
    }
}
