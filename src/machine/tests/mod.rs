use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::{machine::*, types::Address};
use proptest::prelude::*;

use test_context::{TestContext, test_context};

mod bitop;
mod cond;
mod display;
mod flow;
mod keypad;
mod math;
mod mem;

const V0: Index = unsafe { Index::new_unchecked(0x0) };
const VF: Index = unsafe { Index::new_unchecked(0xF) };

struct Context {
    chip8: Chip8,
}

impl TestContext for Context {
    fn setup() -> Self {
        Self {
            chip8: Chip8::new(),
        }
    }
}

#[test_context(Context)]
#[test]
fn test_unsupported_instruction_error(ctx: &mut Context) {
    let instr = Instruction::CallMachineCode {
        address: Address::try_new(0xAB0).unwrap(),
    };

    assert!(matches!(
        ctx.chip8.execute(instr),
        Err(Chip8Error::UnsupportedInstruction)
    ));
}

#[test_context(Context)]
#[test]
fn test_assign_const(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 0;

    let instr = Instruction::AssignConst { x: V0, value: 42 };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(V0), 42);
}

#[test_context(Context)]
#[test]
fn test_add_assign_const(ctx: &mut Context) {
    *ctx.chip8.cpu.vx(V0) = 0xF;

    let instr = Instruction::AddAssignConst { x: V0, value: 42 };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(V0), 57);
}

#[test_context(Context)]
#[test]
fn test_assign_reg(ctx: &mut Context) {
    let index_y = Index::try_new(0xD).unwrap();
    *ctx.chip8.cpu.vx(V0) = 0;
    *ctx.chip8.cpu.vx(index_y) = 0xC4;

    let instr = Instruction::AssignReg { x: V0, y: index_y };

    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(V0), 0xC4);
}

#[test_context(Context)]
#[test]
fn test_random(ctx: &mut Context) {
    ctx.chip8.cpu.random_engine = SmallRng::seed_from_u64(42);

    let mut rng = SmallRng::seed_from_u64(42);

    let instr = Instruction::Rand { x: V0, value: 0xEA };
    assert!(matches!(ctx.chip8.execute(instr), Ok(ExecResult::Advance)));
    assert_eq!(*ctx.chip8.cpu.vx(V0), rng.random_range(0..=0xFF) & 0xEA);
}

proptest! {
    #[test]
    fn test_bcd(num in 0x0..=0xFF_u8) {
        let ones = num % 10;
        let tens = (num / 10) % 10;
        let hundreds = num / 100;

        let mut chip8 = Chip8::new();

        *chip8.cpu.vx(V0) = num;
        let instr = Instruction::SetBCD {
            x: V0,
        };
        chip8.cpu.set_address(0x400).unwrap();

        chip8.execute(instr).unwrap();

        assert_eq!(chip8.memory.read_byte(0x400).unwrap(), hundreds);
        assert_eq!(chip8.memory.read_byte(0x401).unwrap(), tens);
        assert_eq!(chip8.memory.read_byte(0x402).unwrap(), ones);
    }
}
