use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};


fn main() -> Result<(), Box<dyn Error>> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-canged=build.rs");
    println!("cargo:rerun-if-changed=memory.x");

    // link to `librt.a`
    fs::copy("librt.a", out.join("librt.a"))?; // <- NEW!
    println!("cargo:rustc-link-lib=static=rt");

    // rebuild if `librt.a` changed
    println!("cargo:rerun-if-changed=librt.a");


    Ok(())
}