use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        pb.set_draw_delta(10000);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}
