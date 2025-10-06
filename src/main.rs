//! Chip8 emulator in rust

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![feature(custom_test_frameworks)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod decoder;
pub mod machine;
pub mod types;
pub mod window;

use tklog::{Format, LEVEL, LOG};

use crate::window::run_app;

#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::borrow_interior_mutable_const)] // As per docs of tklog, this is correct
/// Setup logging
fn log_init() {
    LOG.set_console(false) // Enables console logging
        .set_level(LEVEL::Info) // Sets the log level; default is Debug
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName) // Defines structured log output with chosen details
        .set_cutmode_by_size("logs/tklogsize.txt", 1 << 22, 10, true) // Cuts logs by file size (4 MB), keeps 10 backups, compresses backups
        .set_formatter("{level}{time} {file}:{message}\n"); // Customizes log output format; default is "{level}{time} {file}:{message}"
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let args = std::env::args();
    log_init();

    if args.len() != 2 {
        eprintln!("Usage: chip8-emulator /path/to/your/rom");
        return;
    }

    run_app(args).expect("Error occured when running the application");
}
