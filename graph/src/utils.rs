use super::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
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
pub(crate) fn not_one(weight: WeightT) -> bool {
    (weight - 1.0).abs() > WeightT::EPSILON
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
                self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst);
            (min_edge_id..max_edge_id).collect::<Vec<EdgeT>>()
        } else {
            vec![edge_id]
        }
    }
}

/// Return validated weight.
///
/// A weight, to be valid in the context of graph machine learning
/// as we have defined, must be strictly positive and non infinite.
///
/// # Arguments
///
/// * `weight`: WeightT - The weight to validate.
///
/// # Examples
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::validate_weight;
/// assert!(validate_weight(0.0).is_err());
/// assert!(validate_weight(-1.0).is_err());
/// assert!(validate_weight(2.0).is_ok());
/// assert_eq!(validate_weight(2.0).unwrap(), 2.0);
/// ```
///
pub fn validate_weight(weight: WeightT) -> Result<WeightT, String> {
    if weight.is_finite() && weight > 0.0 {
        Ok(weight)
    } else {
        Err(format!(
            "The weight is '{}' but the weights must be strictly positives and finite.",
            weight
        ))
    }
}

/// Return given weight parsed from string to float.
///
/// # Arguments
///
/// * `weight`: String - The weight to be parsed.
///
/// # Examples
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::parse_weight;
/// assert!(parse_weight("0.0".to_string()).is_ok());
/// assert!(parse_weight("-1.0".to_string()).is_ok());
/// assert!(parse_weight("2.0".to_string()).is_ok());
/// assert!(parse_weight("2ghgjh.0".to_string()).is_err());
/// assert_eq!(parse_weight("2.0".to_string()).unwrap(), 2.0);
/// ```
///
pub fn parse_weight(weight: String) -> Result<WeightT, String> {
    match weight.parse::<WeightT>() {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("Cannot parse weight {} as a float.", weight)),
    }
}
