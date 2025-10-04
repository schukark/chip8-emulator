use super::*;
use test_context::{TestContext, test_context};

struct Context {
    chip8: Chip8,
    index_x: Index,
    index_y: Index,
}
const VX: u8 = 0x74;
const VY: u8 = 0x3F;

impl TestContext for Context {
    fn setup() -> Self {
        let index_x = Index::try_new(0x4).unwrap();
        let index_y = Index::try_new(0xD).unwrap();
        let mut ctx = Self {
            chip8: Chip8::new(),
            index_x,
            index_y,
        };

        *ctx.chip8.cpu.vx(index_x) = VX;
        *ctx.chip8.cpu.vx(index_y) = VY;

        ctx
    }
}

#[test_context(Context)]
#[test]
fn test_add_assign_reg(ctx: &mut Context) {
    let instr = Instruction::AddAssignReg {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), VX + VY);
}

#[test_context(Context)]
#[test]
fn test_sub_assign_reg(ctx: &mut Context) {
    let instr = Instruction::SubAssignReg {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), VX - VY);
}

#[test_context(Context)]
#[test]
fn test_sub_assign_reg_inverse(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(ctx.index_x) = VY;
    *ctx.chip8.cpu.vx(ctx.index_y) = VX;

    let instr = Instruction::SubAssignRegInverse {
        x: ctx.index_x,
        y: ctx.index_y,
    };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(ctx.index_x), VX - VY);
}
