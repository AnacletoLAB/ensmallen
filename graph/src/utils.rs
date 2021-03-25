use super::*;
use indicatif::{ProgressBar, ProgressStyle};

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

/// Return one-hot encoded version of given vector (optionally multi-label).
///
/// # Arguments
///
/// * `values`: Vec<N> - Vector, optionally multilabel, to one-hot encode.
/// * `max_possible_value`: N - Maximum possible value.
///
pub(crate) fn one_hot_encode<N: ToFromUsize>(values: Vec<N>, max_possible_value: N) -> Vec<N> {
    let mut one_hot_encoded_vector = vec![N::from_usize(0); N::to_usize(max_possible_value)];
    values.into_iter().for_each(|value| {
        one_hot_encoded_vector[N::to_usize(value)] = N::from_usize(1);
    });
    one_hot_encoded_vector
}

#[cfg(test)]
mod tests {
    use super::one_hot_encode;

    #[test]
    fn test_one_hot_encode() {
        let max_label_value = 5;
        let single_label: Vec<usize> = vec![3];
        let multi_label: Vec<usize> = vec![0, 2, 4];
        let expected_single_label_result = vec![0, 0, 0, 1, 0];
        let expected_multi_label_result = vec![1, 0, 1, 0, 1];

        assert_eq!(
            one_hot_encode(single_label, max_label_value),
            expected_single_label_result
        );
        assert_eq!(
            one_hot_encode(multi_label, max_label_value),
            expected_multi_label_result
        );
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
        Err(format!(
            "The weight is '{}' but the weights must be strictly positives and finite.",
            weight
        ))
    }
}

pub fn parse_weight(weight: String) -> Result<WeightT, String> {
    match weight.parse::<WeightT>() {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("Cannot parse weight {} as a float.", weight)),
    }
}
