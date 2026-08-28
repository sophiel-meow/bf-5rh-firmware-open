use std::env;
use std::fs;
use std::path::Path;

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_rs_path = Path::new(&manifest_dir).join("src/lib.rs");
    let lib_rs = fs::read(&lib_rs_path).expect("read abi/src/lib.rs");
    let hash = crc32(&lib_rs);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("build_hash.rs");
    fs::write(dest, format!("pub const BUILD_HASH: u32 = 0x{hash:08X};\n"))
        .expect("write build_hash.rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
