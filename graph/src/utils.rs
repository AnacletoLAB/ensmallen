use super::*;
use indicatif::{ProgressBar, ProgressStyle};

#[macro_export]
/// Take a vector and make it a None if its empty, Some(vector) otherwise
macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

pub fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        let candidate_iterations = total_iterations as u64 / 1000;
        let candidate_iterations = candidate_iterations.max(1);
        pb.set_draw_delta(candidate_iterations);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

/// Validated the provided features.
///
/// Specifically, the features must:
/// - Be provided for all of the expected elements.
/// - Be non-empty.
/// - Be of a consistent size, that is the number of features for each element must be equal.
///
/// # Arguments
/// * `features`: Vec<Vec<f64>> - The features to validate.
/// * `expected_elements_number`: usize - The number of expected elements.
pub(crate) fn validate_features(
    features: &Vec<Vec<f64>>,
    expected_elements_number: usize,
) -> Result<(), String> {
    if features.len() != expected_elements_number {
        return Err(format!(
            concat!(
                "The expected features vector length was expected to be {}, ",
                "but is {}."
            ),
            expected_elements_number,
            features.len()
        ));
    }
    let expected_node_features_length = features.first().unwrap().len();
    if expected_node_features_length == 0 {
        return Err("The node features length must be greater than zero.".to_string());
    }
    for node_features in features.iter() {
        if expected_node_features_length != node_features.len() {
            return Err(format!(
                concat!(
                    "The node features length needs to be consistent: the expected ",
                    "size was {} while the found length was {}."
                ),
                expected_node_features_length,
                node_features.len()
            ));
        }
    }
    Ok(())
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
            unsafe{self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst)};
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
/// # Example
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::validate_weight;
/// assert!(validate_weight(0.0).is_err());
/// assert!(validate_weight(-1.0).is_ok());
/// assert!(validate_weight(2.0).is_ok());
/// assert_eq!(validate_weight(2.0).unwrap(), 2.0);
/// ```
///
pub fn validate_weight(weight: WeightT) -> Result<WeightT, String> {
    if weight.is_finite() && weight != 0.0 {
        Ok(weight)
    } else {
        Err(format!(
            "The weight is '{}' but the weights must be non-zero and finite.",
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
/// # Example
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
    weight
        .parse::<WeightT>()
        .map_err(|_| format!("Cannot parse weight {} as a float.", weight))
}

pub trait ArgMax<T> {
    fn argmax(&self) -> Option<(usize, T)>;
}

impl<T: PartialOrd + Copy> ArgMax<T> for Vec<T> {
    fn argmax(&self) -> Option<(usize, T)> {
        self.iter()
            .enumerate()
            .fold(None, |current_max, (i, &value)| {
                current_max.map_or(Some((i, value)), |(j, current_max_value)| {
                    Some(if value > current_max_value {
                        (i, value)
                    } else {
                        (j, current_max_value)
                    })
                })
            })
    }
}
