use alloc::string::String;

pub struct KernelWriter;

impl core::fmt::Write for KernelWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __kernel_println(s)
    }
}

impl KernelWriter {
    pub const fn new() -> Self {
        Self
    }

    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        core::fmt::Write::write_str(self, s)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        __kernel_println("\n")
    }
}

#[doc(hidden)]
#[inline]
pub fn __kernel_println<S: Into<String>>(string: S) -> core::fmt::Result {
    // Add the null-terminator to the string.
    //
    let string = {
        let mut temp = string.into();
        temp.push('\0');
        temp
    };

    // Print the null-terminated string.
    //
    unsafe { ntapi::ntdbg::DbgPrintEx(0, 0, string.as_ptr() as _) };

    Ok(())
}
