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
pub fn validate_features(
    features: &[Vec<f64>],
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