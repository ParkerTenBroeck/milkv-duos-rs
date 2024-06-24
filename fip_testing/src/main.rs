use std::{
    io::Write,
    mem::offset_of,
};

pub const PARAM1_MAGIC1: [u8; 8] = [b'C', b'V', b'B', b'L', b'0', b'1', b'\n', b'\0'];
pub const PARAM2_MAGIC1: [u8; 8] = [b'C', b'V', b'L', b'D', b'0', b'2', b'\n', b'\0'];

pub const LOADER_2ND_MAGIC_ORIG: U32Le = U32Le([b'B', b'L', b'3', b'3']);
pub const LOADER_2ND_MAGIC_LZMA: U32Le = U32Le([b'B', b'3', b'M', b'A']);
pub const LOADER_2ND_MAGIC_LZ4: U32Le = U32Le([b'B', b'3', b'Z', b'4']);

pub const IMAGE_ALIGN: usize = 512;

#[repr(C)]
#[derive(PartialEq, Eq, Hash)]
pub struct U32Le([u8; 4]);

impl std::fmt::Debug for U32Le {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl U32Le {
    pub fn get(&self) -> u32 {
        u32::from_le_bytes(self.0)
    }

    pub fn new(val: u32) -> Self{
        Self(val.to_le_bytes())
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Hash)]
pub struct U64Le([u8; 8]);

impl std::fmt::Debug for U64Le {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl U64Le {
    pub fn get(&self) -> u64 {
        u64::from_le_bytes(self.0)
    }

    pub fn new(val: u64) -> Self{
        Self(val.to_le_bytes())
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Param1 {
    magic_1: [u8; 8],
    magic_2: [u8; 4],
    param_checksum: U32Le,
    nand_info: [u8; 128],
    nor_info: [u8; 36],
    fip_flags: [u8; 8],
    chip_conf_size: U32Le,
    blcp_img_check_sum: U32Le,
    blcp_img_size: U32Le,
    blcp_img_runaddr: U32Le,
    blcp_param_loadaddr: U32Le,
    blcp_param_size: U32Le,
    bl2_img_check_sum: U32Le,
    bl2_img_size: U32Le,
    bld_img_size: U32Le,

    param2_loadaddr: U32Le,
    _reserved1: U32Le,
    chip_conf: [u8; 760],
    bl_ek: [u8; 32],
    root_pk: [u8; 512],
    bl_pk: [u8; 512],
    bl_pk_sig: [u8; 512],
    chip_conf_sig: [u8; 512],
    bl2_img_sig: [u8; 512],
    blcp_img_sig: [u8; 512],
}

#[derive(Debug)]
#[repr(C)]
pub struct Param2 {
    magic_1: [u8; 8],
    param2_check_sum: U32Le,
    _reserved1: U32Le,

    ddr_param_check_sum: U32Le,
    ddr_param_loadaddr: U32Le,
    ddr_param_size: U32Le,
    _ddr_param_reserved: U32Le,

    blcp_2nd_check_sum: U32Le,
    blcp_2nd_loadaddr: U32Le,
    blcp_2nd_size: U32Le,
    blcp_2nd_runaddr: U32Le,

    monitor_check_sum: U32Le,
    monitor_2nd_loadaddr: U32Le,
    monitor_size: U32Le,
    monitor_runaddr: U32Le,

    loader_2nd_reserved0: U32Le,
    loader_2nd_loadaddr: U32Le,
    loader_2nd_reserver1: U32Le,
    loader_2nd_reserved2: U32Le,

    reserved_last: [u8; 4096 - 16 * 5],
}

#[derive(Debug)]
#[repr(C)]
pub struct Ldr2ndHdr {
    jump0: U32Le,
    magic: U32Le,
    check_sum: U32Le,
    size: U32Le,
    runaddr: U64Le,
    _reserved1: U32Le,
    _reserver2: U32Le,
}

fn main() {
    make().unwrap();
    // dump("../fip.bin");
}

#[derive(Clone, Copy, Default)]
struct CheckSum{
    loc: usize,
    start: usize,
    end: usize,
}

pub fn calc_check_sum(msg: &[u8]) -> u32 {
    let mut crc: u16 = 0x0;
    for byte in msg {
        let mut x = ((crc >> 8) ^ (*byte as u16)) & 255;
        x ^= x >> 4;
        crc = (crc << 8) ^ (x << 12) ^ (x << 5) ^ x;
    }

    crc as u32 | 0xcafe0000
}

fn align_vec(v: &mut Vec<u8>){
    let len = (v.len() + (IMAGE_ALIGN-1)) & !(IMAGE_ALIGN-1);
    v.resize(len, 0);
}

pub fn make() -> std::io::Result<()>{

    let nand_info = std::fs::read("./make/nand_info.bin")?;
    let nor_info = std::fs::read("./make/nor_info.bin")?;
    let fip_flags = std::fs::read("./make/fip_flags.bin")?;

    let mut blcp_img = std::fs::read("./make/blcp.bin")?;
    let mut bl2_img = std::fs::read("./make/bl2.bin")?;

    let mut chip_conf = std::fs::read("./make/chip_conf.bin")?;
    let mut bl_ek: Vec<u8> = std::fs::read("./make/bl_ek.bin")?;
    let mut root_pk = std::fs::read("./make/root_pk.bin")?;
    let mut bl_pk = std::fs::read("./make/bl_pk.bin")?;

    let mut ddr_param = std::fs::read("./make/ddr_param.bin")?;
    let mut blcp_2nd = std::fs::read("./make/blcp_2nd.bin")?;
    let mut monitor = std::fs::read("./make/monitor.bin")?;

    let mut ldr2ndhdr = std::fs::read("./make/ldr2nd.bin")?;
    
    align_vec(&mut blcp_img);
    align_vec(&mut bl2_img);

    align_vec(&mut ddr_param);
    align_vec(&mut blcp_2nd);
    align_vec(&mut monitor);

    {
        // use std::io::Write;
        // let mut data = Vec::new();
        // let mut writer = lzma_rust::LZMAWriter::new(
        //     lzma_rust::CountingWriter::new(&mut data),
        //     &lzma_rust::LZMA2Options{

        //         ..Default::default()
        //     },
        //     true,
        //     false,
        //     None
        // )?;
        // writer.write_all(&ldr2ndhdr)?;
        // writer.finish()?;
        // drop(writer);

        // let mut ldr2ndhdr = data;

        let len = (0x20 + ldr2ndhdr.len() + (512-1)) & !(512-1);
        ldr2ndhdr.resize(len - 0x20, 0);
    }

    assert_eq!(nand_info.len(), 128);
    assert_eq!(nor_info.len(), 36);
    assert_eq!(fip_flags.len(), 8);
    assert_eq!(chip_conf.len(), 760);

    assert_eq!(bl_ek.len(), 32);
    assert_eq!(root_pk.len(), 512);
    assert_eq!(bl_pk.len(), 512);


    let mut file = Vec::new();


    let mut checksums = [CheckSum::default(); 8];

    // magic 1/2
    file.write_all(&PARAM1_MAGIC1)?;
    file.write_all(&U32Le::new(0x0).0)?;

    // param cksum
    checksums[0].loc = file.len();
    file.write_all(&U32Le::new(0x0).0)?;
    checksums[0].start = file.len();

    file.write_all(&nand_info)?;
    file.write_all(&nor_info)?;
    file.write_all(&fip_flags)?;
    file.write_all(&U32Le::new(chip_conf.len() as u32).0)?;

    //blcp 
    //img chsum
    checksums[1].loc = file.len();
    file.write_all(&0u32.to_le_bytes())?;
    // size
    file.write_all(&U32Le::new(blcp_img.len() as u32).0)?;
    //runaddr
    file.write_all(&U32Le::new(0).0)?;
    //loadaddr
    file.write_all(&U32Le::new(0x0).0)?;
    //param size
    file.write_all(&U32Le::new(0x0).0)?;

    //bl2 
    //img chsum
    checksums[2].loc = file.len();
    file.write_all(&0u32.to_le_bytes())?;
    // size
    file.write_all(&U32Le::new(bl2_img.len() as u32).0)?;
    
    // BLD_IMG_SIZE
    file.write_all(&U32Le::new(0).0)?;

    // param 2 loadaddr
    file.write_all(&U32Le::new(0).0)?;
    // reserved
    file.write_all(&U32Le::new(0).0)?;

    chip_conf.fill(0);
    bl_ek.fill(0);
    root_pk.fill(0);
    bl_pk.fill(0);
    file.write_all(&chip_conf)?;
    file.write_all(&bl_ek)?;
    file.write_all(&root_pk)?;
    file.write_all(&bl_pk)?;

    checksums[0].end = file.len();
    for _ in 0..4{
        file.write_all(&[0; 512])?;
    }

    checksums[1].start = file.len();
    file.write_all(&blcp_img)?;
    checksums[1].end = file.len();

    checksums[2].start = file.len();
    file.write_all(&bl2_img)?;
    checksums[2].end = file.len();



    file.write_all(&PARAM2_MAGIC1)?;
    checksums[3].loc = file.len();
    file.write_all(&U32Le::new(0).0)?;
    checksums[3].start = file.len();
    // reserved
    file.write_all(&U32Le::new(0).0)?;

    //ddr param
    checksums[4].loc = file.len();
    file.write_all(&U32Le::new(0).0)?;
    // load addr
    file.write_all(&U32Le::new(0x0000a400).0)?;
    // size
    file.write_all(&U32Le::new(ddr_param.len() as u32).0)?;
    // reserved
    file.write_all(&U32Le::new(0).0)?;

    // blcp 2nd
    checksums[5].loc = file.len();
    file.write_all(&U32Le::new(0).0)?;
    // load addr
    file.write_all(&U32Le::new(0x0000c400).0)?;
    // size
    file.write_all(&U32Le::new(blcp_2nd.len() as u32).0)?;
    // run addr
    file.write_all(&U32Le::new(0x9fe00000).0)?;

    // monitor
    checksums[6].loc = file.len();
    file.write_all(&U32Le::new(0).0)?;
    // load addr
    file.write_all(&U32Le::new(0x0000fa00).0)?;
    // size
    file.write_all(&U32Le::new(monitor.len() as u32).0)?;
    // run addr
    file.write_all(&U32Le::new(0x80000000).0)?;

    // u-boot
    // reserver 0
    file.write_all(&U32Le::new(0).0)?;
    // load addr
    let load_addr_start = file.len();
    file.write_all(&U32Le::new(0).0)?;
    // reserved 1/2
    file.write_all(&U32Le::new(0).0)?;
    file.write_all(&U32Le::new(0).0)?;


    file.write_all(&[0; 4096 - 16 * 5])?;
    checksums[3].end = file.len();


    checksums[4].start = file.len();
    file.write_all(&ddr_param)?;
    checksums[4].end = file.len();

    checksums[5].start = file.len();
    file.write_all(&blcp_2nd)?;
    checksums[5].end = file.len();

    checksums[6].start = file.len();
    file.write_all(&monitor)?;
    checksums[6].end = file.len();


    let offset = file.len() as u32;
    file[load_addr_start..load_addr_start+4].copy_from_slice(&U32Le::new(offset).0);
    // jump0
    file.write_all(&U32Le::new(0x0001a005).0)?;
    // magic
    file.write_all(&LOADER_2ND_MAGIC_ORIG.0)?;
    checksums[7].loc = file.len();
    file.write_all(&U32Le::new(0).0)?;
    checksums[7].start = file.len();
    // size
    file.write_all(&U32Le::new(ldr2ndhdr.len() as u32 + 0x20).0)?;
    // run addr
    file.write_all(&U64Le::new(0x0000000080200000).0)?;
    //reserved 1/2
    file.write_all(&U32Le::new(0xdeadbeec).0)?;
    file.write_all(&U32Le::new(0x0001a011).0)?;
    file.write_all(&ldr2ndhdr)?;
    checksums[7].end = file.len();




    for cksum in checksums.iter().rev(){
        if cksum.loc == 0{
            continue;
        }
        let val = calc_check_sum(&file[cksum.start..cksum.end]);
        file[cksum.loc..cksum.loc+4].copy_from_slice(&U32Le::new(val).0);
    }

    std::fs::write("fip.bin", file)?;

    Ok(())
}

pub fn dump(path: impl AsRef<std::path::Path>) {
    let file = std::fs::read(path).unwrap();

    let mut addr = 0;

    let param1 = &file[addr..addr + std::mem::size_of::<Param1>()];
    let param1 = unsafe { &*param1.as_ptr().cast::<Param1>() };

    assert_eq!(param1.magic_1, PARAM1_MAGIC1);

    assert_eq!(param1.chip_conf_size.get(), 760);

    let param1_ch =
        &file[addr + offset_of!(Param1, nand_info)..addr + offset_of!(Param1, bl_pk_sig)];
    assert_eq!(param1.param_checksum.get(), calc_check_sum(param1_ch));

    const ZEROD: [u8; 512] = [0; 512];
    assert_eq!(param1.bl_pk_sig, ZEROD);
    assert_eq!(param1.bl2_img_sig, ZEROD);
    assert_eq!(param1.blcp_img_sig, ZEROD);
    assert_eq!(param1.chip_conf_sig, ZEROD);

    std::fs::write("./dump/nand_info.bin", param1.nand_info).unwrap();
    std::fs::write("./dump/nor_info.bin", param1.nor_info).unwrap();
    std::fs::write("./dump/fip_flags.bin", param1.fip_flags).unwrap();
    std::fs::write("./dump/chip_conf.bin", param1.chip_conf).unwrap();
    std::fs::write("./dump/bl_ek.bin", param1.bl_ek).unwrap();
    std::fs::write("./dump/root_pk.bin", param1.root_pk).unwrap();
    std::fs::write("./dump/bl_pk.bin", param1.bl_pk).unwrap();

    addr += std::mem::size_of::<Param1>();

    let blcp_len = param1.blcp_img_size.get() as usize;
    let blcp = &file[addr..addr + blcp_len];
    addr += blcp_len;
    assert_eq!(param1.blcp_img_check_sum.get(), calc_check_sum(blcp));
    std::fs::write("./dump/blcp.bin", blcp).unwrap();

    let bl2_len = param1.bl2_img_size.get() as usize;
    let bl2 = &file[addr..addr + bl2_len];
    addr += bl2_len;
    assert_eq!(param1.bl2_img_check_sum.get(), calc_check_sum(bl2));
    std::fs::write("./dump/bl2.bin", bl2).unwrap();

    // param 2
    let param2 = &file[addr..addr + std::mem::size_of::<Param2>()];
    let param2_ch = &param2[offset_of!(Param2, _reserved1)..];

    let param2 = unsafe { &*param2.as_ptr().cast::<Param2>() };
    assert_eq!(param2.magic_1, PARAM2_MAGIC1);

    assert_eq!(param2.param2_check_sum.get(), calc_check_sum(param2_ch));

    addr += std::mem::size_of::<Param2>();

    let ddr_param_len = param2.ddr_param_size.get() as usize;
    let ddr_param = &file[addr..addr + ddr_param_len];
    addr += ddr_param_len;
    assert_eq!(param2.ddr_param_check_sum.get(), calc_check_sum(ddr_param));
    std::fs::write("./dump/ddr_param.bin", ddr_param).unwrap();

    let blcp_2nd_len = param2.blcp_2nd_size.get() as usize;
    let blcp_2nd = &file[addr..addr + blcp_2nd_len];
    addr += blcp_2nd_len;
    assert_eq!(param2.blcp_2nd_check_sum.get(), calc_check_sum(blcp_2nd));
    std::fs::write("./dump/blcp_2nd.bin", blcp_2nd).unwrap();

    let monitor_len = param2.monitor_size.get() as usize;
    let monitor = &file[addr..addr + monitor_len];
    addr += monitor_len;
    assert_eq!(param2.monitor_check_sum.get(), calc_check_sum(monitor));
    std::fs::write("./dump/monitor.bin", monitor).unwrap();
    // decompress_lz4(monitor);

    let ldr2ndhdr = &file[addr..addr + std::mem::size_of::<Ldr2ndHdr>()];
    let ldr2ndhdr_ch = &file[addr + offset_of!(Ldr2ndHdr, size)..];
    let ldr2ndhdr = unsafe { &*ldr2ndhdr.as_ptr().cast::<Ldr2ndHdr>() };
    assert_eq!(ldr2ndhdr.check_sum.get(), calc_check_sum(ldr2ndhdr_ch));

    // ldr2ndhdr.size;
    // addr += std::mem::size_of::<Param2>();

    let start = param2.loader_2nd_loadaddr.get() as usize;
    assert_eq!(start, addr); // no idea if this is true but like it seems like it should be
    let end = start + ldr2ndhdr.size.get() as usize;
    let ldr2nd = &file[start..end];
    let ldr2nd_body = &ldr2nd[std::mem::size_of::<Ldr2ndHdr>()..];

    assert!(end == file.len());

    assert!([
        LOADER_2ND_MAGIC_LZ4,
        LOADER_2ND_MAGIC_LZMA,
        LOADER_2ND_MAGIC_ORIG
    ]
    .contains(&ldr2ndhdr.magic));



    match ldr2ndhdr.magic {
        LOADER_2ND_MAGIC_LZ4 => {
            std::fs::write("./dump/dr2nd.lz4", ldr2nd_body).unwrap();
        }
        LOADER_2ND_MAGIC_LZMA => {
            std::fs::write("./dump/ldr2nd.bin.lzma", ldr2nd_body).unwrap();
            std::process::Command::new("lzma")
            .args(["--decompress", "--single-stream", "./dump/ldr2nd.bin.lzma"]).spawn().unwrap();
        }
        LOADER_2ND_MAGIC_ORIG => {
            std::fs::write("./dump/ldr2nd.bin", ldr2nd_body).unwrap();
        }
        _ => unreachable!(),
    }

    // addr = end;

    // println!("{:#?}", param1);
    println!("{:#016x?}", ldr2ndhdr);
}

// fn compress_lmza(data: &[u8]) -> Vec<u8>{
    // lzma_rust::
// }

// fn decompress_lzma(data: &[u8]) -> Vec<u8>{
//     let mut un_data = Vec::new();
//     lzma::read(data).unwrap().read_to_end(&mut un_data).unwrap();
//     un_data
// }
