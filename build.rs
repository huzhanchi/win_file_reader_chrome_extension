use std::env;
use std::path::Path;
use fs_extra::file::{copy, CopyOptions};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let src_path = Path::new(&manifest_dir).join("com.example.nativeapp.json");
    let dest_path = Path::new(&out_dir).join("../../../com.example.nativeapp.json");

    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy(src_path, dest_path, &options).expect("Failed to copy com.example.nativeapp.json");
    
    println!("cargo:rerun-if-changed=com.example.nativeapp.json");
}