pub struct Logger;

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __kernel_println(s)
    }
}

impl Logger {
    pub const fn new() -> Self {
        Self
    }

    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __kernel_println(s)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        __kernel_println("\n")
    }
}

#[doc(hidden)]
#[inline]
pub fn __kernel_println(msg: &str) -> core::fmt::Result {
    unsafe { ntapi::ntdbg::DbgPrintEx(0, 0, alloc::format!("{}\0", msg).as_ptr() as _) };
    Ok(())
}
