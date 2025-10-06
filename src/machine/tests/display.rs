use crate::types::SpriteHeight;

use super::*;
use ndarray::{ArrayView2, array, s};

#[test_context(Context)]
#[test]
fn test_load_digit_address(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 7;

    let instr = Instruction::SetSpriteAddr { x: V0 };
    ctx.chip8.execute(instr).unwrap();

    assert_eq!(ctx.chip8.cpu.address(), 7 * 5);
}

fn load_digit(chip8: &mut Chip8, digit: u8, start: u8) {
    *chip8.cpu.vx(V0) = digit;

    let instr = Instruction::SetSpriteAddr { x: V0 };
    chip8.execute(instr).unwrap();

    *chip8.cpu.vx(V0) = start;

    let instr = Instruction::DrawSprite {
        x: V0,
        y: V0,
        height: SpriteHeight::try_new(5).unwrap(),
    };

    chip8.execute(instr).unwrap();
}

#[test_context(Context)]
#[test]
fn test_display_sprite_no_collision(ctx: &mut Context) {
    load_digit(&mut ctx.chip8, 0xA, 0x0);

    assert!(ctx.chip8.dirty_flag);
    let display_state = ctx.chip8.display_snapshot().unwrap();
    let display_state = ArrayView2::from(display_state);
    let display_state = display_state.slice(s![0..5, 0..4]);

    let expected = array![
        [true, true, true, true],
        [true, false, false, true],
        [true, true, true, true],
        [true, false, false, true],
        [true, false, false, true]
    ];

    assert_eq!(display_state, expected);
    assert_eq!(*ctx.chip8.cpu.vx(VF), 0);
}

#[test_context(Context)]
#[test]
fn test_display_sprite_with_collision(ctx: &mut Context) {
    load_digit(&mut ctx.chip8, 0xA, 0x0);
    load_digit(&mut ctx.chip8, 0xA, 0x1);

    assert!(ctx.chip8.dirty_flag);
    assert_eq!(*ctx.chip8.cpu.vx(VF), 1);
}

#[test_context(Context)]
#[test]
fn test_clear_display(ctx: &mut Context) {
    load_digit(&mut ctx.chip8, 0xA, 0x0);

    let instr = Instruction::ClearDisplay;
    ctx.chip8.execute(instr).unwrap();

    let display_state = ctx.chip8.display_snapshot().unwrap();

    assert_eq!(display_state, &[[false; 64]; 32]);
}

#[test_context(Context)]
#[test]
fn test_no_rerender(ctx: &mut Context) {
    load_digit(&mut ctx.chip8, 0xA, 0x0);

    assert!(ctx.chip8.dirty_flag);
    let _ = ctx.chip8.display_snapshot();

    assert!(!ctx.chip8.dirty_flag);
    assert!(ctx.chip8.display_snapshot().is_none());
}
