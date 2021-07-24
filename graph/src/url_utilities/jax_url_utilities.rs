use super::*;
/// Returns whether the given node name respects the JAX nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let jax_node_name = "JAX:000046";
/// let not_jax_node_name = "PizzaQuattroStagioni";
/// assert!(is_valid_jax_node_name(jax_node_name));
/// assert!(!is_valid_jax_node_name(not_jax_node_name));
/// ```
pub fn is_valid_jax_node_name(node_name: &str) -> bool {
    is_valid_node_name_from_seeds(
        node_name,
        Some(&["JAX"]),
        Some(10),
        Some(":"),
        None,
        None,
        Some(6),
    )
    .is_ok()
}

/// Returns URL from given JAX node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a JAX node name and
/// may cause a panic if the aforementioned assumption is not true.
pub(crate) unsafe fn format_jax_url_from_node_name(node_name: &str) -> String {
    format_url_from_node_name(
        "https://www.jax.org/strain/{node_name}",
        node_name,
        Some(":"),
    )
}