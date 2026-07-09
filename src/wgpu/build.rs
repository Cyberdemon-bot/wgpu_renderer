use std::env;
use std::path::PathBuf;

fn main() 
{
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/native_window.rs");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path: PathBuf = PathBuf::from(&crate_dir)
        .join("..")
        .join("rust_header.h");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to read cbindgen.toml");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(out_path);
}