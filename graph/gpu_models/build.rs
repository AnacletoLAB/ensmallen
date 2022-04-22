use std::process::Command;

fn main() {
    // TODO!: add recurive walk of the dir so that we detect generally the
    // change of any file in the ptx folder
    println!("cargo:rerun-if-changed=../cuda_kernels/src/mod.rs");
    

    Command::new("cargo")
        .args([
            "rustc",
            "--release",
            "--target=nvptx64-nvidia-cuda",
            "--",
            "-Zcrate-attr=no_main",
        ])
        .current_dir("../cuda_kernels")
        .status()
        .expect("Could not compile the PTX for the current crate.");
}
