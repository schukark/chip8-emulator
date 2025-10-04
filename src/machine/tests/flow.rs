use super::*;

#[test]
fn test_goto_correct() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::Goto {
        address: Address::try_new(0x304).unwrap(),
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Jumped)));
    assert_eq!(chip8.cpu.program_counter(), 0x304);
}

#[test]
fn test_call_function_correct() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::CallSubroutine {
        address: Address::try_new(0x47A).unwrap(),
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Jumped)));
    assert_eq!(chip8.cpu.program_counter(), 0x47A);
    assert_eq!(chip8.cpu.stack_pop().unwrap(), 0x202);
}

#[test]
fn test_call_and_return_from_function() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::CallSubroutine {
        address: Address::try_new(0x47A).unwrap(),
    };

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Jumped)));
    assert_eq!(chip8.cpu.program_counter(), 0x47A);

    let instr = Instruction::Return;

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Jumped)));
    assert_eq!(chip8.cpu.program_counter(), 0x202);
}

#[test]
fn test_return_on_empty_stack() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::Return;

    assert!(matches!(
        chip8.execute(instr),
        Err(Chip8Error::CpuError(CpuError::StackEmpty))
    ));
}

#[test]
fn test_goto_plus_v0() {
    let mut chip8 = Chip8::new();
    let instr = Instruction::GotoPlusV0 {
        address: Address::try_new(0xAB0).unwrap(),
    };

    *chip8.cpu.vx(Index::try_new(0).unwrap()) = 0x1D;

    assert!(matches!(chip8.execute(instr), Ok(ExecResult::Jumped)));
    assert_eq!(chip8.cpu.program_counter(), 0xAB0 + 0x1D);
}
