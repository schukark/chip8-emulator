use super::*;

#[test_context(Context)]
#[test]
fn test_skip_if_pressed_stays(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 4;

    let instr = Instruction::KeyPressedSkip { x: V0 };
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
}

#[test_context(Context)]
#[test]
fn test_skip_if_pressed_skips(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 4;
    ctx.chip8.set_key_state(0x4, true).unwrap();

    let instr = Instruction::KeyPressedSkip { x: V0 };
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Skip)));
}

#[test_context(Context)]
#[test]
fn test_skip_if_released_stays(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 4;

    let instr = Instruction::KeyReleasedSkip { x: V0 };
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Skip)));
}

#[test_context(Context)]
#[test]
fn test_skip_if_released_skips(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 4;
    ctx.chip8.set_key_state(0x4, true).unwrap();

    let instr = Instruction::KeyReleasedSkip { x: V0 };
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
}

#[test_context(Context)]
#[test]
fn test_await_key_ress(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 4;

    let instr = Instruction::AwaitKeyPress { x: V0 };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Wait)));
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Wait)));
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Wait)));

    ctx.chip8.set_key_state(0x4, true).unwrap();
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
}
