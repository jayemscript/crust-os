use std::path::PathBuf;

fn main() {
    // Directory where Cargo stores generated files.
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("OUT_DIR not set")
    );

    // Path to the compiled bare-metal Rust kernel.
    let kernel = PathBuf::from(
        std::env::var_os(
            "CARGO_BIN_FILE_CRUST_OS_KERNEL_crust-os-kernel"
        )
        .expect("Kernel binary path not found")
    );

    // Generate a BIOS-bootable disk image.
    let bios_path = out_dir.join("crust-os-bios.img");

    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("Failed to create BIOS disk image");

    // Make the image path available to the host launcher.
    println!(
        "cargo:rustc-env=BIOS_PATH={}",
        bios_path.display()
    );
}