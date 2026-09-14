use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from("target/x86_64-unknown-none/debug/kernel");
    let image = PathBuf::from("target/untitled2.img");

    bootloader::DiskImageBuilder::new(kernel)
        .create_bios_image(&image)
        .expect("failed to create BIOS image");

    println!("cargo:rerun-if-changed=kernel/src/main.rs");
}