//! Chip8 emulator in rust

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::{env::args, fs::File, io::Read, thread::sleep, time::Duration};

use crate::{decoder::instruction::decode, machine::Chip8};

mod decoder;
mod machine;
mod types;

fn main() {
    let mut args = args();

    if args.len() != 2 {
        eprintln!("Usage: chip8-emulator path/to/rom");
        return;
    }

    let mut file = File::open(args.nth(1).unwrap()).expect("The provided rom file should exist");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Couldn't read all file");

    let decoded = decode(&buf);

    let decoded = match decoded {
        Err(e) => {
            eprintln!("An error occured: {e}");
            return;
        }
        Ok(decoded_instructions) => decoded_instructions,
    };

    for instruction in decoded {
        println!("{}", instruction);
    }

    let mut chip8 = Chip8::new();
    if let Err(e) = chip8.load_program(&buf) {
        eprintln!("Error occured in chip8 emulation: {e}");
    }

    loop {
        if let Err(e) = chip8.step() {
            eprintln!("Error occured in chip8 emulation: {e}");
            return;
        }

        sleep(Duration::from_secs(1));
    }
}
