
/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub fn handle_panics_meta_test(info: Option<&std::panic::PanicInfo>, data: MetaParams, sig_num: Option<i32>) {
    let path = handle_panics_from_vec(info, data.from_vec.clone(), None);
    dump_backtrace(&path);
    
    std::fs::write(format!("{}/meta_test.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the metatest file");
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub fn handle_panics_meta_test_once_loaded(
    info: Option<&std::panic::PanicInfo>,
    data: MetaParams,
    graph: Graph,
    trace: Option<Vec<String>>,
) {    
    let path = handle_panics_from_vec_once_loaded(info, data.from_vec.clone(), graph);
    dump_backtrace(&path);

    std::fs::write(format!("{}/meta_test.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the metatest file");
    
    std::fs::write(format!("{}/internal_trace.txt", &path), format!("{:#4?}", &trace))
        .expect("Cannot write the internal trace file");
}