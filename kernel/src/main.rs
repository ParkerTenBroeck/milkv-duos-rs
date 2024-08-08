#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(c_variadic)]

use core::f32::MIN;
use core::{arch, u64, usize};

use mem::PageTable;
use milkv_rs::{timer, uart};

pub mod entry;
pub mod interrupt_vector;
pub mod io;
pub mod panic;
pub mod prelude;
pub mod proc;
pub mod ray;
pub mod uart_sys;
pub mod vga;

use milkv_rs::*;

pub unsafe fn run_second_core() {
    let addr: usize;
    core::arch::asm!(
        "la {0}, _second_core_start",
        out(reg) addr
    );
    platform::reset_c906l_to_addr(addr)
}

#[no_mangle]
pub extern "C" fn _stage0_main() {
    milkv_rs::uart::print("Entering stage0\n");
    unsafe {
        core::arch::asm!(
            "
            # mhcr
            li t0, 0x11ff
            csrs 0x7C1,t0

            #mhint
            li t0, 0x6e30c
            csrs 0x7C5,t0

            #mxstatus
            li t0, 0x638000
            csrs 0x7C0,t0

            #msmpr
            csrsi 0x7F3,0x1

            #mccr2
            li t0, 0xe0000009
            csrs 0x7C3,t0
            ",
            out("t0") _
        );

        milkv_rs::plic::clear();

        milkv_rs::riscv::register::mideleg::set_sext();
        milkv_rs::riscv::register::mideleg::set_ssoft();
        milkv_rs::riscv::register::mideleg::set_stimer();

        // milkv_rs::riscv::register::medeleg::set_breakpoint();
        // milkv_rs::riscv::register::medeleg::set_illegal_instruction();
        // milkv_rs::riscv::register::medeleg::set_instruction_fault();
        // milkv_rs::riscv::register::medeleg::set_instruction_misaligned();
        // milkv_rs::riscv::register::medeleg::set_instruction_page_fault();
        // milkv_rs::riscv::register::medeleg::set_load_fault();
        // milkv_rs::riscv::register::medeleg::set_load_misaligned();
        // milkv_rs::riscv::register::medeleg::set_load_page_fault();
        // milkv_rs::riscv::register::medeleg::set_store_misaligned();
        // milkv_rs::riscv::register::medeleg::set_store_page_fault();
        milkv_rs::riscv::register::medeleg::set_supervisor_env_call();
        milkv_rs::riscv::register::medeleg::set_user_env_call();

        milkv_rs::riscv::register::pmpcfg0::set_pmp(
            0,
            milkv_rs::riscv::register::Range::NAPOT,
            milkv_rs::riscv::register::Permission::RWX,
            false,
        );
        milkv_rs::riscv::register::pmpaddr0::write(0);
        milkv_rs::riscv::register::pmpaddr0::write(usize::MAX);
        milkv_rs::uart::print("Initialized PMP\n");

        // SHPMCR (S-mode event monitoring control register)
        // core::arch::asm!("
        //     csrw 0x5C9, {0}
        // ", in(reg) 1usize<<63 | 1<<11 | 1<<10 | 3);

        milkv_rs::riscv::register::mcounteren::set_cy();
        milkv_rs::riscv::register::mcounteren::set_ir();
        milkv_rs::riscv::register::mcounteren::set_tm();
        for i in 3..32 {
            milkv_rs::riscv::register::mcounteren::set_hpm(i);
        }

        milkv_rs::riscv::register::scounteren::set_cy();
        milkv_rs::riscv::register::scounteren::set_ir();
        milkv_rs::riscv::register::scounteren::set_tm();
        for i in 3..32 {
            milkv_rs::riscv::register::scounteren::set_hpm(i);
        }

        milkv_rs::riscv::register::sstatus::set_sum();
        milkv_rs::uart::print("Setup supervisor permissions\n");

        milkv_rs::uart::print("Initialized priliminary page table\n");
        page_init();
        core::arch::asm!("th.dcache.call");

        uart::print("Starting second core\n");
        run_second_core();

        milkv_rs::riscv::register::satp::set(
            riscv::register::satp::Mode::Sv39,
            1,
            KERNEL_CTX.priliminary_page_table as usize >> 12,
        );
        milkv_rs::riscv::asm::fence();
        milkv_rs::riscv::asm::fence_i();
        milkv_rs::riscv::asm::sfence_vma_all();

        milkv_rs::riscv::register::mstatus::clear_mie();
        milkv_rs::riscv::register::mstatus::set_mpp(riscv::register::mstatus::MPP::Supervisor);
        let ie = milkv_rs::riscv::register::mstatus::read().mie();
        if ie {
            milkv_rs::riscv::register::mstatus::set_mpie();
        }
        milkv_rs::uart::print("Entering Kernel & Virtual Memory\n");

        // Lazy address translation
        milkv_rs::riscv::register::mepc::write(
            os_main as usize + KERNEL_CTX.layout.virt_start - KERNEL_CTX.layout.phys_start,
        );
        core::arch::asm!("
        mv sp,t0
        mret
        ", in("t0") KERNEL_CTX.sync_int_stack as usize);
        core::hint::unreachable_unchecked();
    }
}

struct EarlyPageBump(usize);

impl EarlyPageBump {
    unsafe fn next_zeroed(&mut self) -> *mut [u8; 0x1000] {
        let val = self.0 as *mut [u8; 0x1000];
        self.0 += 4096;
        core::ptr::write_bytes(val, 0, 1);
        val
    }
    unsafe fn next_uninit(&mut self) -> *mut [u8; 0x1000] {
        let val = self.0 as *mut [u8; 0x1000];
        self.0 += 4096;
        val
    }
    unsafe fn next_n_uninit(&mut self, n: usize) -> *mut [u8; 0x1000] {
        let val = self.0 as *mut [u8; 0x1000];
        self.0 += 4096 * n;
        val
    }
}

pub const PHYS_MAP: u64 = u64::MAX - (0x100000000 - 1);
static mut EARLY_PAGE_BUMP: EarlyPageBump = EarlyPageBump(0);
#[no_mangle]
pub static mut KERNEL_CTX: KernelCtx = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

#[repr(C)]
#[derive(Debug)]
pub struct KernelCtx {
    pub async_int_stack: *mut (),
    pub sync_int_stack: *mut (),
    pub stack_size: usize,
    pub priliminary_page_table: *mut mem::PageTable,
    pub layout: KernelLayout,
}

#[repr(C)]
#[derive(Debug)]
pub struct KernelLayout {
    pub code_size: usize,
    pub ro_size: usize,
    pub data_size: usize,
    pub bss_size: usize,
    pub total_size: usize,

    pub phys_start: usize,
    pub phys_size: usize,

    pub virt_start: usize,
}

struct PageInitializer {
    root: *mut mem::PageTable,
    offset: u64,
}

impl PageInitializer {
    pub fn new(root: *mut mem::PageTable, offset: u64) -> Self {
        Self {
            root: (root as usize + offset as usize) as *mut mem::PageTable,
            offset,
        }
    }

    unsafe fn map_huge_huge_page(&mut self, virt: usize, phys: usize, entry: mem::PageTableEntry) {
        let ppn2 = (virt >> (9 + 9 + 12)) & ((1 << 9) - 1);
        (*self.root).entries[ppn2] = entry.set_ppn(phys as u64 >> 12);
    }

    unsafe fn map_huge_page(&mut self, virt: usize, phys: usize, entry: mem::PageTableEntry) {
        let ppn2 = (virt >> (9 + 9 + 12)) & ((1 << 9) - 1);
        let ppn1 = (virt >> (9 + 12)) & ((1 << 9) - 1);

        let mut curr = &mut *self.root;

        if !curr.entries[ppn2].valid() {
            let new = EARLY_PAGE_BUMP.next_zeroed();

            curr.entries[ppn2] = mem::PageTableEntry::new()
                .set_valid(true)
                .set_ppn(new as u64 >> 12);
        }
        curr = &mut *(((curr.entries[ppn2].ppn() << 12) + self.offset) as *mut mem::PageTable);

        curr.entries[ppn1] = entry.set_ppn(phys as u64 >> 12);
    }

    unsafe fn map_page(&mut self, virt: usize, phys: usize, entry: mem::PageTableEntry) {
        let ppn2 = (virt >> (9 + 9 + 12)) & ((1 << 9) - 1);
        let ppn1 = (virt >> (9 + 12)) & ((1 << 9) - 1);
        let ppn0 = (virt >> (12)) & ((1 << 9) - 1);

        let mut curr = &mut *self.root;

        for ppn in [ppn2, ppn1] {
            if !curr.entries[ppn].valid() {
                let new = EARLY_PAGE_BUMP.next_zeroed();

                curr.entries[ppn] = mem::PageTableEntry::new()
                    .set_valid(true)
                    .set_ppn(new as u64 >> 12);
            }
            curr = &mut *(((curr.entries[ppn].ppn() << 12) + self.offset) as *mut mem::PageTable);
        }

        curr.entries[ppn0] = entry.set_ppn(phys as u64 >> 12);
    }

    unsafe fn map_pages(
        &mut self,
        virt: usize,
        phys: usize,
        size: usize,
        entry: mem::PageTableEntry,
    ) {
        for p in 0..((size + 0x0FFF) >> 12) {
            self.map_page(virt + (p << 12), phys + (p << 12), entry);
        }
    }
}

unsafe fn page_init() {
    let stack_size: usize;
    core::arch::asm!(
        "la {0}, __STACK_SIZE__",
        out(reg) stack_size
    );
    KERNEL_CTX.stack_size = stack_size;

    core::arch::asm!(
        "la {0}, __CODE_SIZE__",
            out(reg) KERNEL_CTX.layout.code_size
    );
    core::arch::asm!(
        "la {0}, __RO_SIZE__",
        out(reg) KERNEL_CTX.layout.ro_size
    );
    core::arch::asm!(
        "la {0}, __DATA_SIZE__",
        out(reg) KERNEL_CTX.layout.data_size
    );

    core::arch::asm!(
        "la {0}, __BSS_SIZE__",
        out(reg) KERNEL_CTX.layout.bss_size
    );

    core::arch::asm!(
        "la {0}, __KERNEL_SIZE__",
        out(reg) KERNEL_CTX.layout.total_size
    );

    // core::arch::asm!(
    //     "li {0}, __KERNEL_START_PHYS__",
    //     out(reg) kl.phys_start
    // );
    // core::arch::asm!(
    //     "la {0}, __KERNEL_START_VIRT__",
    //     out(reg) kl.virt_start
    // );

    KERNEL_CTX.layout.phys_start = 0x80000000;
    KERNEL_CTX.layout.virt_start = 0xFFFFFFC000000000;

    core::arch::asm!(
        "la {0}, __MEM_SIZE__",
        out(reg) KERNEL_CTX.layout.phys_size
    );

    EARLY_PAGE_BUMP = EarlyPageBump(
        KERNEL_CTX.layout.phys_start + KERNEL_CTX.layout.total_size + KERNEL_CTX.stack_size * 2,
    );

    KERNEL_CTX.priliminary_page_table = EARLY_PAGE_BUMP.next_zeroed().cast::<mem::PageTable>();

    let mut page_init = PageInitializer::new(KERNEL_CTX.priliminary_page_table, 0);

    // text section
    page_init.map_pages(
        KERNEL_CTX.layout.virt_start,
        KERNEL_CTX.layout.phys_start,
        KERNEL_CTX.layout.code_size,
        mem::PageTableEntry::COM_EXEC | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // ro section
    page_init.map_pages(
        KERNEL_CTX.layout.virt_start + KERNEL_CTX.layout.code_size,
        KERNEL_CTX.layout.phys_start + KERNEL_CTX.layout.code_size,
        KERNEL_CTX.layout.ro_size,
        mem::PageTableEntry::COM_RO | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // data section
    page_init.map_pages(
        KERNEL_CTX.layout.virt_start + KERNEL_CTX.layout.code_size + KERNEL_CTX.layout.ro_size,
        KERNEL_CTX.layout.phys_start + KERNEL_CTX.layout.code_size + KERNEL_CTX.layout.ro_size,
        KERNEL_CTX.layout.data_size,
        mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // bss section
    page_init.map_pages(
        KERNEL_CTX.layout.virt_start
            + KERNEL_CTX.layout.code_size
            + KERNEL_CTX.layout.ro_size
            + KERNEL_CTX.layout.data_size,
        KERNEL_CTX.layout.phys_start
            + KERNEL_CTX.layout.code_size
            + KERNEL_CTX.layout.ro_size
            + KERNEL_CTX.layout.data_size,
        KERNEL_CTX.layout.bss_size,
        mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // stacks with overflow protection
    KERNEL_CTX.async_int_stack = (KERNEL_CTX.layout.virt_start
        + KERNEL_CTX.layout.total_size
        + 0x1000
        + KERNEL_CTX.stack_size) as *mut ();
    page_init.map_pages(
        KERNEL_CTX.async_int_stack as usize - KERNEL_CTX.stack_size,
        KERNEL_CTX.layout.phys_start + KERNEL_CTX.layout.total_size,
        KERNEL_CTX.stack_size,
        mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
    );
    KERNEL_CTX.sync_int_stack = (KERNEL_CTX.layout.virt_start
        + KERNEL_CTX.layout.total_size
        + 0x1000
        + KERNEL_CTX.stack_size
        + 0x1000
        + KERNEL_CTX.stack_size) as *mut ();
    page_init.map_pages(
        KERNEL_CTX.sync_int_stack as usize - KERNEL_CTX.stack_size,
        KERNEL_CTX.layout.phys_start + KERNEL_CTX.layout.total_size + KERNEL_CTX.stack_size,
        KERNEL_CTX.stack_size,
        mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // phys mem map
    for i in (0..0x100000000).step_by(1 << (12 + 9 + 9)) {
        page_init.map_huge_huge_page(
            PHYS_MAP as usize + i,
            i,
            mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
        );
    }

    // uart
    page_init.map_page(
        0x0000000004140014 & !((1 << 12) - 1),
        0x0000000004140014 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // reset
    page_init.map_page(
        0x00000000050260e0 & !((1 << 12) - 1),
        0x00000000050260e0 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );
    page_init.map_page(
        0x00000000050250ac & !((1 << 12) - 1),
        0x00000000050250ac & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );
    page_init.map_page(
        0x0000000003010004 & !((1 << 12) - 1),
        0x0000000003010004 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // second core
    page_init.map_page(
        0x0000000003003024 & !((1 << 12) - 1),
        0x0000000003003024 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );
    page_init.map_page(
        0x00000000020b0004 & !((1 << 12) - 1),
        0x00000000020b0004 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // stimercmp
    page_init.map_page(
        0x000000007400d004 & !((1 << 12) - 1),
        0x000000007400d004 & !((1 << 12) - 1),
        mem::PageTableEntry::COM_DEV | mem::PageTableEntry::DIRTY_ACCESSED,
    );

    // vga frame buffer
    let size = (core::mem::size_of::<vga::FrameBuf>() + 0x1000 - 1) / 0x1000 * 0x1000;
    let start = EARLY_PAGE_BUMP.next_n_uninit(size / 0x1000);
    vga::FRAME_BUF = start.cast();
    page_init.map_pages(
        start as usize,
        start as usize,
        size,
        mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
    );
}

pub struct Uart;
use core::fmt::Write;
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        milkv_rs::uart::print(s);
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn os_main() {
    extern "C" {
        #[link_name = "strap_vector"]
        pub fn strap_vector();
    }

    unsafe{
        milkv_rs::riscv::register::stvec::write(
            strap_vector as usize,
            riscv::register::stvec::TrapMode::Direct,
        );
    }



    unsafe {
        writeln!(Uart, "{:#x?}", KERNEL_CTX).unwrap();
        let free_kb = (KERNEL_CTX.layout.phys_size
            - (EARLY_PAGE_BUMP.0 - KERNEL_CTX.layout.phys_start))
            / 1024;
        writeln!(Uart, "Mem Free: {}kb", free_kb).unwrap();
    }

    unsafe{
        _ = mem::PageTable::disp_table(KERNEL_CTX.priliminary_page_table, PHYS_MAP,
            milkv_rs::riscv::register::satp::read().asid(), Uart);
    }

    unsafe fn switch_tasks(frame: &mut interrupt_vector::TrapFrame){
        let next = if !CURRENT_TASK.is_null() {
            let curr = &mut *CURRENT_TASK;
            for i in 0..31 {
                curr.regs[i + 1] = frame.regs[i];
            }
            curr.pc = frame.pc;
            curr.next_proc.load(core::sync::atomic::Ordering::Relaxed)
        } else {
            &mut TASKS[0]
        };

        CURRENT_TASK = next;

        let curr = &mut *CURRENT_TASK;
        for i in 0..31 {
            frame.regs[i] = curr.regs[i + 1];
        }
        frame.pc = curr.pc;
        
        // uart::print("asdasd\n");
        milkv_rs::timer::set_stimercmp(
            milkv_rs::timer::get_mtimer()
                + milkv_rs::timer::SYS_COUNTER_FREQ_IN_SECOND / 120,
        );
    }
    unsafe {
        // Timer interrupt
        interrupt_vector::add_timer_handler(
            core::ptr::null_mut::<()>(),
            |_ctx, frame, _scause, _sepc, _stval| {
                switch_tasks(frame);
            },
        );

        interrupt_vector::add_ecall_handler(
            5,
            core::ptr::null_mut::<()>(),
            |_ctx, frame, _scause, _sepc, _stval| {
                switch_tasks(frame);
            },
        );
    }

    // uart::print("Initializing video buffer\n");

    unsafe fn make_task(func: extern "C" fn(), stack_size: usize) -> proc::Task {
        let size = (stack_size + 0x1000 - 1) / 0x1000 * 0x1000;
        let stack = EARLY_PAGE_BUMP.next_n_uninit(size / 0x1000);

        PageInitializer::new(KERNEL_CTX.priliminary_page_table, PHYS_MAP).map_pages(
            stack as usize,
            stack as usize,
            size,
            mem::PageTableEntry::COM_RW | mem::PageTableEntry::DIRTY_ACCESSED,
        );
        let mut tmp = proc::Task {
            regs: [0; 32],
            regs_float: [0; 32],
            pc: func as usize,
            sstatus: milkv_rs::riscv::register::sstatus::read(),
            mem_map: KERNEL_CTX.priliminary_page_table,
            pid: 1,
            next_proc: Default::default(),
        };
        tmp.regs[2] = stack as usize + size;
        writeln!(Uart, "0x{:x?}..{:x?}", tmp.regs[2] - size, tmp.regs[2]);
        // writeln!(Uart, "{:x?}", tmp.regs[2]);
        tmp
    }

    unsafe {
        vga::init_vga();
        io::SOUT = |b| vga::print(b)
    }

    uart::print("Setting up tasks\n");
    unsafe {
        extern "C" fn t1() {
            ray::RayTrace::default().start_ray()
        }
        TASKS[0] = make_task(t1, 0x1000 * 4);

        extern "C" fn t2() {
            loop {
                uart::print("Hiiiiiii~\n");
                yeild();
            }
        }
        TASKS[1] = make_task(t2, 0x1000);

        extern "C" fn t3() {
            loop {
                // println!("\x1b[HTIMER: {}", timer::get_mtimer());
                // yeild();
                timer::mdelay(5000);
                unsafe{reset()}
            }
        }
        TASKS[2] = make_task(t3, 0x1000 * 2);

        use core::sync::atomic::Ordering;
        TASKS[0].next_proc.store(&mut TASKS[1], Ordering::Relaxed);
        TASKS[1].next_proc.store(&mut TASKS[2], Ordering::Relaxed);
        TASKS[2].next_proc.store(&mut TASKS[0], Ordering::Relaxed);

        milkv_rs::riscv::asm::sfence_vma_all();
        milkv_rs::riscv::asm::fence();
        milkv_rs::riscv::asm::fence_i();
        core::arch::asm!("th.dcache.call");
        milkv_rs::riscv::asm::sfence_vma_all();
    }

    unsafe{
        _ = mem::PageTable::disp_table(KERNEL_CTX.priliminary_page_table, PHYS_MAP,
            milkv_rs::riscv::register::satp::read().asid(), Uart);
    }


    uart::print("Enabling interrupts\n");
    unsafe {
        milkv_rs::timer::set_stimercmp(milkv_rs::timer::get_mtimer());

        milkv_rs::riscv::register::sie::set_sext();
        milkv_rs::riscv::register::sie::set_stimer();
        milkv_rs::riscv::register::sie::set_ssoft();

        milkv_rs::riscv::register::sstatus::set_sie();
    }

    timer::mdelay(5000);
    unsafe { reset() }
}

#[inline(always)]
pub fn yeild(){
    unsafe{
        core::arch::asm!("ecall", in("a7") 5)
    }
}

static mut CURRENT_TASK: *mut proc::Task = core::ptr::null_mut();
static mut TASKS: [proc::Task; 10] = unsafe { core::mem::zeroed() };

// unsafe fn mipi_test() {
//     // pin mux

//     let val = 1;

//     //CTRL_PAD_MIPI_TXM4
//     (0x0300_116C as *mut u32).write_volatile(val);

//     //CTRL_PAD_MIPI_TXM4
//     (0x0300_1194 as *mut u32).write_volatile(val);
//     //CTRL_PAD_MIPI_TXP4
//     (0x0300_1198 as *mut u32).write_volatile(val);

//     //CTRL_PAD_MIPI_TXM3
//     (0x0300_119C as *mut u32).write_volatile(val);
//     //CTRL_PAD_MIPI_TXP3
//     (0x0300_11A0 as *mut u32).write_volatile(val);

//     //CTRL_PAD_MIPI_TXM2
//     (0x0300_11A4 as *mut u32).write_volatile(val);
//     //CTRL_PAD_MIPI_TXP2
//     (0x0300_11A8 as *mut u32).write_volatile(val);

//     //CTRL_PAD_MIPI_TXM1
//     (0x0300_11AC as *mut u32).write_volatile(val);
//     //CTRL_PAD_MIPI_TXP1
//     (0x0300_11B0 as *mut u32).write_volatile(val);

//     //CTRL_PAD_MIPI_TXM0
//     (0x0300_11B4 as *mut u32).write_volatile(val);
//     //CTRL_PAD_MIPI_TXP0
//     (0x0300_11B8 as *mut u32).write_volatile(val);

//     #[repr(C)]
//     struct MIPITx {
//         dsi_mac_reg_00: u32,
//         dsi_mac_reg_01: u32,
//         dsi_mac_reg_02: u32,
//         dsi_mac_reg_03: u32,
//         dsi_mac_reg_04: u32,
//         dsi_mac_reg_05: u32,
//         dsi_mac_reg_06: u32,
//         dsi_mac_reg_07: u32,
//         dsi_mac_reg_08: u32,
//         dsi_mac_reg_09: u32,
//     }

//     #[repr(C)]
//     struct MIPITxPHY {
//         reg_00: u32,
//         reg_01: u32,
//         reg_02: u32,
//         reg_03: u32,
//         reg_04: u32,
//         reg_05: u32,
//         _reserved0: [u32; 29],
//         reg_23: u32,
//         reg_24: u32,
//         reg_25: u32,
//         reg_26: u32,
//         reg_27: u32,
//         reg_28: u32,
//         _reserved1: [u32; 4],
//         reg_2d: u32,
//     }

//     let tx_reg = 0x0A08A000 as *mut MIPITx;
//     let tx_phy = 0x0A0D1000 as *mut MIPITxPHY;
//     // clk_lane_en + data_0_lane_en
//     core::ptr::addr_of_mut!((*tx_phy).reg_00).write_volatile(0b1);

//     loop {
//         // core::ptr::addr_of_mut!((*tx_reg).dsi_mac_reg_00).write_volatile(0b1);
//     }
// }
