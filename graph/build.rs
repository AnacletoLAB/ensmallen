// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/bad_code.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/bad_code.c")
        .flag("-march=native")
        .flag("-O3")
        .compile("bad_code.a");
}