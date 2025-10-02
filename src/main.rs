//! Chip8 emulator in rust

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![feature(custom_test_frameworks)]

mod decoder;
mod window;
mod machine;
mod types;

use tklog::{Format, LEVEL, LOG};

/// Setup logging
fn log_init() {
    LOG.set_console(false) // Enables console logging
        .set_level(LEVEL::Trace) // Sets the log level; default is Debug
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName) // Defines structured log output with chosen details
        .set_cutmode_by_size("logs/tklogsize.txt", 1 << 22, 10, true) // Cuts logs by file size (4 MB), keeps 10 backups, compresses backups
        .set_formatter("{level}{time} {file}:{message}\n"); // Customizes log output format; default is "{level}{time} {file}:{message}"
}

fn main() {
    let args = std::env::args();
    log_init();

    if args.len() != 2 {
        eprintln!("Usage: chip8-emulator /path/to/your/rom");
        return;
    }

    let program_path = std::env::args().nth(1).unwrap();
    let program = std::fs::read(program_path).expect("Error occured when opening rom");
}
