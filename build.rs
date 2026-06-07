fn main() {
    cc::Build::new()
        .compiler("riscv64-linux-gnu-gcc")
        .flag("-march=rv64gc")
        .flag("-mabi=lp64d")
        .file("src/boot/entry.S")
        .compile("entry");

    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rerun-if-changed=src/boot/entry.S");
    println!("cargo:rerun-if-changed=linker.ld");
}
