use libcodeanalysis::*;

fn main() {
    // Generate the bindings
    gen_pyi("../bindings/python/src/", "../bindings/python/ensmallen/ensmallen.pyi");

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