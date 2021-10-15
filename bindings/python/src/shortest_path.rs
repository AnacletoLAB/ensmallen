use super::*;
use rayon::prelude::*;

macro_rules! impl_get_shortest_paths_node_embedding {
    ($($dtype:ty),*) => {

    #[pymethods]
    impl Graph {
        #[args(py_kwargs = "**")]
    #[text_signature = "($self, dtype)"]
    /// Return node embedding vector obtained from shortest-paths.
    pub fn get_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        dtype: Option<&str>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyAny>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let dtype = match dtype {
            Some(dtype) => dtype,
            None => {
                let (max_u8, max_u16, max_u32) = (u8::MAX as usize, u16::MAX as usize, u32::MAX as usize);
                match pe!(self.inner.get_diameter(Some(true), verbose))? as usize {
                    x if (0..=max_u8).contains(&x) => "u8",
                    x if (max_u8..=max_u16).contains(&x) => "u16",
                    x if (max_u16..=max_u32).contains(&x) => "u32",
                    _ => "u64",
                }
            }
        };
        let nodes_number = self.inner.get_nodes_number() as usize;
        match dtype {
            $(
                stringify!($dtype) => {
                    let (number_of_node_features, node_embedding_iterator, anchor_node_names) =
                    pe!(self.inner.get_shortest_paths_node_embedding::<$dtype>(
                        node_centralities,
                        node_centralities_distribution,
                        adjust_by_central_node_distance,
                        number_of_nodes_to_sample_per_feature,
                        maximum_number_of_features,
                        validate_node_centralities,
                        maximal_depth,
                        central_node_name,
                        central_node_id,
                        random_state,
                        return_sampled_node_names,
                        verbose,
                    ))?;
                let node_embedding: ThreadDataRaceAware<PyArray2<$dtype>> = ThreadDataRaceAware {
                    t: PyArray2::new(
                        gil.python(),
                        [nodes_number, number_of_node_features as usize],
                        false,
                    ),
                };
                node_embedding_iterator
                    .enumerate()
                    .for_each(|(number_of_node_feature, iterator)| {
                        iterator
                            .enumerate()
                            .for_each(|(node_id, node_feature)| unsafe {
                                *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                                    node_feature;
                            });
                    });
                Ok((
                    node_embedding.t.to_owned().into_py(gil.python()),
                    anchor_node_names,
                ))
                }
            )*
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {} is not supported. The supported ",
                    "data types are `u8`, `u16`, `u32` and `u64`."
                ),
                dtype
            ))),
        }
    }
}};
}

impl_get_shortest_paths_node_embedding! {u8, u16, u32, u64}
