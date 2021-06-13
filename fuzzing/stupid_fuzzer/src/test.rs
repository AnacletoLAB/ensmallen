use arbitrary::{Arbitrary, Unstructured};
use graph_harness::*;
use std::fs::metadata;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif::{ParallelProgressIterator, ProgressIterator};


use clap::{AppSettings, Clap};

/// This entrypoint is used to run and debug the corpus generated from the fuzzer.
///
/// Example:
/// cargo run --release --bin test -- meta_test ../corpus/meta_test
///
///
#[derive(Clap)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Which harness to use to read the corpus files and run the tests
    #[clap(possible_values(&["from_csv", "from_vec", "meta_test"]))]
    harness_name: String,

    /// Path to the folder or file to test.
    /// If a file is provided, it will test it.
    /// If a folder is provided, every file in the folder (non recursively) will be tested.
    corpus_path: String,

    /// How many times every file will be tested.
    /// This is used for coping with non-deterministic bugs.
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "1")]
    number_of_iterations: u64,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

macro_rules! define_harness {
    ($harness_struct:ty, $harness_func:ident) => {{
        |file_name: String,  number_of_test_runs:u64, verbose: bool| {
            if verbose {
                println!("Testing file: '{}'", &file_name);
            }
            // read the file
            let mut data = std::fs::read(file_name.clone())
                .expect(&format!("Cannot read the file '{}'", file_name));
            // build the parameters
            let mut raw_data = Unstructured::new(&mut data);
            let maybe_params = <$harness_struct>::arbitrary(&mut raw_data);

            if let Ok(params) = maybe_params {
                for _ in (0..number_of_test_runs) {
                    let _harness_result = $harness_func(params.clone());
                }
            }
        }}
    };
}

fn main() {
    // validate and parse the arguments
    let opts: Opts = Opts::parse();

    // Dispatch the right function
    let test_file_function: fn(String, u64, bool) = match opts.harness_name.as_str() {
        "from_csv" => define_harness!(FromCsvHarnessParams, from_csv_harness),
        "from_vec" => define_harness!(FromVecHarnessParams, from_vec_harness),
        "meta_test" => define_harness!(MetaParams, meta_test_harness),
        _ => unreachable!("The given harness name is not one of the supported one, this should not happens since the argument parsing should check the validity."),
    };

    // get the metadata of the path to check that we can access it
    // and wheter it's a file or a folder
    let md = metadata(&opts.corpus_path)
        .expect(&format!(
            "Cannot access the given corpus_path: '{}'",
            opts.corpus_path,
        ));

    // If its only one file, run it
    if md.is_file() {
        test_file_function(opts.corpus_path.clone(), opts.number_of_iterations);
        return;
    }

    // Get the filenames of everyting in the folder
    let filenames = std::fs::read_dir(opts.corpus_path.clone())
        .expect("CANNOT READ CORPUS FOLDER")
        .into_iter().map(|filename| 
            filename.expect("Cannot get file in dir")
            .path().into_os_string()
            .into_string().unwrap()
        ).collect::<Vec<String>>();

    // If it's a dir, run all the files in the folder.
    filenames.into_iter()
        .filter(|filename| {
            metadata(&filename).unwrap().is_file()
        })
        .for_each(move |filename| {
            test_file_function(
                filename, 
                opts.number_of_iterations,
            );
        });
}
