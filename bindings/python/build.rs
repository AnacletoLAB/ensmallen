use std::fs;
use std::io::prelude::*;


// Example custom build script.
fn main() {
    // Tell cargo to re-run the build script iff one of the env var changed
    let env_vars = [
        "CARGO_CFG_TARGET_FEATURE",
        "TARGET",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ENDIAN",

        // these are just optional infos, so we might want to remove them?
        "HOST",
        "OPT_LEVEL",
        "RUSTUP_TOOLCHAIN",
    ];
    for env_var in env_vars {
        println!("cargo:rerun-if-env-changed={}", env_var);
    }

    let content = format!(
r#"
# Library Compilation flags
FLAGS = {:?}
TARGET_TRIPLE = {target_triple:?}
CPU_ARCH = {cpu_arch:?}
OS = {os:?}
ENDIANESS = {endianess:?}

# Compilation host info
HOST = {host:?}
OPT_LEVEL = {opt_level:?}
TOOLCHAIN = {toolchain:?}
"#,
        flags = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap().split(",").collect::<Vec<_>>(),
        target_triple = std::env::var("TARGET").unwrap(),
        cpu_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
        os = std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
        endianess = std::env::var("CARGO_CFG_TARGET_ENDIAN").unwrap(),
        host = std::env::var("HOST").unwrap(),
        opt_level = std::env::var("OPT_LEVEL").unwrap(),
        toolchain = std::env::var("RUSTUP_TOOLCHAIN").unwrap(),
    );

    // Check if the file is already correct, this avoid modifing the ctime of 
    // the file and thus avoid useless recompilations
    if let Ok(file_content) = fs::read_to_string("./ensmallen/compilation_flags.py") {
        if content == file_content {
            return;
        }
    }
    
    let mut file = fs::File::create("./ensmallen/compilation_flags.py")
        .expect("Could not open / create the file compilation_flags.py");
    file.write_all(content.as_bytes()).expect("Could not write to the file compilation_flags.py");
}
