use libcodeanalysis::*;

fn main() {
    // Generate the bindings
    gen_bindings("../graph/src", "../bindings/python/src/auto_generated_bindings.rs", "../bindings/python/ensmallen/__init__.py");

    // Format the files
    assert!(
        std::process::Command::new("cargo")
            .args(&["fmt"])
            .current_dir("../bindings/python")
            .status()
            .expect("Could not run format on the python bindings")
            .success(),
        "The cargo format failed and returned non-zero exit status"
    );
}