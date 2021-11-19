use super::*;
use rayon::prelude::*;

macro_rules! impl_get_spine {
    ($($dtype:ty),*) => {
        #[pymethods]
        impl Graph {
            #[args(py_kwargs = "**")]
            #[text_signature = "($self, embedding_size, dtype, verbose)"]
            /// Return node embedding vector obtained from shortest-paths.
            ///
            /// Parameters
            /// ----------------------------
            /// embedding_size: Optional[int] - The number of features to generate. By default 100, or the number of nodes in the graph if it is lower.
            /// dtype: Optional[str] - Dtype to use for the embedding. Note that an improper dtype may cause overflows.
            /// verbose: Optional[bool] - Whether to show the loading bar. By default true.
            pub fn get_spine(
                &self,
                embedding_size: Option<usize>,
                dtype: Option<&str>,
                verbose: Option<bool>,
            ) -> PyResult<Py<PyAny>> {
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
                            let (number_of_node_features, node_embedding_iterator) =
                            pe!(self.inner.get_spine::<$dtype>(
                                embedding_size,
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
                        Ok(
                            node_embedding.t.to_owned().into_py(gil.python()),
                        )
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
        }
    };
}

impl_get_spine! {u8, u16, u32}
