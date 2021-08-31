use std::fs::File;
use std::io::prelude::*;
use backtrace::Backtrace;

/// Simple macro to dump in a standard way all the pairs key:value
macro_rules! dump {
    ($file:expr, $key:expr, $val:expr) => {
        write!($file, "{},{:?}\n", $key, $val).expect("Cannot write to file.");
    };
}

pub fn get_folder() -> String {
    // Find the root of the repository
    let mut currdir = get_path("ensmallen_graph");
    // Build the path to the folder for the tests
    currdir.push("fuzzing");
    currdir.push("unit_tests");
    // Create a random directory
    let path = graph::test_utilities::random_path(currdir.to_str());
    std::fs::create_dir_all(&path).unwrap();
    return path;
}

/// Return a path stopping at the first occurence of wanted_folder.
pub fn get_path(wanted_folder: &str) -> std::path::PathBuf {
    let curr_dir = std::env::current_dir().unwrap().canonicalize().unwrap();

    let mut new_path = std::path::PathBuf::new();

    for part in curr_dir.iter() {
        new_path.push(part);
        if part == wanted_folder{
            break
        }
    }

    new_path
}


/// Dump the informations about the panic
pub fn dump_panic_info(path: String, info: &std::panic::PanicInfo){
    let mut file = File::create(path).unwrap();
    if let Some(s) = info.location() {
        dump!(file, "file", s.file());
        dump!(file, "line", s.line());
        dump!(file, "col",  s.column());
    }

    if let Some(s) = info.message() {
        dump!(file, "message", s);
    }
}

pub fn dump_backtrace(path: &str) {
    let current_backtrace = Backtrace::new();
    std::fs::write(
        format!("{}/backtrace.txt", &path),
        format!("{:#4?}", &current_backtrace),
    )
    .expect("Cannot write the backtrace file");
}
