use test_case::test_case;

use crate::{
    decoder::instruction::Instruction,
    types::{Address, Index},
};

fn from_nibbles(nibbles: [u16; 4]) -> u16 {
    (nibbles[0] << 12) | (nibbles[1] << 8) | (nibbles[2] << 4) | nibbles[3]
}

#[test]
fn test_correct_clear_display() {
    let code = from_nibbles([0, 0, 0xE, 0]);
    let result = Instruction::try_from(code).unwrap();

    assert!(matches!(result, Instruction::ClearDisplay));
}

#[test]
fn test_correct_return() {
    let code = from_nibbles([0, 0, 0xE, 0xE]);
    let result = Instruction::try_from(code).unwrap();

    assert!(matches!(result, Instruction::Return));
}

#[test_case(0, 0, 0; "lowest edge case")]
#[test_case(0xF, 0xF, 0xF; "highest edge case")]
#[test_case(4, 6, 0; "random case 1")]
#[test_case(0xE, 7, 0xF; "random case 2")]
fn test_correct_goto(n1: u16, n2: u16, n3: u16) {
    let code = from_nibbles([1, n1, n2, n3]);
    let result = Instruction::try_from(code).unwrap();

    let correct_address: Address = [n1 as u8, n2 as u8, n3 as u8].into();
    assert!(matches!(
        result,
        Instruction::Goto {
            address
        } if address == correct_address
    ));
}

#[test_case(0, 0, 0; "lowest edge case")]
#[test_case(0xF, 0xF, 0xF; "highest edge case")]
#[test_case(0xC, 0xB, 3; "random case 1")]
#[test_case(0, 1, 0xD; "random case 2")]
fn test_correct_call_subroutine(n1: u16, n2: u16, n3: u16) {
    let code = from_nibbles([2, n1, n2, n3]);
    let result = Instruction::try_from(code).unwrap();

    let correct_address: Address = [n1 as u8, n2 as u8, n3 as u8].into();
    assert!(matches!(
        result,
        Instruction::CallSubroutine {
            address
        } if address == correct_address
    ));
}

#[test_case(0, 0, 0; "lowest edge case")]
#[test_case(0xF, 0xF, 0xF; "highest edge case")]
#[test_case(0xC, 0xB, 3; "random case 1")]
#[test_case(0, 1, 0xD; "random case 2")]
fn test_correct_eq_const(x: u16, n1: u16, n2: u16) {
    let code = from_nibbles([3, x, n1, n2]);
    let result = Instruction::try_from(code).unwrap();

    let correct_reg = Index::try_new(x as u8).unwrap();
    let correct_const = (n1 * 16 + n2) as u8;
    assert!(matches!(
        result,
        Instruction::EqConst {
            x,
            value
        } if x == correct_reg && value == correct_const
    ));
}

#[test_case(0, 0, 0; "lowest edge case")]
#[test_case(0xF, 0xF, 0xF; "highest edge case")]
#[test_case(0xC, 0xB, 3; "random case 1")]
#[test_case(0, 1, 0xD; "random case 2")]
fn test_correct_neq_const(x: u16, n1: u16, n2: u16) {
    let code = from_nibbles([4, x, n1, n2]);
    let result = Instruction::try_from(code).unwrap();

    let correct_reg = Index::try_new(x as u8).unwrap();
    let correct_const = (n1 * 16 + n2) as u8;
    assert!(matches!(
        result,
        Instruction::NeqConst {
            x,
            value
        } if x == correct_reg && value == correct_const
    ));
}

#[test_case(0, 0; "lowest edge case")]
#[test_case(0xF, 0xF; "highest edge case")]
#[test_case(0xC, 0xB; "random case 1")]
#[test_case(7, 1; "random case 2")]
fn test_correct_eq_reg(x: u16, y: u16) {
    let code = from_nibbles([5, x, y, 0]);
    let result = Instruction::try_from(code).unwrap();

    let correct_x = Index::try_new(x as u8).unwrap();
    let correct_y = Index::try_new(y as u8).unwrap();
    assert!(matches!(
        result,
        Instruction::EqReg {
            x,
            y
        } if x == correct_x && y == correct_y
    ));
}
