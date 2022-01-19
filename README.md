![Rust](https://github.com/not-matthias/kernel-print-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/kernel-print.svg)](https://crates.io/crates/kernel-print)
[![docs.rs](https://docs.rs/kernel-print/badge.svg)](https://docs.rs/kernel-print)

# kernel-log-rs

A minimalistic logger for Windows Kernel Drivers.

## Usage

```rust
#![no_std]

use kernel_log::KernelLogger;

#[no_mangle]
pub extern "system" fn DriverEntry(driver: *mut DRIVER_OBJECT, _: u64) -> NTSTATUS {
    KernelLogger::init(LevelFilter::Info).expect("Failed to initialize logger");

    log::warn!("This is an example message.")
}
```