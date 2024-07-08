fn main(){

    cc::Build::new()
        .compiler("riscv64-none-elf-gcc")
        .file("c/src/ddr.c")
        .file("c/src/ddr_sys_bring_up.c")
        .file("c/src/cvx16_pinmux.c")
        .file("c/src/ddr_pkg_info.c")
        .file("c/src/phy_pll_init.c")
        .file("c/src/cvx16_dram_cap_check.c")
        .file("c/src/ddr3_1866_x16/ddr_patch_regs.c")
        .file("c/src/ddr3_1866_x16/ddrc_init.c")
        .file("c/src/ddr3_1866_x16/phy_init.c")
        .include("c/include")
        .include("c/include/ddr3_1866_x16")
        .flag("-Wno-unused-variable")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")

        .define("DDR3", None)
        .define("REAL_DDRPHY", None)
        .define("uint", "unsigned int")
        .define("__packed", "__attribute__((packed))")
        .compile("ddr");


}