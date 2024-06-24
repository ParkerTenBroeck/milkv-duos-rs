#[macro_export]
macro_rules! mmio_write_32 {
    ($ptr:expr, $val:expr) => {
        ($ptr as *mut u32).write_volatile($val)
    };
}
#[macro_export]
macro_rules! mmio_read_32 {
    ($ptr:expr) => {
        ($ptr as *const u32).read_volatile()
    };
}
