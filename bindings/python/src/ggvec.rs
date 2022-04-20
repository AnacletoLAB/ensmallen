use super::*;
use numpy::PyArray2;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, )"]
    /// Compute the GGVEC node embedding for the current graph.
    ///
    /// Parameters
    /// ------------------------
    /// embedding_size: Optional[int] = 100
    ///     The embedding size. By default, 100.
    /// epochs: Optional[int] = 500
    ///     Maximal number of epochs to train for. By default, 500.
    /// negative_ratio: Optional[float]
    ///     Negative sampling ratio. Setting this higher will do more negative sampling. This is slower, but can lead to higher quality embeddings.
    /// exponent: Optional[float] = 0.5
    ///     Weighing exponent in loss function. Having this lower reduces effect of large edge weights.
    /// tollerance: Optional[float] = None
    ///     timization early stopping criterion. Stops average loss < tollerance for tol_samples epochs. By default, sets as a function of learning_rate
    /// patience: Optional[int] = 75
    ///     Optimization early stopping criterion. This is the number of epochs to sample for loss stability. Once loss is stable over this number of epochs we stop early. By defauly 75.
    /// negative_decay: Optional[float] = 0.0
    ///     Decay on negative ratio. If >0 then negative ratio will decay by (1-negative_decay) *poch. You should usually leave this to 0.
    /// learning_rate: Optional[float] = 0.05
    ///     Optimization learning rate.
    /// max_loss: Optional[float] = 30.0
    ///     Loss value ceiling for numerical stability.
    /// random_state: Optional[int] = 42
    ///     ndom rate for reproducible embeddings.
    /// verbose: Optional[bool] = True
    ///     hether to show the loading bar. By default, true.
    ///
    /// References
    /// ------------------------
    /// Please refer to the original implementation that can be found
    /// in the [CSRGraph repository](https://github.com/VHRanger/CSRGraph/blob/master/csrgraph/ggvec.py)
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the number of epochs is not a strictly positive integer.
    /// ValueError
    ///     If the negative ratio is not between zero and one.
    /// ValueError
    ///     If the negative decay is not between zero and one.
    /// ValueError
    ///     If the learning rate is not between zero and one.
    /// ValueError
    ///     If the graph does not have weights.
    fn compute_ggvec_embedding(
        &self,
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        negative_ratio: Option<f32>,
        exponent: Option<f32>,
        tollerance: Option<f64>,
        patience: Option<usize>,
        negative_decay: Option<f32>,
        learning_rate: Option<f32>,
        max_loss: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let embedding_size = embedding_size.unwrap_or(100);

        let rows_number = self.inner.get_nodes_number() as usize;
        let columns_number = embedding_size;
        let embedding = PyArray2::zeros(gil.python(), [rows_number, columns_number], false);

        let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

        pe!(self.inner.compute_ggvec_embedding(
            embedding_slice,
            Some(embedding_size),
            epochs,
            negative_ratio,
            exponent,
            tollerance,
            patience,
            negative_decay,
            learning_rate,
            max_loss,
            random_state,
            verbose,
        ))?;

        Ok(embedding.into_py(gil.python()))
    }
}
