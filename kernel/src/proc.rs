#[repr(C)]
pub struct Task {
    pub regs: [usize; 32],
    pub regs_float: [usize; 32],
    pub pc: usize,
    pub sstatus: milkv_rs::riscv::register::sstatus::Sstatus,
    pub mem_map: *const milkv_rs::mem::PageTable,

    pub pid: usize,
    pub next_proc: core::sync::atomic::AtomicPtr<Task>,
}
