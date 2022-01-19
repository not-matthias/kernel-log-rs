![Rust](https://github.com/not-matthias/kernel-log-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/kernel-log.svg)](https://crates.io/crates/kernel-log)
[![docs.rs](https://docs.rs/kernel-log/badge.svg)](https://docs.rs/kernel-log)

# kernel-log-rs

A minimalistic logger for Windows Kernel Drivers.

## Usage

```rust
#![no_std]

use kernel_log::KernelLogger;

#[no_mangle]
pub extern "system" fn DriverEntry(_: PDRIVER_OBJECT, _: u64) -> NTSTATUS {
    KernelLogger::init(LevelFilter::Info).expect("Failed to initialize logger");

    log::warn!("This is an example message.")
}
```