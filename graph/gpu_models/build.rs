use std::process::Command;

fn main() {
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