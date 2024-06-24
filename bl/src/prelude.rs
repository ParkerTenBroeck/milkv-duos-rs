
#[macro_export]
macro_rules! println {
    () => { 
        use core::fmt::Write;
        writeln!($crate::prelude::Std).unwrap();
    };
    ($($arg:tt)*) => { 
        use core::fmt::Write;
        writeln!($crate::prelude::Std, $($arg)*).unwrap();
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { 
        use core::fmt::Write;
        write!($crate::prelude::Std, $($arg)*).unwrap();
    };
}

pub struct Std;

impl core::fmt::Write for Std{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::uart::print(s);
        Ok(())
    }
}

// pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, num: usize){

// }