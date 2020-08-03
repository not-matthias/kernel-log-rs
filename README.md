# kernel-logger-rs

Implements the `print!`, `println!` and `dbg!` macros so they can be used in the kernel without the use of an allocator.

By default the macros are prefixed with `kernel_`. If you want to remove the prefix, you can enable the `std_name` feature.

## Usage

Exactly as you'd use the original macros from the standard library.

```no_run
#![no_std]

// ...

kernel_dbg!("{}", dbg!(2 + 2));
kernel_println!("Hello {}!", "world");
```

## Features

- `std_name`: Allows you to use the macros without the `kernel_` prefix.
