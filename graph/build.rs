// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/core.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/core.c")
        .flag("-march=native")
        .flag("-mtune=native")
        .flag("-msse4.1")
        .flag("-mavx")
        .flag("-mavx2")
        .flag("-mlzcnt")
        .flag("-O3")
        .compile("core.a");
}
