use arbitrary::{Arbitrary, Unstructured};
use std::fs::metadata;
use graph_harness::{from_vec_harness, FromVecHarnessParams};
use indicatif::ProgressBar;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use indicatif::ProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

const BATCH_SIZE: u64 = 1;

pub fn get_loading_bar(verbose: bool, desc: &str, total_iterations: u64) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{wide_bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta_precise}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

fn test_file(file_name: String, number_of_test_runs:u64) {
    println!("Running '{}'", &file_name);
    let mut data = std::fs::read(file_name.clone()).expect("Cannot read the file");
    let mut raw_data = Unstructured::new(&mut data);
    let maybe_params = FromVecHarnessParams::arbitrary(&mut raw_data);
    println!("{:#4?}", maybe_params);
    let bar = get_loading_bar(true, "Running SigAbortTest", number_of_test_runs / BATCH_SIZE);

    if let Ok(params) = maybe_params {
        (0..number_of_test_runs / BATCH_SIZE).into_par_iter().map(|x| x).progress_with(bar).for_each(
            |x| {
                let _ = from_vec_harness(params.clone());
            }
        );
    }
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
        test_file(filename.into_os_string().into_string().unwrap(), number_of_test_runs);
    }
}
