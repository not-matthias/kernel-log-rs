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
        core::fmt::Write::write_str(self, s)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        self.write_str("\n")
    }
}

#[doc(hidden)]
pub fn __kernel_println(msg: &str) -> core::fmt::Result {
    unsafe { ntapi::ntdbg::DbgPrint(msg.as_ptr() as _) };
    Ok(())
}
