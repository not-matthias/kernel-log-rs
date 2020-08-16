//! Implements the `print!`, `println!` and `dbg!` macros so they can be used in the kernel without the use of an allocator.
//!
//! By default the macros are prefixed with `kernel_`. If you want to remove the prefix, you can enable the `std_name` feature.
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
//! kernel_dbg!("{}", dbg!(2 + 2));
//! kernel_println!("Hello {}!", "world");
//! ```
//!
//! ## Features
//!
//! - `std_name`: Allows you to use the macros without the `kernel_` prefix.

#![no_std]

extern crate alloc;

#[doc(hidden)]
pub mod logger;

#[cfg(feature = "std_name")]
#[doc(hidden)]
pub mod std_name {
    pub use super::kernel_dbg as dbg;
    pub use super::kernel_print as print;
    pub use super::kernel_println as println;
}
#[cfg(feature = "std_name")]
pub use std_name::*;

/// Macro for printing the value of a given expression for quick and dirty debugging.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html) for full documentation.
#[macro_export]
macro_rules! kernel_dbg {
    () => {
        $crate::kernel_println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::kernel_println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { $crate::kernel_dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::kernel_dbg!($val)),+,)
    };
}

/// Prints to the standard output.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`print!`](https://doc.rust-lang.org/std/macro.print.html) for full documentation.
#[macro_export]
macro_rules! kernel_print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut logger = $crate::logger::Logger::new();
            logger.write_fmt(format_args!($($arg)*));
        }
    };
}

/// Prints to the standard output, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`println!`](https://doc.rust-lang.org/std/macro.println.html) for full documentation.
#[macro_export]
macro_rules! kernel_println {
    () => {
        $crate::kernel_println!("")
    };
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut logger = $crate::logger::Logger::new();
            logger.write_fmt(format_args!($($arg)*));
            logger.write_nl();
        }
    };
}
