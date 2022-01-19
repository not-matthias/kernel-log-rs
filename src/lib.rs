//! Implements the `print!`, `println!` and `dbg!` macros so they can be used in
//! the kernel without the use of an allocator.
//!
//! By default the macros are prefixed with `kernel_`. If you want to remove the
//! prefix, you can enable the `std_name` feature.
//!
//! ## Usage
//!
//! Exactly as you'd use the original macros from the standard library.
//!
//! ```no_run
//! #![no_std]
//!
//! // ...
//!
//! kernel_dbg!(2 + 2);
//! kernel_print!("{} + {} = {}\n", 2, 2, 2 + 2);
//! kernel_println!("{} + {} = {}", 2, 2, 2 + 2);
//! ```
//!
//! ## Features
//!
//! - `std_name`: Allows you to use the macros without the `kernel_` prefix.
//! - `format`: Uses the `format!` macro instead of the `core::fmt::Write` trait
//!   to convert the passed data into a string.

//! This crate provides a simple wrapper for logging with the `DbgPrint`
//! function. The logs won't be included in the final binary which helps to
//! harden reverse engineering.

#![no_std]

extern crate alloc;

use alloc::{format, string::ToString};
use log::{LevelFilter, Metadata, Record, SetLoggerError};
use ntapi::ntdbg::DbgPrint;

static LOGGER: KernelLogger = KernelLogger;

pub struct KernelLogger;

impl KernelLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool { true }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!(
                "{:<5} [{}] {}\0",
                record.level().to_string(),
                record.target(),
                record.args()
            );
            unsafe { DbgPrint(message.as_ptr() as _) };
        }
    }

    fn flush(&self) {}
}
