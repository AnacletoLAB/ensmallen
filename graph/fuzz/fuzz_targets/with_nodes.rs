#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;

mod utils;
use utils::*;

fuzz_target!(|data: &[u8]| {
    // writing to file is REALLY slow but
    // the from_csv function read the file line by line
    // and we want coverage on this function
    // so I don't think there are any workarounds or mocking

    // TEST Directed
    
    // to speedup the fuzzing it might be sensible to mount the /tmp
    // to a ramdisk https://wiki.gentoo.org/wiki/Tmpfs
    let fname = Path::new("/tmp").join(random_string(64));
    let filename = fname.to_str().unwrap();

    // Write the fuzzer output to the file
    let mut file = File::create(&filename).unwrap();
    file.write_all(data).unwrap();
    
    
    let edge_path = "tests/data/het_graph_edges.tsv";
    let graph = graph::Graph::from_csv(
        edge_path,
        "subject",
        "object",
        false,
        None,
        None,
        Some("weight"),
        Some(1.0),
        Some(&filename),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    );

    if graph.is_ok(){
        let _ = graph.unwrap().walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false));
    }
    
    let _ = remove_file(&filename).unwrap();
});
