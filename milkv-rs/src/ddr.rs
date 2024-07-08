
extern "C" {
    #[link_name = "ddr_init"]
    fn ddr_init();
}

pub unsafe fn init_ddr() {
    ddr_init()
}