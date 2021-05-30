use arbitrary::{Arbitrary, Unstructured};
use graph_harness::*;
use indicatif::ProgressBar;
use std::fs::metadata;


fn test_file(file_name: String, number_of_test_runs:u64) {
    println!("Running '{}'", &file_name);
    let mut data = std::fs::read(file_name.clone()).expect("Cannot read the file");
    let mut raw_data = Unstructured::new(&mut data);

    let maybe_params = QueueParams::arbitrary(&mut raw_data);

    println!("{:#4?}", maybe_params);
    let bar = ProgressBar::new(number_of_test_runs);
    if let Ok(params) = maybe_params {
        for _ in (0..number_of_test_runs) {

            let harness_result = queue_harness(params.clone());

            bar.inc(1);
        }
    }
    bar.finish();
    println!("Executed successfully from '{}'", &file_name);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut number_of_test_runs = 1;
    if args.len() < 2 {
        println!("Usage: debug PATH_TO_FILE_TO_DEBUG [number of test runs (defualt: 1)]");
        return;
    }
    if args.len() > 2 {
        number_of_test_runs = args[2].parse::<u64>().unwrap();
    }

    let path = &args[1];
    let md = metadata(path).expect("Cannot access the given file_path");

    // If its only one file, run it
    if md.is_file() {
        test_file(path.clone(), number_of_test_runs);
        return;
    }

    // If it's a dir, run all the files in the folder.
    for filename in std::fs::read_dir(path).expect("CANNOT READ CORPUS") {
        let filename = filename.expect("Cannot get file in dir").path();
        if metadata(&filename).unwrap().is_file() {
            test_file(
                filename.into_os_string().into_string().unwrap(), 
                number_of_test_runs
            );
        }
    }
}
