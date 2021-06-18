use arbitrary::{Arbitrary, Unstructured};
use graph_harness::*;
use std::fs::metadata;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif::{ParallelProgressIterator, ProgressIterator};


use clap::{AppSettings, Clap};

/// This entrypoint is used to trace the 
///
/// Example:
/// cargo run --release --bin trace -- ../corpus/meta_test
///
///
#[derive(Clap)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Path to the folder or file to trace.
    /// If a file is provided, it will trace it.
    /// If a folder is provided, every file in the folder (non recursively) will be traced.
    corpus_path: String,
}

fn test_file_function(file_name: String) {
    println!("Testing file: '{}'", &file_name);
    // read the file
    let mut data = std::fs::read(file_name.clone())
        .expect(&format!("Cannot read the file '{}'", file_name));
    // build the parameters
    let mut raw_data = Unstructured::new(&mut data);
    let maybe_params = MetaParams::arbitrary(&mut raw_data);

    if let Ok(params) = maybe_params {
        let _ = meta_test_trace(params);
    }
}

fn main() {
    // validate and parse the arguments
    let opts: Opts = Opts::parse();

    // get the metadata of the path to check that we can access it
    // and wheter it's a file or a folder
    let md = metadata(&opts.corpus_path)
        .expect(&format!(
            "Cannot access the given corpus_path: '{}'",
            opts.corpus_path,
        ));

    // If its only one file, run it
    if md.is_file() {
        test_file_function(opts.corpus_path.clone());
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
            );
        });
}
