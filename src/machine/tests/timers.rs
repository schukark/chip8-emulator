use super::*;

#[test_context(Context)]
#[test]
fn test_set_delay_timer(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 0x4A;
    let instr = Instruction::SetDelayTimer { x: V0 };

    ctx.chip8.execute(instr).unwrap();

    assert_eq!(*ctx.chip8.cpu.vx(V0), 0x4A);
    assert_eq!(ctx.chip8.cpu.delay_timer(), 0x4A);
}

#[test_context(Context)]
#[test]
fn test_set_sound_timer(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 0x4A;
    let instr = Instruction::SetSoundTimer { x: V0 };

    ctx.chip8.execute(instr).unwrap();

    assert_eq!(*ctx.chip8.cpu.vx(V0), 0x4A);
    assert_eq!(ctx.chip8.cpu.sound_timer(), 0x4A);
}

#[test_context(Context)]
#[test]
fn test_delay_timer_tick(ctx: &mut Context) {
    ctx.chip8.cpu.set_delay_timer(0x4A);
    let instr = Instruction::GetDelayTimer { x: V0 };
    ctx.chip8.execute(instr).unwrap();

    assert_eq!(*ctx.chip8.cpu.vx(V0), 0x4A);

    ctx.chip8.tick_timers();
    assert_eq!(ctx.chip8.cpu.delay_timer(), 0x49);
}

#[test_context(Context)]
#[test]
fn test_sound_timer_tick(ctx: &mut Context) {
    ctx.chip8.cpu.set_sound_timer(0x1);

    assert_eq!(ctx.chip8.cpu.sound_timer(), 0x1);
    assert!(ctx.chip8.is_sound_playing());

    ctx.chip8.tick_timers();
    assert_eq!(ctx.chip8.cpu.sound_timer(), 0x0);
    assert!(!ctx.chip8.is_sound_playing());
}
