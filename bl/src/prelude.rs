#[macro_export]
macro_rules! println {
    () => {{
        use core::fmt::Write;
        writeln!($crate::prelude::Stdout).unwrap();
    }};
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!($crate::prelude::Stdout, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::prelude::Stdout, $($arg)*).unwrap();
    }};
}

pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        milkv_rs::uart::print(s);
        Ok(())
    }
}


pub use milkv_rs::*;