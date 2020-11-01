use super::*;
use indicatif::{ProgressBar, ProgressStyle};

#[macro_export]
/// Macro that computes the maximum between two numbers
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a >= $b {
            $a
        } else {
            $b
        }
    };
}
#[macro_export]
/// Macro that computes the minimum between two numbers
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

pub(crate) fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        pb.set_draw_delta(total_iterations as u64 / 100);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

/// Return true if the given weight is near to one.
pub(crate) fn not_one(weight:WeightT)->bool {
    (weight - 1.0).abs() > f32::EPSILON
}

impl Graph {
    /// Return vector of edges to be inserted in the holdout.
    pub(crate) fn compute_edge_ids_vector(
        &self,
        edge_id: EdgeT,
        src: NodeT,
        dst: NodeT,
        include_all_edge_types: bool,
    ) -> Vec<EdgeT> {
        if include_all_edge_types {
            let (min_edge_id, max_edge_id) =
                self.get_unchecked_edge_types_min_max_edge_ids(src, dst);
            (min_edge_id..max_edge_id).collect::<Vec<EdgeT>>()
        } else {
            vec![edge_id]
        }
    }
}

pub fn validate_weight(weight: WeightT) -> Result<WeightT, String> {
    if weight.is_finite() && weight > 0.0 {
        Ok(weight)
    } else {
        Err(format!("The weight is '{}' but the weights must be strictly positives and finite.", weight))
    }
}

pub fn parse_weight(weight: Option<String>) -> Result<Option<WeightT>, String> {
    match weight {
        None => Ok(None),
        Some(w) => match w.parse::<WeightT>() {
            Ok(val) => match validate_weight(val) {
                Ok(val) => Ok(Some(val)),
                Err(e) => Err(e)
            },
            Err(_) => Err(format!("Cannot parse weight {} as a float.", w)),
        },
    }
}