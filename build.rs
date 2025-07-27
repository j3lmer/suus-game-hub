// build.rs
fn main() {
    cc::Build::new()
        .file("csrc/2048.c")
        .file("csrc/2048_wrapper.c") // Both are compiled into separate .o files
        .define("NDEBUG", None)
        .include("csrc") // Make sure the compiler can find 2048.h
        .compile("2048");

    println!("cargo:rustc-link-lib=static=2048");
    println!("cargo:rerun-if-changed=csrc/2048.c");
    println!("cargo:rerun-if-changed=csrc/2048_wrapper.c");
    println!("cargo:rerun-if-changed=csrc/2048.h"); // Add this line for the new header
}
