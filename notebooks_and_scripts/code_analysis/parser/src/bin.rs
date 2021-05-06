use parser::*;
use std::fs;


const BLACKLIST: &'static [&'static str] = &[
    "utils.rs", // macro rules
    "types.rs", // traits
    "walks.rs", // extern
    "lib.rs",   // mods
    "core.c",   // it is C
];

fn main() {
    'outer: for path in fs::read_dir("../../../graph/src").unwrap() {
        let path = path.unwrap().path().into_os_string().into_string().unwrap();
        println!("DAAAD WHATS FOR BREAKFAST {}", path);

        for deny in BLACKLIST.iter(){
            if path.contains(deny) {
                println!("SKIPPING");
                continue 'outer;
            }
        }

        let contents = fs::read_to_string(path)
            .expect("File not found");
        let (_reminder, parsing) = Module::parse(contents.as_bytes());
        println!("{:#4?}", parsing);
    }
}