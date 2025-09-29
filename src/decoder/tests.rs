use proptest::prelude::*;
use test_case::test_case;

use crate::{
    decoder::instruction::{DecodeError, Instruction},
    types::Index,
};

const HEX: std::ops::RangeInclusive<u16> = 0x0..=0xF;

fn from_nibbles(nibbles: [u16; 4]) -> u16 {
    (nibbles[0] << 12) | (nibbles[1] << 8) | (nibbles[2] << 4) | nibbles[3]
}

#[test]
fn test_clear_display_correct() -> anyhow::Result<()> {
    let code = from_nibbles([0, 0, 0xE, 0]);
    assert!(matches!(
        Instruction::try_from(code)?,
        Instruction::ClearDisplay
    ));

    Ok(())
}

#[test]
fn test_return_correct() -> anyhow::Result<()> {
    let code = from_nibbles([0, 0, 0xE, 0xE]);
    assert!(matches!(Instruction::try_from(code)?, Instruction::Return));

    Ok(())
}

#[test_case(0xFFFF ; "incorrect opcode 1")]
#[test_case(0x8FAB ; "incorrect opcode 2")]
#[test_case(0xE01B ; "incorrect opcode 3")]
#[test_case(0x8FAB ; "incorrect opcode 4")]
fn test_invalid_opcdes(opcode: u16) {
    let result = Instruction::try_from(opcode);
    assert!(matches!(result, Err(DecodeError::NoSuchInstruction(_))));
}

proptest! {
    #[test]
    fn test_call_machine_code(n1 in HEX, n2 in HEX, n3 in HEX) {
        prop_assume!([n1, n2, n3] != [0, 0xE, 0xE]);
        prop_assume!([n1, n2, n3] != [0, 0xE, 0]);

        let code = from_nibbles([0, n1, n2, n3]);
        let correct_address = (n1 << 8) | (n2 << 4) | n3 ;
        let result = Instruction::try_from(code).unwrap();

        let ok = matches!(result, Instruction::CallMachineCode {
            address
        } if address.into_inner() == correct_address);
        prop_assert!(ok, "code: {:#4X}, result: {:?}, expected: {:#03X}", code, result, correct_address);
    }

    #[test]
    fn test_goto(n1 in HEX, n2 in HEX, n3 in HEX) {
        let code = from_nibbles([1, n1, n2, n3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_address = (n1 << 8) | (n2 << 4) | n3;

        let ok = matches!(
            result,
            Instruction::Goto { address }
                if address.into_inner() == correct_address
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#03X}", code, result, correct_address);
    }

    #[test]
    fn test_call_subroutine(n1 in HEX, n2 in HEX, n3 in HEX) {
        let code = from_nibbles([2, n1, n2, n3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_address = (n1 << 8) | (n2 << 4) | n3;

        let ok = matches!(
            result,
            Instruction::CallSubroutine { address }
                if address.into_inner() == correct_address
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#03X}", code, result, correct_address);
    }

    #[test]
    fn test_skip_if_eq_const(x in HEX, n1 in HEX, n2 in HEX) {
        let code = from_nibbles([3, x, n1, n2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_const = ((n1 << 4) | n2) as u8;
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::EqConst { x, value }
                if value == correct_const && x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_skip_if_neq_const(x in HEX, n1 in HEX, n2 in HEX) {
        let code = from_nibbles([4, x, n1, n2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_const = ((n1 << 4) | n2) as u8;
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::NeqConst { x, value }
                if value == correct_const && x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_skip_if_eq_reg(x in HEX, y in HEX) {
        let code = from_nibbles([5, x, y, 0]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::EqReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_load_const(x in HEX, n1 in HEX, n2 in HEX) {
        let code = from_nibbles([6, x, n1, n2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_const = ((n1 << 4) | n2) as u8;
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::AssignConst { x, value }
                if value == correct_const && x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_add_assign_const(x in HEX, n1 in HEX, n2 in HEX) {
        let code = from_nibbles([7, x, n1, n2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_const = ((n1 << 4) | n2) as u8;
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::AddAssignConst { x, value }
                if value == correct_const && x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_assign_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 0]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::AssignReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_or_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 1]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::OrReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_and_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::AndReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_xor_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::XorReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_add_assign_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 4]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::AddAssignReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }
    #[test]
    fn test_sub_assign_reg(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 5]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::SubAssignReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_right_shift(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 6]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::RShift { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_sub_assign_reg_inverse(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 7]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::SubAssignRegInverse { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_left_shift(x in HEX, y in HEX) {
        let code = from_nibbles([8, x, y, 0xE]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::LShift { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_neq_reg(x in HEX, y in HEX) {
        let code = from_nibbles([9, x, y, 0]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};

        let ok = matches!(
            result,
            Instruction::NeqReg { x, y }
                if x == correct_x && y == correct_y
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: VX={:X}, VY={:X}", code, result, correct_x, correct_y);
    }

    #[test]
    fn test_set_address(n1 in HEX, n2 in HEX, n3 in HEX) {
        let code = from_nibbles([0xA, n1, n2, n3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_address = (n1 << 8) | (n2 << 4) | n3;

        let ok = matches!(
            result,
            Instruction::SetI { address }
                if address.into_inner() == correct_address
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#03X}", code, result, correct_address);
    }

    #[test]
    fn test_goto_plus_v0(n1 in HEX, n2 in HEX, n3 in HEX) {
        let code = from_nibbles([0xB, n1, n2, n3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_address = (n1 << 8) | (n2 << 4) | n3;

        let ok = matches!(
            result,
            Instruction::GotoPlusV0 { address }
                if address.into_inner() == correct_address
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#03X}", code, result, correct_address);
    }

    #[test]
    fn test_rand(x in HEX, n1 in HEX, n2 in HEX) {
        let code = from_nibbles([0xC, x, n1, n2]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_const = ((n1 << 4) | n2) as u8;

        let ok = matches!(
            result,
            Instruction::Rand { x, value }
                if x == correct_x && value == correct_const
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_draw_sprites(x in HEX, y in HEX, n in HEX) {
        let code = from_nibbles([0xD, x, y, n]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};
        let correct_y = unsafe {Index::new_unchecked(y as u8)};
        let correct_const = n as u8;

        let ok = matches!(
            result,
            Instruction::DrawSprite { x, y, height }
                if x == correct_x && y == correct_y && height.into_inner() == correct_const
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}, expected: {:#02X}", code, result, correct_const);
    }

    #[test]
    fn test_skip_if_pressed(x in HEX) {
        let code = from_nibbles([0xE, x, 9, 0xE]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::KeyPressedSkip { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_skip_if_released(x in HEX) {
        let code = from_nibbles([0xE, x, 0xA, 1]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::KeyReleasedSkip { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_get_delay_timer(x in HEX) {
        let code = from_nibbles([0xF, x, 0, 7]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::GetDelayTimer { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_await_key_press(x in HEX) {
        let code = from_nibbles([0xF, x, 0, 0xA]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::AwaitKeyPress { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_set_delay_timer(x in HEX) {
        let code = from_nibbles([0xF, x, 1, 5]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::SetDelayTimer { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_set_sound_timer(x in HEX) {
        let code = from_nibbles([0xF, x, 1, 8]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::SetSoundTimer { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_add_assign_address(x in HEX) {
        let code = from_nibbles([0xF, x, 1, 0xE]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::AddAssignAddress { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_load_sprite(x in HEX) {
        let code = from_nibbles([0xF, x, 2, 9]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::SetSpriteAddr { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_set_bcd(x in HEX) {
        let code = from_nibbles([0xF, x, 3, 3]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::SetBCD { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_dump_registers(x in HEX) {
        let code = from_nibbles([0xF, x, 5, 5]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::DumpRegisters { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }

    #[test]
    fn test_read_registers(x in HEX) {
        let code = from_nibbles([0xF, x, 6, 5]);
        let result = Instruction::try_from(code).unwrap();
        let correct_x = unsafe {Index::new_unchecked(x as u8)};

        let ok = matches!(
            result,
            Instruction::LoadRegisters { x }
                if x == correct_x
        );

        prop_assert!(ok, "code: {:#04X}, result: {:?}", code, result);
    }
}
