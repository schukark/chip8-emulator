//! Macro helpers to reduce boilerplate in the decoding process

/// Helper macro to reduce boilerplate
///
/// Reg2 is concerned with ?X??
#[macro_export]
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

/// Helper macro to reduce boilerplate
///
/// Reg2 is concerned with ?XY?
#[macro_export]
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

/// Helper macro to reduce boilerplate
///
/// Reg3 is concerned with ?XYZ
#[macro_export]
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

/// Helper macro to reduce boilerplate
///
/// addr is concerned with ?NNN
#[macro_export]
macro_rules! op_addr {
    ($variant:ident, $n1:expr, $n2:expr, $n3:expr) => {
        Instruction::$variant {
            address: [$n1, $n2, $n3].into(),
        }
    };
}

/// Helper macro to reduce boilerplate
///
/// addr is concerned with ?XNN
#[macro_export]
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
