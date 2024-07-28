use std::{io::Write, path::{Path, PathBuf}};

pub const PARAM1_MAGIC1: [u8; 8] = [b'C', b'V', b'B', b'L', b'0', b'1', b'\n', b'\0'];

pub const IMAGE_ALIGN: usize = 512;

#[repr(C)]
pub struct U32Le([u8; 4]);

impl U32Le {
    pub fn new(val: u32) -> Self{
        Self(val.to_le_bytes())
    }
}

#[repr(C)]
pub struct U64Le([u8; 8]);

impl U64Le {
    pub fn new(val: u64) -> Self{
        Self(val.to_le_bytes())
    }
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

#[derive(Default)]
struct Options{
    bl: Option<PathBuf>,
    s2: Option<PathBuf>,
    out: Option<PathBuf>,
}

fn help(){
    println!("
minimal fip.bin maker for MilkV-Duos

    -bl <path> 
    -s2 <path> 
    -o <path> (default 'fip.bin')
    -help Print this message

-b2 is needed");
}

fn main() -> std::io::Result<()>{

    let mut options = Options::default();

    let mut args = std::env::args().skip(1);

    while let Some(next) = args.next(){
        match next.as_str().trim(){
            "-bl" => if let Some(p) = args.next(){
                options.bl = Some(p.into())
            }else{
                println!("Expected path found nothing. type -help for help");
                std::process::exit(-1);
            }
            "-s2" => if let Some(p) = args.next(){
                options.s2 = Some(p.into())
            }else{
                println!("Expected path found nothing. type -help for help");
                std::process::exit(-1);
            }
            "-o" => if let Some(p) = args.next(){
                options.out = Some(p.into())
            }else{
                println!("Expected path found nothing. type -help for help");
                std::process::exit(-1);
            }
            "-help" => {
                help();
                return Ok(())
            }
            arg => {
                println!("Unknown argument {arg:?}. type -help to see options");
                std::process::exit(-1);
            }
        }
    }

    if matches!(&options.bl, None){
        println!("-b2 is needed");
        std::process::exit(-1);
    }

    let mut bl_img = if let Some(p) = &options.bl{
        std::fs::read(p)?
    }else{
        Vec::new()
    };
    let mut s2 = if let Some(p) = &options.s2{
        std::fs::read(p)?
    }else{
        Vec::new()
    };

    align_vec(&mut bl_img);
    align_vec(&mut s2);
    
    let mut file = Vec::new();

    let mut checksums = [CheckSum::default(); 2];

    // magic 1/2
    file.write_all(&PARAM1_MAGIC1)?;
    file.write_all(&U32Le::new(0x0).0)?;

    // param cksum
    checksums[0].loc = file.len();
    file.write_all(&U32Le::new(0x0).0)?;
    checksums[0].start = file.len();


    // nand info
    file.write_all(&[0x0; 128])?;
    // nor info
    file.write_all(&[0xFF; 36])?;
    // fip flags
    file.write_all(&[0; 8])?;
    // chip_conf len
    file.write_all(&U32Le::new(760).0)?;

    //blcp 
    //img chsum
    file.write_all(&0u32.to_le_bytes())?;
    // size
    file.write_all(&U32Le::new(0).0)?;
    //runaddr
    file.write_all(&U32Le::new(0).0)?;
    //loadaddr
    file.write_all(&U32Le::new(0x0).0)?;
    //param size
    file.write_all(&U32Le::new(0x0).0)?;

    //bl2 
    //img chsum
    checksums[1].loc = file.len();
    file.write_all(&0u32.to_le_bytes())?;
    // size
    file.write_all(&U32Le::new(bl_img.len() as u32).0)?;
    
    // BLD_IMG_SIZE
    file.write_all(&U32Le::new(0).0)?;

    // param 2 loadaddr
    file.write_all(&U32Le::new(0).0)?;
    // param 2 loadaddr
    file.write_all(&U32Le::new(s2.len() as u32).0)?;

    file.write_all(&[0; 760])?;

    // bl_ek
    file.write_all(&[0; 32])?;
    // root_pk
    file.write_all(&[0; 512])?;
    // bl_pk
    file.write_all(&[0; 512])?;

    checksums[0].end = file.len();
    for _ in 0..4{
        file.write_all(&[0; 512])?;
    }

    // empty blcp
    file.write_all(&[])?;

    checksums[1].start = file.len();
    file.write_all(&bl_img)?;
    checksums[1].end = file.len();

    file.write_all(&s2)?;

    for cksum in checksums.iter().rev(){
        if cksum.loc == 0{
            continue;
        }
        let val = calc_check_sum(&file[cksum.start..cksum.end]);
        file[cksum.loc..cksum.loc+4].copy_from_slice(&U32Le::new(val).0);
    }

    let out = if let Some(p) = &options.out{
        p.as_path()
    }else{
        Path::new("fip.bin")
    };
    std::fs::write(out, file)?;

    Ok(())
}