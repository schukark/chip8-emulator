//! Macros to simplify constructing decoded `Instruction` variants
//!
//! These macros are used during opcode decoding to convert nibbles
//! (or groups of nibbles) into strongly typed `Instruction` fields,
//! like register indices or addresses.

/// Constructs an instruction with a single register field.
///
/// This macro is used for opcodes of the form `?X??`,
/// where the second nibble (`X`) specifies a single register index.
///
/// # Example
/// ```
/// op_reg1!(Cls, x_nibble);
/// ```
macro_rules! op_reg1 {
    ($variant:ident, $x:expr) => {{
        let x = $x;
        unsafe {
            Instruction::$variant {
                x: Index::try_new(x).unwrap_unchecked(),
            }
        }
    }};
}

/// Constructs an instruction with two register fields.
///
/// Used for opcodes matching `?XY?`, where the second and third nibbles
/// (`X` and `Y`) are register indices.
///
/// # Example
/// ```
/// op_reg2!(Add, x_nibble, y_nibble);
/// ```
macro_rules! op_reg2 {
    ($variant:ident, $x:expr, $y:expr) => {{
        let x = $x;
        let y = $y;
        unsafe {
            Instruction::$variant {
                x: Index::try_new(x).unwrap_unchecked(),
                y: Index::try_new(y).unwrap_unchecked(),
            }
        }
    }};
}

/// Constructs an instruction with two register fields and an immediate value.
///
/// Used for opcodes like `?XYZ`, where `X` and `Y` are register indices
/// and `Z` represents a small immediate (e.g., sprite height).
///
/// # Example
/// ```
/// op_reg3!(Draw, x_nibble, y_nibble, height_nibble);
/// ```
macro_rules! op_reg3 {
    ($variant:ident, $x:expr, $y:expr, $z:expr) => {{
        let x = $x;
        let y = $y;
        let z = $z;
        unsafe {
            Instruction::$variant {
                x: Index::try_new(x).unwrap_unchecked(),
                y: Index::try_new(y).unwrap_unchecked(),
                height: SpriteHeight::try_new(z).unwrap_unchecked(),
            }
        }
    }};
}

/// Constructs an instruction with an address field.
///
/// Used for opcodes of the form `?NNN`, where the last three nibbles
/// represent a 12-bit address.
///
/// # Example
/// ```
/// op_addr!(Jump, n1, n2, n3);
/// ```
macro_rules! op_addr {
    ($variant:ident, $n1:expr, $n2:expr, $n3:expr) => {
        Instruction::$variant {
            address: [$n1, $n2, $n3].into(),
        }
    };
}

/// Constructs an instruction with a register and an immediate value.
///
/// Used for opcodes like `?XNN`, where `X` is a register index and
/// the last two nibbles form an 8-bit constant.
///
/// # Example
/// ```
/// op_regconst!(LoadConst, x_nibble, n1, n2);
/// ```
macro_rules! op_regconst {
    ($variant:ident, $x:expr, $n1:expr, $n2:expr) => {{
        let x = $x;
        let n1 = $n1;
        let n2 = $n2;
        unsafe {
            Instruction::$variant {
                x: Index::try_new(x).unwrap_unchecked(),
                value: n1 * 16 + n2,
            }
        }
    }};
}

pub(crate) use op_addr;
pub(crate) use op_reg1;
pub(crate) use op_reg2;
pub(crate) use op_reg3;
pub(crate) use op_regconst;
