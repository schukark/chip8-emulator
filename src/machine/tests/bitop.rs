use super::*;
use test_context::{TestContext, test_context};

struct Context {
    chip8: Chip8,
    index_x: Index,
    index_y: Index,
}

impl TestContext for Context {
    fn setup() -> Self {
        let index_x = Index::try_new(0x4).unwrap();
        let index_y = Index::try_new(0xD).unwrap();
        let mut ctx = Self {
            chip8: Chip8::new(),
            index_x,
            index_y,
        };

        *ctx.chip8.cpu.vx(index_x) = 0x4F;
        *ctx.chip8.cpu.vx(index_y) = 0xC4;

        ctx
    }
}

#[test_context(Context)]
#[test]
fn test_or_reg(ctx: &mut Context) {
    let instr = Instruction::OrReg {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0xC4 | 0x4F);
}

#[test_context(Context)]
#[test]
fn test_and_reg(ctx: &mut Context) {
    let instr = Instruction::AndReg {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0xC4 & 0x4F);
}

#[test_context(Context)]
#[test]
fn test_xor_reg(ctx: &mut Context) {
    let instr = Instruction::XorReg {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0xC4 ^ 0x4F);
}

#[test_context(Context)]
#[test]
fn test_rshift(ctx: &mut Context) {
    let instr = Instruction::RShift {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0x4F >> 1);
    assert_eq!(*ctx.chip8.cpu.vx(Chip8::VF), 1);
}

#[test_context(Context)]
#[test]
fn test_lshift(ctx: &mut Context) {
    let instr = Instruction::LShift {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0x4F << 1);
    assert_eq!(*ctx.chip8.cpu.vx(Chip8::VF), 0);

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), 0x4F << 2);
    assert_eq!(*ctx.chip8.cpu.vx(Chip8::VF), 1);
}
